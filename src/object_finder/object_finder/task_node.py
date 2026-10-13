"""找物任务主节点：同步 RGB-D、调用视觉模型、执行 TF 与安全检查并控制 MG400.

阅读顺序：main -> __init__ -> _prepare_startup -> _command_callback
-> _submit_task -> _run_task -> _execute_or_simulate_motion -> _send_action。
相机回调持续更新最新帧；每次任务使用提交时选定的同一组 RGB-D 消息。
"""

from dataclasses import dataclass
import math
import select
import signal
import sys
import threading
import time
from typing import Any, Optional

from action_msgs.msg import GoalStatus
import cv2
from cv_bridge import CvBridge
from geometry_msgs.msg import PointStamped, PoseStamped
from message_filters import ApproximateTimeSynchronizer, Subscriber
from mg400_msgs.action import JointMovJ, MovJ, MovL
from mg400_msgs.msg import RobotMode
import rclpy
from rclpy.action import ActionClient
from rclpy.duration import Duration
from rclpy.executors import ExternalShutdownException, MultiThreadedExecutor
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from rclpy.time import Time
from sensor_msgs.msg import CameraInfo, Image, JointState
from std_msgs.msg import Bool, String
from tf2_geometry_msgs import do_transform_point
from tf2_ros import Buffer, TransformException, TransformListener

from .depth import (
    depth_to_meters,
    localize_depth_region,
    validate_depth_parameters,
)
from .models import (
    CameraIntrinsics,
    MotionError,
    ObjectFinderError,
    SafetyError,
    VisionError,
)
from .readiness import ready_qos
from .safety import (
    WorkspaceBounds,
    validate_height_parameters,
    validate_mg400_joint_angles,
    validate_mg400_radial_reach,
    validate_percentage,
)
from .vision import (
    QwenProviderConfig,
    create_vision_grounder,
    select_single_detection,
)


@dataclass(frozen=True)
class FrameBundle:
    """保存一组近似同步的彩色图、对齐深度图和相机内参."""

    # 三条消息必须作为同一次观测冻结，任务执行期间不能被新帧替换。
    color: Image
    depth: Image
    camera_info: CameraInfo
    received_monotonic: float  # 本机接收时刻（秒），不是图像采集时间戳。


@dataclass(frozen=True)
class JointStateSnapshot:
    """保存最新关节状态及本机接收时刻，用于检查反馈是否过期."""

    message: JointState
    received_monotonic: float


