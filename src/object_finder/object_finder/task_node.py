"""ROS 2 task coordinator for terminal-driven MG400 object finding."""

from dataclasses import dataclass
import math
import threading
import time
from typing import Any, Optional

from action_msgs.msg import GoalStatus
from cv_bridge import CvBridge
from geometry_msgs.msg import PointStamped, PoseStamped
from message_filters import ApproximateTimeSynchronizer, Subscriber
from mg400_msgs.action import JointMovJ, MovJ, MovL
import rclpy
from rclpy.action import ActionClient
from rclpy.duration import Duration
from rclpy.executors import MultiThreadedExecutor
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from rclpy.time import Time
from sensor_msgs.msg import CameraInfo, Image, JointState
from std_msgs.msg import String
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
    """One approximately synchronized RGB-D observation."""

    # 三条消息必须作为同一次观测冻结，任务执行期间不能被新帧替换。
    color: Image
    depth: Image
    camera_info: CameraInfo
    received_monotonic: float


@dataclass(frozen=True)
class JointStateSnapshot:
    """Most recently received MG400 joint state."""

    message: JointState
    received_monotonic: float


class ObjectFinderNode(Node):
    """Coordinate perception and motion while enforcing safe defaults."""

    def __init__(self) -> None:
        super().__init__('object_finder')
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
        self._task_active = False
        self._motion_started_for_task = False

        self._base_frame = self._string_parameter('frames.base')
        # 保存历史 TF，确保使用“拍照时刻”而不是“开始运动后”的相机位姿。
        self._tf_buffer = Buffer(cache_time=Duration(seconds=15.0))
        self._tf_listener = TransformListener(self._tf_buffer, self)

        # command/status 是未来语音识别和语音播报模块的稳定接口。
        self._status_publisher = self.create_publisher(
            String,
            self._string_parameter('topics.status'),
            10,
        )
        self._target_publisher = self.create_publisher(
            PointStamped,
            self._string_parameter('topics.target_point'),
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

        self._publish_status(
            'IDLE',
            '节点已启动。默认禁止真机运动，请等待同步 RGB-D 图像。',
        )
        if self._bool_parameter('terminal.enabled'):
            threading.Thread(
                target=self._terminal_loop,
                name='object_finder_terminal',
                daemon=True,
            ).start()

    def _declare_parameters(self) -> None:
        parameters = {
            'topics.color': '/camera/camera/color/image_raw',
            'topics.depth': (
                '/camera/camera/aligned_depth_to_color/image_raw'
            ),
            'topics.camera_info': '/camera/camera/color/camera_info',
            'topics.command': '/object_finder/command',
            'topics.status': '/object_finder/status',
            'topics.target_point': '/object_finder/target_point',
            'topics.joint_states': '/mg400/joint_states',
            'frames.base': 'mg400_origin_link',
            'frames.flange': 'mg400_end_effector_flange',
            'sync.queue_size': 10,
            'sync.slop_sec': 0.08,
            'sync.maximum_frame_age_sec': 1.0,
            'sync.require_matching_frames': True,
            'tf.timeout_sec': 1.0,
            'depth.scale_16u': 0.001,
            'depth.minimum_m': 0.10,
            'depth.maximum_m': 2.0,
            'depth.minimum_valid_ratio': 0.20,
            'depth.minimum_valid_samples': 25,
            'depth.roi_shrink_ratio': 0.20,
            'vision.provider': 'disabled',
            'vision.minimum_confidence': 0.60,
            'vision.selection_policy': 'require_single',
            'vision.fixed_bbox': [0.35, 0.35, 0.65, 0.65],
            'vision.qwen.model': '',
            'vision.qwen.base_url': '',
            'vision.qwen.api_key_env': 'DASHSCOPE_API_KEY',
            'vision.qwen.timeout_sec': 30.0,
            'vision.qwen.max_retries': 2,
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
            'motion.action_result_timeout_sec': 15.0,
            'actions.mov_j': '/mg400/mov_j',
            'actions.mov_l': '/mg400/mov_l',
            'actions.joint_mov_j': '/mg400/joint_mov_j',
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
            'home.joint_tolerance_rad': 0.035,
            'home.position_tolerance_m': 0.01,
            'home.yaw_tolerance_rad': 0.035,
            'home.state_max_age_sec': 0.5,
            'home.capture_state_tolerance_sec': 0.25,
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
        # 配置采用失败关闭策略：不完整或非法参数不能带病运行。
        required_strings = (
            'topics.color',
            'topics.depth',
            'topics.camera_info',
            'topics.command',
            'topics.status',
            'topics.target_point',
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
        nonnegative_parameters = ('sync.slop_sec',)
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
        # 回调只保存最新完整观测，耗时的视觉推理放到独立任务线程。
        with self._frame_lock:
            self._latest_frame = FrameBundle(
                color=color,
                depth=depth,
                camera_info=camera_info,
                received_monotonic=time.monotonic(),
            )

    def _joint_state_callback(self, message: JointState) -> None:
        with self._joint_state_lock:
            self._latest_joint_state = JointStateSnapshot(
                message=message,
                received_monotonic=time.monotonic(),
            )

    def _command_callback(self, message: String) -> None:
        self._submit_task(message.data)

    def _terminal_loop(self) -> None:
        while rclpy.ok():
            try:
                query = input('\n请输入要寻找的物体（quit 退出输入）：').strip()
            except (EOFError, KeyboardInterrupt):
                return
            if not query:
                continue
            if query.lower() in {'quit', 'exit'}:
                self._publish_status('IDLE', '终端输入线程已退出。')
                return
            self._submit_task(query)

    def _submit_task(self, query: str) -> None:
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
            # 在这里复制引用并冻结本次任务使用的观测帧。
            with self._frame_lock:
                frame = self._latest_frame
            if frame is None:
                self._publish_status(
                    'WAITING_FRAME',
                    '尚未收到同步 RGB-D 图像，机械臂保持不动。',
                )
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
                return
            self._task_active = True

        threading.Thread(
            target=self._run_task,
            args=(normalized_query, frame),
            name='object_finder_task',
            daemon=True,
        ).start()

    def _run_task(self, query: str, frame: FrameBundle) -> None:
        self._motion_started_for_task = False
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
            detection = select_single_detection(
                self._vision_grounder.locate(color_bgr, query),
                minimum_confidence=self._float_parameter(
                    'vision.minimum_confidence'
                ),
                policy=self._string_parameter(
                    'vision.selection_policy'
                ),
            )

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
            optical_point.header = frame.depth.header
            optical_point.point.x = localized.x
            optical_point.point.y = localized.y
            optical_point.point.z = localized.z
            base_point = self._transform_to_base(optical_point)
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
        except Exception as error:  # defensive boundary for physical safety
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
            self._motion_started_for_task = False
            with self._task_lock:
                self._task_active = False

    def _validate_frame_bundle(self, frame: FrameBundle) -> None:
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
        return abs(math.atan2(
            math.sin(first - second),
            math.cos(first - second),
        ))

    def _execute_or_simulate_motion(
        self,
        query: str,
        surface_point: PointStamped,
    ) -> None:
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
        self._publish_status('SUCCEEDED', f'任务完成：{query}')

    def _validate_real_motion_configuration(
        self,
        workspace_configured: bool,
    ) -> WorkspaceBounds:
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

    @staticmethod
    def _format_action_error_ids(result: Any) -> str:
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

    @staticmethod
    def _wait_for_future(future, timeout_sec: float, name: str) -> None:
        completed = threading.Event()
        future.add_done_callback(lambda unused: completed.set())
        if not completed.wait(timeout_sec):
            raise MotionError(
                f'{name} 等待超时；当前驱动取消操作不能保证停止，'
                '不再发送后续命令。'
            )
        if future.exception() is not None:
            raise MotionError(f'{name} 调用异常：{future.exception()}')

    def _publish_status(self, state: str, message: str) -> None:
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
        value = self._int_parameter(name)
        validate_percentage(value, name, allow_zero=allow_zero)
        return value

    def _string_parameter(self, name: str) -> str:
        return str(self.get_parameter(name).value)

    def _float_parameter(self, name: str) -> float:
        return float(self.get_parameter(name).value)

    def _int_parameter(self, name: str) -> int:
        return int(self.get_parameter(name).value)

    def _bool_parameter(self, name: str) -> bool:
        return bool(self.get_parameter(name).value)


def main(args=None) -> None:
    """Run the object finder with an executor that can service actions."""
    rclpy.init(args=args)
    node = ObjectFinderNode()
    executor = MultiThreadedExecutor(num_threads=4)
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        pass
    finally:
        executor.shutdown()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