class ObjectFinderNode(Node):
    """协调输入门控、视觉定位、坐标转换和分段运动的 ROS 主节点."""

    def __init__(self, *, parameter_overrides=None) -> None:
        """加载配置并创建 ROS 接口，随后启动等待使能和归位的线程.

        parameter_overrides 用于在测试或调用方覆盖参数；正常 launch 加载 YAML。
        初始化不会直接接收任务，必须由 _prepare_startup 发布 ready=True。
        """
        super().__init__(
            'object_finder', parameter_overrides=parameter_overrides,
        )
        self._declare_parameters()
        # 先检查全部配置，再创建订阅和 Action 客户端，错误配置直接拒绝启动。
        self._validate_configuration()

        self._bridge = CvBridge()
        # 相机回调、关节回调、终端线程和任务线程会并发访问这些状态。
        self._frame_lock = threading.Lock()
        self._joint_state_lock = threading.Lock()
        self._task_lock = threading.Lock()
        self._latest_frame: Optional[FrameBundle] = None
        self._latest_joint_state: Optional[JointStateSnapshot] = None
        self._task_active = False  # 标记已有任务占用执行权，防止运动交叉。
        self._input_ready = False  # 是否允许接收新任务，通过 ready 话题通知输入端。
        self._latest_robot_mode: Optional[int] = None
        self._robot_mode_received = 0.0
        self._motion_started_for_task = False
        self._shutdown_event = threading.Event()
        self._terminal_thread: Optional[threading.Thread] = None
        self._task_thread: Optional[threading.Thread] = None

        self._base_frame = self._string_parameter('frames.base')
        # 保存历史 TF，确保使用“拍照时刻”而不是“开始运动后”的相机位姿。
        self._tf_buffer = Buffer(cache_time=Duration(seconds=15.0))
        self._tf_listener = TransformListener(self._tf_buffer, self)

        # command/status 是未来语音识别和语音播报模块的稳定接口。
        self._status_publisher = self.create_publisher(
            String,
            self._string_parameter('topics.status'),
            ready_qos(),
        )
        self._ready_publisher = self.create_publisher(
            Bool, self._string_parameter('topics.ready'), ready_qos(),
        )
        self._target_publisher = self.create_publisher(
            PointStamped,
            self._string_parameter('topics.target_point'),
            10,
        )
        # 将最终选中的目标框发布给 RViz，方便人工核对模型结果。
        self._debug_image_publisher = self.create_publisher(
            Image,
            self._string_parameter('topics.debug_image'),
            10,
        )
        self._command_subscription = self.create_subscription(
            String,
            self._string_parameter('topics.command'),
            self._command_callback,
            10,
        )
        self._joint_state_subscription = self.create_subscription(
            JointState,
            self._string_parameter('topics.joint_states'),
            self._joint_state_callback,
            qos_profile_sensor_data,
        )
        self._robot_mode_subscription = self.create_subscription(
            RobotMode,
            self._string_parameter('topics.robot_mode'),
            self._robot_mode_callback,
            qos_profile_sensor_data,
        )

        self._configure_rgbd_sync()
        self._vision_grounder = self._create_vision_grounder()

        # 三个 Action 分段执行，任意一段失败都不会继续发送下一段。
        self._mov_j_client = ActionClient(
            self,
            MovJ,
            self._string_parameter('actions.mov_j'),
        )
        self._mov_l_client = ActionClient(
            self,
            MovL,
            self._string_parameter('actions.mov_l'),
        )
        self._joint_mov_j_client = ActionClient(
            self,
            JointMovJ,
            self._string_parameter('actions.joint_mov_j'),
        )

        if self._bool_parameter('motion.enabled'):
            startup_message = (
                '节点已启动，真机运动已启用；'
                '请确认人员已离开工作区并保持急停可用。'
            )
        else:
            startup_message = (
                '节点已启动。真机运动未启用，请等待同步 RGB-D 图像。'
            )
        self._publish_status('IDLE', startup_message)
        self._set_input_ready(False)
        # 归位线程等待使能和状态；executor 必须继续处理 TF、关节和 Action 回调。
        self._startup_thread = threading.Thread(
            target=self._prepare_startup,
            name='object_finder_startup',
            daemon=True,
        )
        self._startup_thread.start()
        if self._bool_parameter('terminal.enabled'):
            if sys.stdin.isatty():
                self._terminal_thread = threading.Thread(
                    target=self._terminal_loop,
                    name='object_finder_terminal',
                )
                self._terminal_thread.start()
            else:
                self.get_logger().warning(
                    '当前进程没有可交互标准输入，已禁用内置终端输入。'
                    '请从另一终端向 command 话题发布指令，'
                    '或使用 ros2 run 直接启动节点。'
                )

    def _declare_parameters(self) -> None:
        """声明全部 ROS 参数的默认值，供 YAML 或 parameter_overrides 覆盖."""
        # 这里给出无 YAML 时的安全默认值；正常启动由 object_finder.yaml 覆盖。
        parameters = {
            # 输入/输出话题必须与 D435、MG400 驱动及输入桥的命名保持一致。
            'topics.color': '/camera/camera/color/image_raw',
            'topics.depth': (
                '/camera/camera/aligned_depth_to_color/image_raw'
            ),
            'topics.camera_info': '/camera/camera/color/camera_info',
            'topics.command': '/object_finder/command',
            'topics.status': '/object_finder/status',
            'topics.ready': '/object_finder/ready',
            'topics.robot_mode': '/mg400/robot_mode',
            'topics.target_point': '/object_finder/target_point',
            'topics.debug_image': '/object_finder/debug_image',
            'topics.joint_states': '/mg400/joint_states',
            # base 是所有运动目标的最终坐标系，flange 用于校验末端状态。
            'frames.base': 'mg400_origin_link',
            'frames.flange': 'mg400_end_effector_flange',
            # RGB、对齐深度和内参时间戳允许的近似同步条件。
            'sync.queue_size': 10,
            'sync.slop_sec': 0.08,
            'sync.maximum_frame_age_sec': 1.0,
            'sync.require_matching_frames': True,
            'tf.timeout_sec': 1.0,
            # 深度单位、有效范围和目标框中央采样规则，长度单位均为米。
            'depth.scale_16u': 0.001,
            'depth.minimum_m': 0.10,
            'depth.maximum_m': 2.0,
            'depth.minimum_valid_ratio': 0.20,
            'depth.minimum_valid_samples': 25,
            'depth.roi_shrink_ratio': 0.20,
            # disabled/fixed_bbox/qwen 分别表示禁用、几何联调和真实模型。
            'vision.provider': 'disabled',
            'vision.minimum_confidence': 0.60,
            'vision.selection_policy': 'require_single',
            'vision.fixed_bbox': [0.35, 0.35, 0.65, 0.65],
            'vision.qwen.model': '',
            'vision.qwen.base_url': '',
            'vision.qwen.api_key_env': 'DASHSCOPE_API_KEY',
            'vision.qwen.timeout_sec': 30.0,
            'vision.qwen.max_retries': 2,
            # 最高级运动联锁默认关闭；两个高度都是相对物体表面的距离。
            'motion.enabled': False,
            'motion.final_standoff_m': 0.05,
            'motion.approach_height_m': 0.15,
            'motion.yaw_policy': 'radial',
            'motion.fixed_yaw_rad': 0.0,
            'motion.speed_j': 10,
            'motion.acc_j': 10,
            'motion.speed_l': 10,
            'motion.acc_l': 10,
            'motion.cp': 0,
            'motion.action_server_timeout_sec': 3.0,
            'motion.action_result_timeout_sec': 30.0,
            # MG400_ROS2 驱动提供的三个 Action 名称。
            'actions.mov_j': '/mg400/mov_j',
            'actions.mov_l': '/mg400/mov_l',
            'actions.joint_mov_j': '/mg400/joint_mov_j',
            # Home 是固定观察位和任务完成返回位，未实测前 configured 必须为 false。
            'home.configured': False,
            'home.mode': 'joint',
            'home.joint_angles_rad': [0.0, 0.0, 0.0, 0.0],
            'home.joint_names': [
                'mg400_j1',
                'mg400_j2_1',
                'mg400_j4_2',
                'mg400_j5',
            ],
            'home.pose': [0.0, 0.0, 0.0, 0.0],
            'home.auto_on_startup': True,
            'home.startup_timeout_sec': 60.0,
            'home.verify_timeout_sec': 5.0,
            'home.settle_sec': 0.3,
            'home.joint_tolerance_rad': 0.035,
            'home.position_tolerance_m': 0.01,
            'home.yaw_tolerance_rad': 0.035,
            'home.state_max_age_sec': 0.5,
            'home.capture_state_tolerance_sec': 0.25,
            # 自定义工作区及现场间隙确认是开启真机运动的必要联锁。
            'safety.workspace_configured': False,
            'safety.workspace_bounds': [
                -0.40,
                0.40,
                -0.40,
                0.40,
                -0.10,
                0.30,
            ],
            'safety.clearance_verified': False,
            'terminal.enabled': True,
        }
        for name, default_value in parameters.items():
            self.declare_parameter(name, default_value)

    def _validate_configuration(self) -> None:
        """启动前检查参数范围、Home 定义及真机运动联锁.

        非法配置抛出 SafetyError 或 VisionError，使节点拒绝初始化。
        这里只校验配置，不代表外参或整条运动路径已经完成现场验证。
        """
        # 配置采用失败关闭策略：不完整或非法参数不能带病运行。
        required_strings = (
            'topics.color',
            'topics.depth',
            'topics.camera_info',
            'topics.command',
            'topics.status',
            'topics.ready',
            'topics.robot_mode',
            'topics.target_point',
            'topics.debug_image',
            'topics.joint_states',
            'frames.base',
            'frames.flange',
            'actions.mov_j',
            'actions.mov_l',
            'actions.joint_mov_j',
        )
        for name in required_strings:
            if not self._string_parameter(name).strip():
                raise SafetyError(f'{name} 不能为空。')

        if self._int_parameter('sync.queue_size') < 1:
            raise SafetyError('sync.queue_size 必须大于等于 1。')
        nonnegative_parameters = ('sync.slop_sec', 'home.settle_sec')
        for name in nonnegative_parameters:
            value = self._float_parameter(name)
            if not math.isfinite(value) or value < 0.0:
                raise SafetyError(f'{name} 必须是非负有限数值。')
        positive_parameters = (
            'sync.maximum_frame_age_sec',
            'tf.timeout_sec',
            'depth.scale_16u',
            'motion.action_server_timeout_sec',
            'motion.action_result_timeout_sec',
            'home.joint_tolerance_rad',
            'home.position_tolerance_m',
            'home.yaw_tolerance_rad',
            'home.state_max_age_sec',
            'home.capture_state_tolerance_sec',
            'home.startup_timeout_sec',
            'home.verify_timeout_sec',
        )
        for name in positive_parameters:
            value = self._float_parameter(name)
            if not math.isfinite(value) or value <= 0.0:
                raise SafetyError(f'{name} 必须是大于 0 的有限数值。')

        validate_depth_parameters(
            roi_shrink_ratio=self._float_parameter(
                'depth.roi_shrink_ratio'
            ),
            minimum_depth_m=self._float_parameter('depth.minimum_m'),
            maximum_depth_m=self._float_parameter('depth.maximum_m'),
            minimum_valid_ratio=self._float_parameter(
                'depth.minimum_valid_ratio'
            ),
            minimum_valid_samples=self._int_parameter(
                'depth.minimum_valid_samples'
            ),
        )
        validate_height_parameters(
            self._float_parameter('motion.final_standoff_m'),
            self._float_parameter('motion.approach_height_m'),
        )

        minimum_confidence = self._float_parameter(
            'vision.minimum_confidence'
        )
        if not math.isfinite(minimum_confidence) or not (
            0.0 <= minimum_confidence <= 1.0
        ):
            raise VisionError(
                'vision.minimum_confidence 必须位于 [0, 1]。'
            )
        if self._string_parameter(
            'vision.selection_policy'
        ) != 'require_single':
            raise VisionError('当前只支持 require_single 目标选择策略。')

        yaw_policy = self._string_parameter('motion.yaw_policy')
        if yaw_policy not in {'radial', 'fixed'}:
            raise SafetyError(f'不支持的 yaw 策略：{yaw_policy}')
        fixed_yaw = self._float_parameter('motion.fixed_yaw_rad')
        if not math.isfinite(fixed_yaw):
            raise SafetyError('motion.fixed_yaw_rad 必须是有限数值。')

        for name in (
            'motion.speed_j',
            'motion.acc_j',
            'motion.speed_l',
            'motion.acc_l',
        ):
            validate_percentage(self._int_parameter(name), name)
        validate_percentage(
            self._int_parameter('motion.cp'),
            'motion.cp',
            allow_zero=True,
        )

        workspace = WorkspaceBounds.from_sequence(
            self.get_parameter('safety.workspace_bounds').value
        )
        home_mode = self._string_parameter('home.mode')
        if home_mode not in {'joint', 'pose'}:
            raise SafetyError(f'不支持的 home.mode：{home_mode}')
        joint_names = list(self.get_parameter('home.joint_names').value)
        if (
            len(joint_names) != 4
            or any(not str(name).strip() for name in joint_names)
            or len(set(joint_names)) != 4
        ):
            raise SafetyError(
                'home.joint_names 必须包含 4 个不同的非空关节名。'
            )
        if self._bool_parameter('home.configured'):
            self._validate_home_configuration(workspace)

        if self._bool_parameter('motion.enabled'):
            missing = []
            if not self._bool_parameter('home.configured'):
                missing.append('home.configured')
            if not self._bool_parameter('safety.workspace_configured'):
                missing.append('safety.workspace_configured')
            if not self._bool_parameter('safety.clearance_verified'):
                missing.append('safety.clearance_verified')
            if missing:
                raise SafetyError(
                    'motion.enabled=true，但安全条件未满足：'
                    + ', '.join(missing)
                )

    def _configure_rgbd_sync(self) -> None:
        """订阅彩色图、对齐深度和内参，按时间戳配对后触发 _rgbd_callback."""
        # 彩色图、对齐深度图和内参必须来自近似相同的采集时刻。
        self._color_subscriber = Subscriber(
            self,
            Image,
            self._string_parameter('topics.color'),
            qos_profile=qos_profile_sensor_data,
        )
        self._depth_subscriber = Subscriber(
            self,
            Image,
            self._string_parameter('topics.depth'),
            qos_profile=qos_profile_sensor_data,
        )
        self._camera_info_subscriber = Subscriber(
            self,
            CameraInfo,
            self._string_parameter('topics.camera_info'),
            qos_profile=qos_profile_sensor_data,
        )
        self._synchronizer = ApproximateTimeSynchronizer(
            [
                self._color_subscriber,
                self._depth_subscriber,
                self._camera_info_subscriber,
            ],
            queue_size=self._int_parameter('sync.queue_size'),
            slop=self._float_parameter('sync.slop_sec'),
        )
        self._synchronizer.registerCallback(self._rgbd_callback)

    def _create_vision_grounder(self):
        """根据 vision.provider 创建定位后端，返回具有 locate 接口的对象.

        disabled 拒绝定位，fixed_bbox 返回测试框，qwen 调用真实视觉模型。
        后端的创建与每次任务的 locate 调用是两个不同阶段。
        """
        # provider 可替换，任务节点不依赖具体千问型号或部署方式。
        qwen_config = QwenProviderConfig(
            model=self._string_parameter('vision.qwen.model'),
            base_url=self._string_parameter('vision.qwen.base_url'),
            api_key_environment_variable=self._string_parameter(
                'vision.qwen.api_key_env'
            ),
            timeout_sec=self._float_parameter('vision.qwen.timeout_sec'),
            max_retries=self._int_parameter('vision.qwen.max_retries'),
        )
        return create_vision_grounder(
            provider=self._string_parameter('vision.provider'),
            fixed_bbox=list(
                self.get_parameter('vision.fixed_bbox').value
            ),
            qwen_config=qwen_config,
        )

    def _rgbd_callback(
        self,
        color: Image,
        depth: Image,
        camera_info: CameraInfo,
    ) -> None:
        """接收同步器配对的三条消息，在锁内更新最新观测供任务选取."""
        # 回调只保存最新完整观测，耗时的视觉推理放到独立任务线程。
        with self._frame_lock:
            self._latest_frame = FrameBundle(
                color=color,
                depth=depth,
                camera_info=camera_info,
                received_monotonic=time.monotonic(),
            )

    def _joint_state_callback(self, message: JointState) -> None:
        """缓存最新关节位置和接收时间，供关节 Home 模式核验."""
        with self._joint_state_lock:
            self._latest_joint_state = JointStateSnapshot(
                message=message,
                received_monotonic=time.monotonic(),
            )

    def _robot_mode_callback(self, message: RobotMode) -> None:
        """缓存机器人当前模式和接收时间，供每段运动前后检查状态."""
        with self._joint_state_lock:
            self._latest_robot_mode = int(message.robot_mode)
            self._robot_mode_received = time.monotonic()

    def _set_input_ready(self, ready: bool) -> None:
        """更新本节点输入门控，并发布 Bool 通知终端客户端和输入桥."""
        # 与输入桥共用的状态：归位和任务期间均不缓存新的找物指令。
        self._input_ready = ready
        self._ready_publisher.publish(Bool(data=ready))

    def _require_robot_enabled(self) -> None:
        """要求机器人反馈新鲜且处于空闲 ENABLE，否则抛出 SafetyError.

        RUNNING 虽然也是使能状态，但不允许作为新一段运动的起始状态。
        """
        with self._joint_state_lock:
            mode = self._latest_robot_mode
            age = time.monotonic() - self._robot_mode_received
        if age > self._float_parameter('home.state_max_age_sec'):
            raise SafetyError('MG400 实时模式已过期或尚未收到。')
        if mode != RobotMode.ENABLE:
            raise SafetyError(f'MG400 未处于空闲使能状态，robot_mode={mode}。')

    def _home_reached(self) -> bool:
        """用当前反馈判断是否到 Home，返回 bool；缺失或过期反馈则报错.

        joint 模式比较四个物理关节角；pose 模式比较最新法兰 TF 的位置和 yaw。
        此处检查当前位姿，拍照时刻的位姿由 _validate_observation_pose 检查。
        """
        if self._string_parameter('home.mode') == 'joint':
            with self._joint_state_lock:
                snapshot = self._latest_joint_state
            if snapshot is None or (
                time.monotonic() - snapshot.received_monotonic
                > self._float_parameter('home.state_max_age_sec')
            ):
                raise SafetyError('尚未收到新鲜的 Home 关节状态。')
            message = snapshot.message
            if len(message.name) != len(message.position):
                raise SafetyError('JointState 关节名称与位置数量不一致。')
            positions = dict(zip(message.name, message.position))
            names = list(self.get_parameter('home.joint_names').value)
            expected = list(self.get_parameter('home.joint_angles_rad').value)
            if any(name not in positions for name in names):
                raise SafetyError('JointState 缺少 Home 关节。')
            return all(
                math.isfinite(positions[name])
                and self._angular_distance(positions[name], target)
                <= self._float_parameter('home.joint_tolerance_rad')
                for name, target in zip(names, expected)
            )

        try:
            transform = self._tf_buffer.lookup_transform(
                self._base_frame,
                self._string_parameter('frames.flange'),
                Time(),
            )
        except TransformException as error:
            raise SafetyError(f'尚未获得当前法兰 TF：{error}') from error
        stamp = Time.from_msg(transform.header.stamp).nanoseconds
        age = (self.get_clock().now().nanoseconds - stamp) / 1e9
        if abs(age) > self._float_parameter('home.state_max_age_sec'):
            raise SafetyError(f'当前法兰 TF 已过期（{age:.2f} s）。')
        expected = list(self.get_parameter('home.pose').value)
        translation = transform.transform.translation
        position_error = math.sqrt(sum(
            (actual - target) ** 2
            for actual, target in zip(
                (translation.x, translation.y, translation.z), expected[:3],
            )
        ))
        rotation = transform.transform.rotation
        yaw = math.atan2(
            2.0 * (rotation.w * rotation.z + rotation.x * rotation.y),
            1.0 - 2.0 * (rotation.y ** 2 + rotation.z ** 2),
        )
        return (
            math.isfinite(position_error)
            and position_error <= self._float_parameter(
                'home.position_tolerance_m'
            )
            and self._angular_distance(yaw, expected[3])
            <= self._float_parameter('home.yaw_tolerance_rad')
        )

    def _wait_for_home(self, timeout_sec: float) -> None:
        """轮询空闲使能状态与 Home 到位结果；超时或关闭时抛出 MotionError."""
        deadline = time.monotonic() + timeout_sec
        last_error = '当前位置与 Home 不一致'
        while not self._shutdown_event.is_set() and rclpy.ok():
            try:
                self._require_robot_enabled()
                if self._home_reached():
                    return
            except SafetyError as error:
                last_error = str(error)
            if time.monotonic() >= deadline:
                raise MotionError(f'Home 到位确认超时：{last_error}')
            self._shutdown_event.wait(0.1)
        raise MotionError('系统正在关闭，Home 确认已中止。')

    def _wait_for_robot_idle(self) -> None:
        """等待 Action 结果之后的新 ENABLE 反馈，并要求空闲稳定至少 0.1 秒.

        ERROR、DISABLED 或确认超时会抛出 MotionError，阻止后续运动。
        """
        # Action 结果和 robot_mode 分属不同话题，允许末尾 RUNNING -> ENABLE 延迟。
        # 不能复用运动开始前的 ENABLE；必须有结果之后收到的新状态并稳定一小段时间。
        minimum_received = time.monotonic()
        deadline = time.monotonic() + self._float_parameter(
            'home.verify_timeout_sec'
        )
        idle_since = None
        last_error = '尚未收到运动结束后的空闲使能状态'
        while not self._shutdown_event.is_set() and rclpy.ok():
            try:
                self._require_robot_enabled()
                with self._joint_state_lock:
                    received = self._robot_mode_received
                if received > minimum_received:
                    now = time.monotonic()
                    if idle_since is None:
                        idle_since = now
                    elif now - idle_since >= 0.1:
                        return
            except SafetyError as error:
                idle_since = None
                last_error = str(error)
                with self._joint_state_lock:
                    mode = self._latest_robot_mode
                if mode in {RobotMode.ERROR, RobotMode.DISABLED}:
                    raise MotionError(f'运动结束后机械臂不可继续执行：{error}') from error
            if time.monotonic() >= deadline:
                raise MotionError(f'运动后等待空闲使能超时：{last_error}')
            self._shutdown_event.wait(0.05)
        raise MotionError('系统正在关闭，不再执行下一段运动。')

    def _refresh_home_observation(self) -> None:
        """到 Home 后等待稳定、清除旧帧，再等待并核验一组新 RGB-D.

        以 ROS 采集时间戳筛选新帧，并检查数据尺寸、坐标系和拍照位姿。
        超时或关闭时抛出异常，不开放下一次输入。
        """
        if self._shutdown_event.wait(self._float_parameter('home.settle_sec')):
            raise MotionError('系统正在关闭，已停止等待图像。')
        # 必须在归位稳定后重新取图，不能复用移动中拍到的 RGB-D。
        minimum_stamp = self.get_clock().now().nanoseconds
        with self._frame_lock:
            self._latest_frame = None
        deadline = time.monotonic() + self._float_parameter(
            'home.verify_timeout_sec'
        )
        last_error = '等待 Home 位置的新 RGB-D 图像'
        while not self._shutdown_event.is_set() and rclpy.ok():
            with self._frame_lock:
                frame = self._latest_frame
            if frame is not None and (
                Time.from_msg(frame.depth.header.stamp).nanoseconds
                >= minimum_stamp
            ):
                try:
                    self._require_robot_enabled()
                    self._validate_frame_bundle(frame)
                    self._validate_observation_pose(frame)
                    return
                except SafetyError as error:
                    last_error = str(error)
            if time.monotonic() >= deadline:
                raise SafetyError(f'归位后图像确认超时：{last_error}')
            self._shutdown_event.wait(0.1)
        raise MotionError('系统正在关闭，已停止等待图像。')

    def _prepare_startup(self) -> None:
        """在启动线程中等待使能，必要时归位并取新图，最后开放输入.

        真机模式失败时发布 STARTUP_FAILED 并保持输入关闭。
        dry-run 跳过使能、归位和新帧等待；图像可用性由 _submit_task 检查。
        """
        try:
            if self._bool_parameter('motion.enabled'):
                self._publish_status('WAITING_ENABLE', '等待 MG400 使能和法兰状态。')
                deadline = time.monotonic() + self._float_parameter(
                    'home.startup_timeout_sec'
                )
                at_home = False
                while not self._shutdown_event.is_set() and rclpy.ok():
                    try:
                        self._require_robot_enabled()
                        at_home = self._home_reached()
                        break
                    except SafetyError:
                        with self._joint_state_lock:
                            mode = self._latest_robot_mode
                        if mode == RobotMode.ERROR:
                            raise SafetyError('MG400 有报警，启动归位已停止。')
                    if time.monotonic() >= deadline:
                        raise SafetyError('等待 MG400 使能或新鲜法兰状态超时。')
                    self._shutdown_event.wait(0.1)
                if self._shutdown_event.is_set() or not rclpy.ok():
                    return
                if not at_home:
                    if not self._bool_parameter('home.auto_on_startup'):
                        raise SafetyError('未在 Home，且启动自动归位已关闭。')
                    self._validate_action_servers()
                    self._publish_status('HOMING', '未在 Home，正在低速归位。')
                    self._send_home()
                self._wait_for_home(
                    self._float_parameter('home.verify_timeout_sec')
                )
                self._publish_status('HOME_REACHED', '已到 Home，等待相机稳定。')
                self._refresh_home_observation()
            self._publish_status('READY', '已准备好，等待下一条文字或语音指令。')
            self._set_input_ready(True)
        except Exception as error:
            if self._shutdown_event.is_set() or not rclpy.ok():
                return
            self._set_input_ready(False)
            self._publish_status(
                'STARTUP_FAILED',
                f'启动准备失败：{error}。输入未开放，请检查后重新启动。',
            )

    def _command_callback(self, message: String) -> None:
        """接收统一 command 话题的字符串，将其交给 _submit_task 门控."""
        self._submit_task(message.data)

    def _terminal_loop(self) -> None:
        """直接运行主节点时读取终端输入，并交给同一个任务提交入口.

        launch 会关闭这一内置入口，改用独立的 text_input.py 客户端。
        此处 quit 仅退出输入线程，主节点仍由 ROS Executor 运行。
        """
        prompt = '\n请输入要寻找的物体（quit 退出输入）：'
        prompt_shown = False
        while rclpy.ok() and not self._shutdown_event.is_set():
            if not self._input_ready:
                prompt_shown = False
                self._shutdown_event.wait(0.1)
                continue
            if not prompt_shown:
                print(prompt, end='', flush=True)
                prompt_shown = True
            try:
                readable, _, _ = select.select(
                    [sys.stdin],
                    [],
                    [],
                    0.2,
                )
            except (OSError, ValueError):
                return
            if not readable:
                continue
            line = sys.stdin.readline()
            if line == '':
                return
            query = line.strip()
            if not query:
                print(prompt, end='', flush=True)
                continue
            if query.lower() in {'quit', 'exit'}:
                self._publish_status('IDLE', '终端输入线程已退出。')
                return
            self._submit_task(query)
            prompt_shown = False

    def stop_terminal_input(self) -> None:
        """设置关闭事件，并为每个工作线程最多等待一秒结束.

        由 main 的清理路径调用；等待时间有限，不能保证 HTTP 请求已结束，
        也不代表已经停止机械臂运动。
        """
        self._shutdown_event.set()
        for worker in (
            self._terminal_thread, self._startup_thread, self._task_thread,
        ):
            if worker is None:
                continue
            deadline = time.monotonic() + 1.0
            while worker.is_alive():
                remaining = deadline - time.monotonic()
                if remaining <= 0.0:
                    break
                try:
                    worker.join(timeout=min(0.2, remaining))
                except KeyboardInterrupt:
                    # ros2 launch 可能在清理期间再转发一次 SIGINT。
                    continue

    def _submit_task(self, query: str) -> None:
        """检查忙碌状态、输入门控和帧龄，选定观测后启动一次任务线程.

        query 是目标文字。拒绝时通过状态话题说明原因；接受后先关闭输入。
        这里只选取 FrameBundle 的引用，后续相机回调不会替换任务中的引用。
        """
        normalized_query = query.strip()
        if not normalized_query:
            self._publish_status('REJECTED', '目标描述不能为空。')
            return

        with self._task_lock:
            # 同一时间只允许一个任务，避免两组运动命令交叉执行。
            if self._task_active:
                self._publish_status(
                    'BUSY',
                    '已有任务正在执行，本次命令未接收。',
                )
                return
            if not self._input_ready:
                self._publish_status('NOT_READY', '尚未在 Home 准备好，指令未接收。')
                return
            # 在这里复制引用并冻结本次任务使用的观测帧。
            with self._frame_lock:
                frame = self._latest_frame
            if frame is None:
                self._publish_status(
                    'WAITING_FRAME',
                    '尚未收到同步 RGB-D 图像，机械臂保持不动。',
                )
                self._set_input_ready(True)
                return
            maximum_age = self._float_parameter(
                'sync.maximum_frame_age_sec'
            )
            age = time.monotonic() - frame.received_monotonic
            if age > maximum_age:
                self._publish_status(
                    'STALE_FRAME',
                    f'最近图像已过期（{age:.2f} s），机械臂保持不动。',
                )
                self._set_input_ready(True)
                return
            self._task_active = True
            self._set_input_ready(False)

        self._task_thread = threading.Thread(
            target=self._run_task,
            args=(normalized_query, frame),
            name='object_finder_task',
            daemon=True,
        )
        self._task_thread.start()

    def _run_task(self, query: str, frame: FrameBundle) -> None:
        """执行一次找物任务：二维定位 -> 深度 -> 基座点 -> 安全检查与运动.

        query 和 frame 均来自提交时选定的数据；耗时的模型请求在此线程执行。
        最后的 finally 释放任务占用权，并根据成功与是否开始运动决定开放输入。
        失败时不会在这里自动补发返回 Home 的运动。
        """
        self._motion_started_for_task = False
        succeeded = False
        try:
            # 数据流：文字+彩色图定位二维目标，再由深度和 TF 算机器人坐标。
            self._publish_status('LOCALIZING', f'正在寻找：{query}')
            color_bgr = self._bridge.imgmsg_to_cv2(
                frame.color,
                desired_encoding='bgr8',
            )
            depth_raw = self._bridge.imgmsg_to_cv2(
                frame.depth,
                desired_encoding='passthrough',
            )
            self._validate_frame_bundle(frame)
            # 真机模式必须确认图像是在固定观察/Home 位置拍摄的。
            if self._bool_parameter('motion.enabled'):
                self._validate_observation_pose(frame)
            # locate 返回候选列表；select_single_detection 再筛选为唯一目标。
            detection = select_single_detection(
                self._vision_grounder.locate(color_bgr, query),
                minimum_confidence=self._float_parameter(
                    'vision.minimum_confidence'
                ),
                policy=self._string_parameter(
                    'vision.selection_policy'
                ),
            )
            self._publish_debug_detection(
                color_bgr,
                frame.color,
                detection,
            )

            # 模型只提供二维框；深度和相机内参负责生成真实米制三维点。
            depth_m = depth_to_meters(
                depth_raw,
                frame.depth.encoding,
                self._float_parameter('depth.scale_16u'),
            )
            localized = localize_depth_region(
                depth_m=depth_m,
                intrinsics=CameraIntrinsics.from_camera_matrix(
                    frame.camera_info.k
                ),
                bounding_box=detection.bounding_box,
                mask=detection.mask,
                roi_shrink_ratio=self._float_parameter(
                    'depth.roi_shrink_ratio'
                ),
                minimum_depth_m=self._float_parameter(
                    'depth.minimum_m'
                ),
                maximum_depth_m=self._float_parameter(
                    'depth.maximum_m'
                ),
                minimum_valid_ratio=self._float_parameter(
                    'depth.minimum_valid_ratio'
                ),
                minimum_valid_samples=self._int_parameter(
                    'depth.minimum_valid_samples'
                ),
            )

            # 此点仍属于相机光学坐标系，不能直接作为 MG400 目标。
            optical_point = PointStamped()
            optical_point.header = frame.depth.header  # 保留光学帧名和拍照时刻。
            optical_point.point.x = localized.x
            optical_point.point.y = localized.y
            optical_point.point.z = localized.z
            base_point = self._transform_to_base(optical_point)
            # 发布的是物体表面点；法兰最终悬停点还需要在其 Z 上增加间隙。
            self._target_publisher.publish(base_point)

            self._publish_status(
                'TARGET_READY',
                '目标坐标：'
                f'({base_point.point.x:.4f}, '
                f'{base_point.point.y:.4f}, '
                f'{base_point.point.z:.4f}) m；'
                f'有效深度比例 {localized.valid_ratio:.1%}。',
            )
            self._execute_or_simulate_motion(query, base_point)
            succeeded = True
        except ObjectFinderError as error:
            failure_action = (
                '不再发送后续运动命令，请确认机械臂当前状态。'
                if self._motion_started_for_task
                else '未发送运动命令，机械臂保持当前位置。'
            )
            self._publish_status(
                'FAILED',
                f'{error} {failure_action}',
            )
        except Exception as error:  # 捕获未预期异常，阻止继续下发后续动作。
            self.get_logger().error(f'任务发生未预期错误：{error!r}')
            failure_action = (
                '不再发送后续运动命令，请确认机械臂当前状态。'
                if self._motion_started_for_task
                else '未发送运动命令，机械臂保持当前位置。'
            )
            self._publish_status(
                'FAILED',
                f'未预期错误：{error}。{failure_action}',
            )
        finally:
            if self._shutdown_event.is_set() or not rclpy.ok():
                return
            # 运动失败时锁住输入，不自动继续归位；感知失败且仍在 Home 可重试。
            ready = succeeded
            if not self._motion_started_for_task and not succeeded:
                try:
                    if self._bool_parameter('motion.enabled'):
                        self._require_robot_enabled()
                        ready = self._home_reached()
                    else:
                        ready = True
                except SafetyError:
                    ready = False
            self._motion_started_for_task = False
            with self._task_lock:
                self._task_active = False
                self._set_input_ready(ready)
            if ready:
                self._publish_status('READY', '等待下一条文字或语音指令。')

    def _publish_debug_detection(
        self,
        color_bgr,
        source_message: Image,
        detection,
    ) -> None:
        """复制彩色图并绘制选中的目标框和置信度，发布到 RViz 调试话题.

        source_message 提供原图时间戳和帧名；绘图不修改用于定位的原数组。
        """
        debug_image = color_bgr.copy()
        height, width = debug_image.shape[:2]
        box = detection.bounding_box.to_pixels(width, height)
        top_left = (int(round(box.x_min)), int(round(box.y_min)))
        bottom_right = (int(round(box.x_max)), int(round(box.y_max)))
        cv2.rectangle(
            debug_image,
            top_left,
            bottom_right,
            (0, 255, 0),
            2,
        )
        # OpenCV 默认字体不能稳定渲染中文，图上只标数值置信度。
        cv2.putText(
            debug_image,
            f'confidence={detection.confidence:.2f}',
            (top_left[0], max(18, top_left[1] - 6)),
            cv2.FONT_HERSHEY_SIMPLEX,
            0.55,
            (0, 255, 0),
            2,
            cv2.LINE_AA,
        )
        output = self._bridge.cv2_to_imgmsg(
            debug_image,
            encoding='bgr8',
        )
        output.header = source_message.header
        self._debug_image_publisher.publish(output)

    def _validate_frame_bundle(self, frame: FrameBundle) -> None:
        """检查 RGB、对齐深度和内参的帧名及分辨率是否一致.

        不一致时抛出 SafetyError；关闭 require_matching_frames 后跳过检查。
        这里不重新检查采集时间或帧龄，近似同步由 message_filters 完成。
        """
        if not self._bool_parameter('sync.require_matching_frames'):
            return
        color_frame = frame.color.header.frame_id
        depth_frame = frame.depth.header.frame_id
        info_frame = frame.camera_info.header.frame_id
        if not color_frame or not depth_frame or not info_frame:
            raise SafetyError(
                '彩色图、对齐深度图或 CameraInfo 缺少 frame_id。'
            )
        # 对齐深度必须与彩色图使用相同光学坐标系和相同分辨率。
        if len({color_frame, depth_frame, info_frame}) != 1:
            raise SafetyError(
                'RGB-D 数据坐标系不一致：'
                f'color={color_frame}, depth={depth_frame}, '
                f'camera_info={info_frame}。'
            )

        color_size = (frame.color.width, frame.color.height)
        depth_size = (frame.depth.width, frame.depth.height)
        info_size = (
            frame.camera_info.width,
            frame.camera_info.height,
        )
        if color_size != depth_size or color_size != info_size:
            raise SafetyError(
                'RGB-D 数据分辨率不一致：'
                f'color={color_size}, depth={depth_size}, '
                f'camera_info={info_size}。'
            )

    def _transform_to_base(self, point: PointStamped) -> PointStamped:
        """按 point 的拍照时间查询 TF，将相机三维点转换到基座坐标系.

        输入和输出位置单位均为米；无法取得历史 TF 时抛出 SafetyError。
        """
        try:
            # 使用消息时间戳查询 TF，防止机械臂移动后坐标发生漂移。
            transform = self._tf_buffer.lookup_transform(
                self._base_frame,
                point.header.frame_id,
                Time.from_msg(point.header.stamp),
                timeout=Duration(
                    seconds=self._float_parameter('tf.timeout_sec')
                ),
            )
        except TransformException as error:
            raise SafetyError(
                '无法在拍照时间获得相机到机械臂基坐标的 TF：'
                f'{error}'
            ) from error
        return do_transform_point(point, transform)

    def _validate_observation_pose(self, frame: FrameBundle) -> None:
        """确认选定图像是在配置的 Home 观察位拍摄，不在位则抛出 SafetyError.

        pose 模式查询拍照时刻的法兰 TF；joint 模式转交关节快照校验。
        """
        mode = self._string_parameter('home.mode')
        if mode == 'joint':
            self._validate_observation_joint_state(frame)
            return

        # 笛卡尔 Home 模式直接比较拍照时刻的基座到法兰 TF。
        try:
            transform = self._tf_buffer.lookup_transform(
                self._base_frame,
                self._string_parameter('frames.flange'),
                Time.from_msg(frame.depth.header.stamp),
                timeout=Duration(
                    seconds=self._float_parameter('tf.timeout_sec')
                ),
            )
        except TransformException as error:
            raise SafetyError(
                '无法确认拍照时法兰是否位于观察位置：'
                f'{error}'
            ) from error

        expected = [
            float(value)
            for value in self.get_parameter('home.pose').value
        ]
        translation = transform.transform.translation
        distance = math.sqrt(
            (translation.x - expected[0]) ** 2
            + (translation.y - expected[1]) ** 2
            + (translation.z - expected[2]) ** 2
        )
        rotation = transform.transform.rotation
        actual_yaw = math.atan2(
            2.0 * (
                rotation.w * rotation.z
                + rotation.x * rotation.y
            ),
            1.0 - 2.0 * (
                rotation.y * rotation.y
                + rotation.z * rotation.z
            ),
        )
        yaw_error = self._angular_distance(actual_yaw, expected[3])
        if (
            distance > self._float_parameter(
                'home.position_tolerance_m'
            )
            or yaw_error > self._float_parameter(
                'home.yaw_tolerance_rad'
            )
        ):
            raise SafetyError(
                '拍照时法兰不在配置的观察位置：'
                f'位置误差={distance:.4f} m，'
                f'yaw 误差={yaw_error:.4f} rad。'
            )

    def _validate_observation_joint_state(
        self,
        frame: FrameBundle,
    ) -> None:
        """从最新 JointState 提取四个物理关节，与 Home 角度比较.

        同时检查接收新鲜度，以及关节时间戳与拍照时刻的差值（秒）。
        当前只保存最新快照，并没有按拍照时间保存关节历史。
        """
        with self._joint_state_lock:
            snapshot = self._latest_joint_state
        if snapshot is None:
            raise SafetyError('尚未收到 MG400 JointState，无法确认观察位置。')

        age = time.monotonic() - snapshot.received_monotonic
        maximum_age = self._float_parameter('home.state_max_age_sec')
        if age > maximum_age:
            raise SafetyError(
                f'MG400 JointState 已过期（{age:.2f} s）。'
            )

        message = snapshot.message
        if len(message.name) != len(message.position):
            raise SafetyError('MG400 JointState 的名称与位置数量不一致。')
        state_by_name = dict(zip(message.name, message.position))
        # 驱动发布 8 个 URDF 关节，配置只提取四个物理控制关节。
        joint_names = list(self.get_parameter('home.joint_names').value)
        missing = [name for name in joint_names if name not in state_by_name]
        if missing:
            raise SafetyError(
                'MG400 JointState 缺少观察位关节：' + ', '.join(missing)
            )

        # 关节状态不仅要“刚收到”，时间戳还必须接近图像采集时间。
        frame_stamp = Time.from_msg(frame.depth.header.stamp).nanoseconds
        state_stamp = Time.from_msg(message.header.stamp).nanoseconds
        stamp_difference = abs(frame_stamp - state_stamp) / 1e9
        maximum_difference = self._float_parameter(
            'home.capture_state_tolerance_sec'
        )
        if stamp_difference > maximum_difference:
            raise SafetyError(
                'JointState 与拍照时间相差过大：'
                f'{stamp_difference:.3f} s。'
            )

        expected = [
            float(value)
            for value in self.get_parameter(
                'home.joint_angles_rad'
            ).value
        ]
        tolerance = self._float_parameter('home.joint_tolerance_rad')
        errors = [
            self._angular_distance(float(state_by_name[name]), target)
            for name, target in zip(joint_names, expected)
        ]
        if any(error > tolerance for error in errors):
            formatted = ', '.join(f'{error:.4f}' for error in errors)
            raise SafetyError(
                '拍照时机械臂不在配置的观察关节位置，'
                f'各关节误差(rad)=[{formatted}]。'
            )

    @staticmethod
    def _angular_distance(first: float, second: float) -> float:
        """返回两角的最短绝对角距离（弧度），处理跨越正负 pi 的情况."""
        return abs(math.atan2(
            math.sin(first - second),
            math.cos(first - second),
        ))

    def _execute_or_simulate_motion(
        self,
        query: str,
        surface_point: PointStamped,
    ) -> None:
        """由基座系表面点生成法兰接近/悬停位姿，并执行任务运动序列.

        先检查半径和端点工作区。dry-run 发布规划后返回；真机模式依次
        接近、下降、抬升、归位，并在取得新的 Home 图像后报告成功。
        任意步骤抛出异常都会回到 _run_task 的失败处理。
        """
        final_standoff = self._float_parameter(
            'motion.final_standoff_m'
        )
        approach_height = self._float_parameter(
            'motion.approach_height_m'
        )
        validate_height_parameters(final_standoff, approach_height)

        x = surface_point.point.x
        y = surface_point.point.y
        # 深度点代表物体可见表面；沿基坐标 Z 轴加高度得到法兰目标。
        final_z = surface_point.point.z + final_standoff
        approach_z = surface_point.point.z + approach_height
        yaw = self._target_yaw(x, y)

        # 官方最大工作半径是硬上限，用户配置的工作区只能比它更保守。
        validate_mg400_radial_reach(x, y)
        workspace_configured = self._bool_parameter(
            'safety.workspace_configured'
        )
        if workspace_configured:
            workspace = WorkspaceBounds.from_sequence(
                self.get_parameter('safety.workspace_bounds').value
            )
            workspace.validate(x, y, final_z, '最终目标点')
            workspace.validate(x, y, approach_z, '安全接近点')

        # dry-run 仍会完整计算目标，但在这一分支绝不会创建运动目标。
        if not self._bool_parameter('motion.enabled'):
            self._publish_status(
                'DRY_RUN',
                '运动未启用，仅输出规划：'
                f'接近点=({x:.4f}, {y:.4f}, {approach_z:.4f}, '
                f'yaw={yaw:.4f})，'
                f'最终点=({x:.4f}, {y:.4f}, {final_z:.4f}, '
                f'yaw={yaw:.4f})。',
            )
            return

        workspace = self._validate_real_motion_configuration(
            workspace_configured
        )
        self._validate_home_configuration(workspace)
        self._require_robot_enabled()
        if not self._home_reached():
            raise SafetyError('视觉推理期间机械臂离开了 Home，目标运动已取消。')
        self._validate_action_servers()
        approach_pose = self._make_pose(x, y, approach_z, yaw)
        target_pose = self._make_pose(x, y, final_z, yaw)

        # 先关节运动到高处，再直线下降；避免 MovJ 直接贴近桌面。
        self._publish_status('MOVING_APPROACH', '正在移动到安全接近点。')
        self._motion_started_for_task = True
        self._send_mov_j(approach_pose)

        self._publish_status('MOVING_TARGET', '正在垂直下降到目标上方。')
        self._send_mov_l(target_pose)

        self._publish_status('FOUND', f'已找到目标：{query}')

        # 找到后沿原直线路径抬升，再返回固定观察位。
        self._publish_status('RETREATING', '正在垂直抬升。')
        self._send_mov_l(approach_pose)

        self._publish_status('RETURNING_HOME', '正在返回观察位置。')
        self._send_home()
        self._wait_for_home(self._float_parameter('home.verify_timeout_sec'))
        self._refresh_home_observation()
        self._publish_status('SUCCEEDED', f'任务完成：{query}')

    def _validate_real_motion_configuration(
        self,
        workspace_configured: bool,
    ) -> WorkspaceBounds:
        """核验真机所需联锁及速度范围，返回用于端点检查的工作区边界."""
        # 三个联锁必须由用户在完成实物验证后显式打开。
        missing = []
        if not workspace_configured:
            missing.append('safety.workspace_configured')
        if not self._bool_parameter('safety.clearance_verified'):
            missing.append('safety.clearance_verified')
        if not self._bool_parameter('home.configured'):
            missing.append('home.configured')
        if missing:
            raise SafetyError(
                '真机运动安全条件未满足：' + ', '.join(missing)
            )
        for name in (
            'motion.speed_j',
            'motion.acc_j',
            'motion.speed_l',
            'motion.acc_l',
        ):
            self._percentage_parameter(name)
        self._percentage_parameter('motion.cp', allow_zero=True)
        return WorkspaceBounds.from_sequence(
            self.get_parameter('safety.workspace_bounds').value
        )

    def _validate_home_configuration(
        self,
        workspace: WorkspaceBounds,
    ) -> None:
        """检查 Home 定义是否合法：关节模式检查角度，位姿模式检查坐标.

        pose 的顺序为 [x, y, z, yaw]，单位米/弧度；非法配置抛出 SafetyError。
        """
        mode = self._string_parameter('home.mode')
        if mode == 'joint':
            values = list(
                self.get_parameter('home.joint_angles_rad').value
            )
            validate_mg400_joint_angles(values)
            return
        if mode == 'pose':
            values = list(self.get_parameter('home.pose').value)
            if len(values) != 4 or not all(
                math.isfinite(float(value)) for value in values
            ):
                raise SafetyError(
                    'home.pose 必须包含有限的 x、y、z、yaw。'
                )
            validate_mg400_radial_reach(
                float(values[0]),
                float(values[1]),
            )
            workspace.validate(
                float(values[0]),
                float(values[1]),
                float(values[2]),
                '观察/初始位置',
            )
            return
        raise SafetyError(f'不支持的 home.mode：{mode}')

    def _validate_action_servers(self) -> None:
        """在第一段运动前确认完整任务所需的 Action Server 均可用.

        总是需要 MovJ、MovL；joint Home 模式还需要 JointMovJ。
        """
        timeout = self._float_parameter(
            'motion.action_server_timeout_sec'
        )
        # 在第一段运动前一次检查完整流程需要的服务，减少中途才发现缺失。
        required = [
            (self._mov_j_client, 'MovJ'),
            (self._mov_l_client, 'MovL'),
        ]
        if self._string_parameter('home.mode') == 'joint':
            required.append((self._joint_mov_j_client, 'JointMovJ'))
        for client, name in required:
            if not client.wait_for_server(timeout_sec=timeout):
                raise MotionError(f'{name} Action 服务不可用。')

    def _target_yaw(self, x: float, y: float) -> float:
        """计算目标 yaw（弧度）：radial 朝基座径向，fixed 采用配置固定角."""
        policy = self._string_parameter('motion.yaw_policy')
        if policy == 'radial':
            # 径向 yaw 通常可使末端旋转关节保持较小角度。
            return math.atan2(y, x)
        if policy == 'fixed':
            return self._float_parameter('motion.fixed_yaw_rad')
        raise SafetyError(f'不支持的 yaw 策略：{policy}')

    def _make_pose(
        self,
        x: float,
        y: float,
        z: float,
        yaw: float,
    ) -> PoseStamped:
        """将基座系 x/y/z（米）和 yaw（弧度）封装为运动目标 PoseStamped.

        姿态只编码绕 Z 轴的旋转，不生成 roll 或 pitch 的运动自由度。
        """
        pose = PoseStamped()
        pose.header.frame_id = self._base_frame
        pose.header.stamp = self.get_clock().now().to_msg()
        pose.pose.position.x = x
        pose.pose.position.y = y
        pose.pose.position.z = z
        # MG400 只能控制绕竖直轴的 yaw，法兰姿态始终保持垂直向下。
        pose.pose.orientation.z = math.sin(yaw / 2.0)
        pose.pose.orientation.w = math.cos(yaw / 2.0)
        return pose

    def _send_mov_j(self, pose: PoseStamped) -> None:
        """组装位姿目标及关节速度参数，并通过 _send_action 等待 MovJ 完成."""
        goal = MovJ.Goal()
        goal.pose = pose
        goal.set_speed_j = True
        goal.speed_j = self._percentage_parameter('motion.speed_j')
        goal.set_acc_j = True
        goal.acc_j = self._percentage_parameter('motion.acc_j')
        goal.set_cp = True
        goal.cp = self._percentage_parameter(
            'motion.cp', allow_zero=True
        )
        self._send_action(self._mov_j_client, goal, 'MovJ')

    def _send_mov_l(self, pose: PoseStamped) -> None:
        """组装位姿目标及直线速度参数，并通过 _send_action 等待 MovL 完成."""
        goal = MovL.Goal()
        goal.pose = pose
        goal.set_speed_l = True
        goal.speed_l = self._percentage_parameter('motion.speed_l')
        goal.set_acc_l = True
        goal.acc_l = self._percentage_parameter('motion.acc_l')
        goal.set_cp = True
        goal.cp = self._percentage_parameter(
            'motion.cp', allow_zero=True
        )
        self._send_action(self._mov_l_client, goal, 'MovL')

    def _send_home(self) -> None:
        """按 Home 模式发送 JointMovJ 或 MovJ；到位核验由调用方另行执行."""
        mode = self._string_parameter('home.mode')
        if mode == 'joint':
            angles = list(
                self.get_parameter('home.joint_angles_rad').value
            )
            if len(angles) != 4:
                raise SafetyError(
                    'home.joint_angles_rad 必须包含 4 个关节角。'
                )
            goal = JointMovJ.Goal()
            goal.joint_angles = [float(value) for value in angles]
            goal.set_speed_j = True
            goal.speed_j = self._percentage_parameter('motion.speed_j')
            goal.set_acc_j = True
            goal.acc_j = self._percentage_parameter('motion.acc_j')
            goal.set_cp = True
            goal.cp = self._percentage_parameter(
                'motion.cp', allow_zero=True
            )
            self._send_action(
                self._joint_mov_j_client,
                goal,
                'JointMovJ(home)',
            )
            return
        if mode == 'pose':
            values = list(self.get_parameter('home.pose').value)
            if len(values) != 4:
                raise SafetyError('home.pose 必须包含 x、y、z、yaw。')
            self._send_mov_j(self._make_pose(*[float(v) for v in values]))
            return
        raise SafetyError(f'不支持的 home.mode：{mode}')

    def _send_action(
        self,
        client: ActionClient,
        goal: Any,
        action_name: str,
    ) -> None:
        """发送一个 Action 目标，等待接收与结果，再确认机器人空闲.

        本函数在工作线程中等待，ROS Executor 负责处理 Future 完成回调。
        目标被拒绝、执行失败或超时时抛出 MotionError；不会可靠停机或自动重试。
        """
        if self._shutdown_event.is_set():
            raise MotionError('系统正在关闭，不再发送运动命令。')
        self._require_robot_enabled()
        server_timeout = self._float_parameter(
            'motion.action_server_timeout_sec'
        )
        if not client.wait_for_server(timeout_sec=server_timeout):
            raise MotionError(f'{action_name} Action 服务不可用。')

        goal_future = client.send_goal_async(goal)
        self._wait_for_future(goal_future, server_timeout, action_name)
        goal_handle = goal_future.result()
        if goal_handle is None or not goal_handle.accepted:
            raise MotionError(f'{action_name} 目标被拒绝。')

        # 等待超时后只停止发送后续命令；当前底层 cancel 不能可靠停机。
        result_future = goal_handle.get_result_async()
        result_timeout = self._float_parameter(
            'motion.action_result_timeout_sec'
        )
        self._wait_for_future(result_future, result_timeout, action_name)
        wrapped_result = result_future.result()
        if wrapped_result is None:
            raise MotionError(f'{action_name} 没有返回结果。')
        if wrapped_result.status != GoalStatus.STATUS_SUCCEEDED:
            raise MotionError(
                f'{action_name} 未成功，Action 状态为 '
                f'{wrapped_result.status}；'
                f'{self._format_action_error_ids(wrapped_result.result)}。'
            )
        if not wrapped_result.result.result:
            raise MotionError(
                f'{action_name} 返回执行失败；'
                f'{self._format_action_error_ids(wrapped_result.result)}。'
            )
        self._wait_for_robot_idle()

    @staticmethod
    def _format_action_error_ids(result: Any) -> str:
        """将驱动结果中的控制器和各伺服错误码整理成可读日志文本."""
        # 把驱动返回的控制器/伺服报警码带到终端，方便按手册排障。
        error_id = getattr(result, 'error_id', None)
        if error_id is None:
            return '未返回错误码'
        entries = []
        controller_ids = list(getattr(error_id.controller, 'ids', []))
        if controller_ids:
            entries.append(f'controller={controller_ids}')
        for index, servo in enumerate(error_id.servos):
            servo_ids = list(getattr(servo, 'ids', []))
            if servo_ids:
                entries.append(f'servo{index + 1}={servo_ids}')
        return ', '.join(entries) if entries else '错误码为空'

    def _wait_for_future(self, future, timeout_sec: float, name: str) -> None:
        """通过完成事件等待 ROS Future；响应异常、超时或关闭时抛出 MotionError.

        timeout_sec 为本机等待上限，name 仅用于错误提示；超时不会取消目标。
        """
        completed = threading.Event()
        future.add_done_callback(lambda unused: completed.set())
        deadline = time.monotonic() + timeout_sec
        while not completed.wait(0.1):
            if self._shutdown_event.is_set() or not rclpy.ok():
                raise MotionError(f'系统正在关闭，停止等待 {name}。')
            if time.monotonic() >= deadline:
                raise MotionError(
                    f'{name} 等待超时；当前驱动取消操作不能保证停止，'
                    '不再发送后续命令。'
                )
        if future.exception() is not None:
            raise MotionError(f'{name} 调用异常：{future.exception()}')

    def _publish_status(self, state: str, message: str) -> None:
        """把状态和说明格式化为 [STATE] 文本，同时写日志和发布状态话题."""
        text = f'[{state}] {message}'
        self.get_logger().info(text)
        output = String()
        output.data = text
        self._status_publisher.publish(output)

    def _percentage_parameter(
        self,
        name: str,
        allow_zero: bool = False,
    ) -> int:
        """读取并检查 MG400 百分比参数；速度/加速度为 1~100，CP 可允许 0."""
        value = self._int_parameter(name)
        validate_percentage(value, name, allow_zero=allow_zero)
        return value

    def _string_parameter(self, name: str) -> str:
        """按名称读取 ROS 参数并转换为字符串，供话题、帧名和策略使用."""
        return str(self.get_parameter(name).value)

    def _float_parameter(self, name: str) -> float:
        """按名称读取浮点参数；距离、角度、时间单位由相应参数定义决定."""
        return float(self.get_parameter(name).value)

    def _int_parameter(self, name: str) -> int:
        """按名称读取整数参数，用于队列长度、样本数量和速度百分比等."""
        return int(self.get_parameter(name).value)

    def _bool_parameter(self, name: str) -> bool:
        """按名称读取布尔开关，用于运动联锁、输入和校验条件."""
        return bool(self.get_parameter(name).value)


def main(args=None) -> None:
    """ROS 可执行入口：创建主节点，运行 Executor，并在退出时回收资源.

    入口由 setup.py 的 object_finder_node 指向；工作线程等待 Action 时，
    Executor 继续处理 ROS 回调，退出时先通知工作线程再销毁节点。
    """
    rclpy.init(args=args)
    node = ObjectFinderNode()
    executor = MultiThreadedExecutor(num_threads=4)
    executor.add_node(node)
    try:
        executor.spin()
    except (KeyboardInterrupt, ExternalShutdownException):
        pass
    finally:
        # launch 和 rclpy 都可能转发 SIGINT，清理期间忽略重复信号。
        signal.signal(signal.SIGINT, signal.SIG_IGN)
        node.stop_terminal_input()
        executor.shutdown()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
