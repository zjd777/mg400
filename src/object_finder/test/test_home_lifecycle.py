"""隔离 ROS 域中的模拟验收：不连接真机，验证启动归位、输入门控和任务返回."""

import math
import threading
import time
import uuid

import numpy as np
import pytest

pytest.importorskip('rclpy')
pytest.importorskip('mg400_msgs.action')

from cv_bridge import CvBridge  # noqa: E402
from geometry_msgs.msg import TransformStamped  # noqa: E402
from mg400_msgs.action import MovJ, MovL  # noqa: E402
from mg400_msgs.msg import RobotMode  # noqa: E402
import rclpy  # noqa: E402
from rclpy.action import ActionServer, GoalResponse  # noqa: E402
from rclpy.callback_groups import ReentrantCallbackGroup  # noqa: E402
from rclpy.executors import MultiThreadedExecutor  # noqa: E402
from rclpy.node import Node  # noqa: E402
from rclpy.parameter import Parameter  # noqa: E402
from rclpy.qos import qos_profile_sensor_data  # noqa: E402
from sensor_msgs.msg import CameraInfo, Image  # noqa: E402
from std_msgs.msg import String  # noqa: E402
from tf2_ros import TransformBroadcaster  # noqa: E402

from object_finder.command_bridge import CommandBridge  # noqa: E402
from object_finder.models import VisionError  # noqa: E402
from object_finder.readiness import ready_qos  # noqa: E402
from object_finder.task_node import ObjectFinderNode  # noqa: E402
from object_finder.text_input import TextInputNode  # noqa: E402


HOME = (0.350, 0.000, 0.150, 0.000)


def wait_for(predicate, timeout=8.0):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(0.02)
    assert predicate(), '模拟 ROS 流程未在期限内达到预期状态'


def overrides(values):
    return [Parameter(name, value=value) for name, value in values.items()]


class FakeHardware(Node):
    """只在隔离域和测试命名空间发布状态，Action 不包含任何真实硬件调用."""

    def __init__(self, prefix, pose, mode, reject_home=False):
        super().__init__('fake_home_hardware')
        self.prefix = prefix
        self.pose = pose
        self.mode = mode
        self.reject_home = reject_home
        self.fail_next_action = False
        self.publish_frames = True
        self.publish_tf = True
        self.idle_after = 0.0
        self.goals = []
        self.statuses = []
        self.broadcast = TransformBroadcaster(self)
        self.bridge = CvBridge()
        self.mode_pub = self.create_publisher(
            RobotMode, prefix + '/robot_mode', qos_profile_sensor_data,
        )
        self.color_pub = self.create_publisher(
            Image, prefix + '/color', qos_profile_sensor_data,
        )
        self.depth_pub = self.create_publisher(
            Image, prefix + '/depth', qos_profile_sensor_data,
        )
        self.info_pub = self.create_publisher(
            CameraInfo, prefix + '/info', qos_profile_sensor_data,
        )
        self.input_pub = self.create_publisher(String, prefix + '/input', 10)
        self.status_sub = self.create_subscription(
            String, prefix + '/status',
            lambda msg: self.statuses.append(msg.data), ready_qos(),
        )
        group = ReentrantCallbackGroup()
        self.servers = [
            ActionServer(
                self, action, prefix + '/' + name,
                execute_callback=lambda goal, a=action, n=name:
                self.execute(goal, a, n),
                goal_callback=self.accept, callback_group=group,
            )
            for action, name in ((MovJ, 'mov_j'), (MovL, 'mov_l'))
        ]
        self.timer = self.create_timer(0.025, self.publish_state)

    def accept(self, goal):
        return GoalResponse.REJECT if self.reject_home else GoalResponse.ACCEPT

    def execute(self, goal, action, name):
        target = goal.request.pose
        point = target.pose.position
        rotation = target.pose.orientation
        yaw = 2.0 * math.atan2(rotation.z, rotation.w)
        self.goals.append((name, (point.x, point.y, point.z, yaw)))
        time.sleep(0.06)
        result = action.Result()
        if self.fail_next_action:
            self.fail_next_action = False
            result.result = False
            goal.abort()
        else:
            self.pose = (point.x, point.y, point.z, yaw)
            # 模拟 Action 返回后状态话题稍晚才恢复 ENABLE，不能误判失败。
            self.idle_after = time.monotonic() + 0.08
            result.result = True
            goal.succeed()
        return result

    def publish_state(self):
        mode = self.mode
        if time.monotonic() < self.idle_after:
            mode = RobotMode.RUNNING
        self.mode_pub.publish(RobotMode(robot_mode=mode))
        stamp = self.get_clock().now().to_msg()
        if self.publish_tf:
            flange = TransformStamped()
            flange.header.stamp = stamp
            flange.header.frame_id = 'test_home_origin'
            flange.child_frame_id = 'test_home_flange'
            xyz = flange.transform.translation
            xyz.x, xyz.y, xyz.z = self.pose[:3]
            flange.transform.rotation.z = math.sin(self.pose[3] / 2.0)
            flange.transform.rotation.w = math.cos(self.pose[3] / 2.0)
            optical = TransformStamped()
            optical.header = flange.header
            optical.child_frame_id = 'test_home_optical'
            optical.transform.translation.x = 0.32
            optical.transform.translation.z = 0.35
            optical.transform.rotation.x = 1.0  # 光学 +Z 朝基座 -Z。
            optical.transform.rotation.w = 0.0
            self.broadcast.sendTransform([flange, optical])
        if not self.publish_frames:
            return
        color = self.bridge.cv2_to_imgmsg(
            np.zeros((64, 64, 3), dtype=np.uint8), encoding='bgr8',
        )
        depth = self.bridge.cv2_to_imgmsg(
            np.full((64, 64), 0.4, dtype=np.float32), encoding='32FC1',
        )
        color.header.stamp = stamp
        color.header.frame_id = 'test_home_optical'
        depth.header = color.header
        info = CameraInfo()
        info.header = color.header
        info.width = info.height = 64
        info.k = [100.0, 0.0, 32.0, 0.0, 100.0, 32.0, 0.0, 0.0, 1.0]
        self.color_pub.publish(color)
        self.depth_pub.publish(depth)
        self.info_pub.publish(info)

    def saw(self, state):
        return any(msg.startswith('[' + state + ']') for msg in self.statuses)


@pytest.fixture
def system():
    """使用独立 ROS domain 197 和独立 Action 名称，绝不触达 /mg400 真机."""
    rclpy.init(args=[], domain_id=197)
    nodes = []
    executor = MultiThreadedExecutor(num_threads=6)
    spin_thread = threading.Thread(target=executor.spin, daemon=True)
    spin_thread.start()

    def start(pose=HOME, mode=RobotMode.ENABLE, reject_home=False,
              frames=True, tf=True):
        # 每个用例使用新命名空间，避免 DDS 尚未回收的旧 Action 发现记录串扰。
        prefix = '/object_finder_home_test_' + uuid.uuid4().hex
        fake = FakeHardware(prefix, pose, mode, reject_home)
        fake.publish_frames = frames
        fake.publish_tf = tf
        values = {
            'motion.enabled': True,
            'home.configured': True,
            'home.mode': 'pose',
            'home.pose': list(HOME),
            'home.startup_timeout_sec': 2.0,
            'home.verify_timeout_sec': 2.0,
            'home.settle_sec': 0.05,
            'safety.workspace_configured': True,
            'safety.clearance_verified': True,
            'safety.workspace_bounds': [0.25, 0.36, -0.1, 0.1, -0.11, 0.2],
            'frames.base': 'test_home_origin',
            'frames.flange': 'test_home_flange',
            'vision.provider': 'fixed_bbox',
            'terminal.enabled': False,
            'actions.mov_j': prefix + '/mov_j',
            'actions.mov_l': prefix + '/mov_l',
            'actions.joint_mov_j': prefix + '/joint_mov_j',
        }
        for name in ('command', 'status', 'ready', 'robot_mode'):
            values['topics.' + name] = prefix + '/' + name
        values.update({
            'topics.color': prefix + '/color',
            'topics.depth': prefix + '/depth',
            'topics.camera_info': prefix + '/info',
        })
        finder = ObjectFinderNode(parameter_overrides=overrides(values))
        bridge = CommandBridge(parameter_overrides=overrides({
            'input_topic': prefix + '/input',
            'output_topic': prefix + '/command',
            'ready_topic': prefix + '/ready',
        }))
        client = TextInputNode(parameter_overrides=overrides({
            'input_topic': prefix + '/input',
            'status_topic': prefix + '/status',
            'ready_topic': prefix + '/ready',
        }))
        nodes.extend([fake, finder, bridge, client])
        for node in nodes:
            executor.add_node(node)
        return fake, finder, bridge, client

    yield start
    for node in nodes:
        if isinstance(node, ObjectFinderNode):
            node.stop_terminal_input()
    executor.shutdown(timeout_sec=3.0)
    spin_thread.join(timeout=3.0)
    for node in reversed(nodes):
        node.destroy_node()
    rclpy.shutdown()


@pytest.mark.parametrize('pose,goal_count', [(HOME, 0),
                         ((0.30, 0.0, 0.10, 0.0), 1)])
def test_startup_only_homes_if_needed(system, pose, goal_count):
    fake, finder, bridge, client = system(pose=pose)
    wait_for(lambda: finder._input_ready and bridge._ready and client.ready)
    assert len(fake.goals) == goal_count
    if goal_count:
        assert fake.goals[0] == ('mov_j', HOME)
    assert fake.saw('HOME_REACHED')
    assert finder._latest_frame is not None


def test_wait_enable_blocks_both_input_paths(system):
    fake, finder, bridge, client = system(
        pose=(0.30, 0.0, 0.10, 0.0), mode=RobotMode.DISABLED,
    )
    wait_for(lambda: finder._latest_robot_mode == RobotMode.DISABLED)
    assert not client.publish_command('白色杯子')
    bridge._forward('模拟 ASR 的白色杯子')
    finder._submit_task('直接发送的白色杯子')
    assert not finder._task_active
    assert not fake.goals
    fake.mode = RobotMode.ENABLE
    wait_for(lambda: client.ready and bridge._ready)
    assert fake.goals == [('mov_j', HOME)]


@pytest.mark.parametrize('kwargs', [
    {'pose': (0.30, 0.0, 0.10, 0.0), 'reject_home': True},
    {'frames': False},
    {'tf': False},
])
def test_startup_failure_never_opens_input(system, kwargs):
    fake, finder, bridge, client = system(**kwargs)
    wait_for(lambda: client.startup_failed)
    assert not finder._input_ready
    assert not bridge._ready
    assert not client.ready
    assert fake.saw('STARTUP_FAILED')


def test_two_tasks_each_return_home_and_reopen_input(system):
    fake, finder, bridge, client = system()
    wait_for(lambda: client.ready and bridge._ready)
    for index in range(2):
        assert client.publish_command('白色杯子')
        wait_for(lambda: len(fake.goals) == (index + 1) * 4)
        wait_for(lambda: client.ready and bridge._ready)
        assert fake.goals[-1] == ('mov_j', HOME)
        assert [name for name, _ in fake.goals[-4:]] == [
            'mov_j', 'mov_l', 'mov_l', 'mov_j',
        ]
        assert fake.pose == HOME
    assert sum(msg.startswith('[SUCCEEDED]') for msg in fake.statuses) == 2


def test_motion_failure_stays_locked_without_auto_return(system):
    fake, finder, bridge, client = system()
    wait_for(lambda: client.ready and bridge._ready)
    fake.fail_next_action = True
    assert client.publish_command('白色杯子')
    wait_for(lambda: fake.saw('FAILED') and not finder._task_active)
    assert len(fake.goals) == 1
    assert not finder._input_ready
    assert not client.ready
    assert not bridge._ready
    bridge._forward('第二次语音输入')
    assert len(fake.goals) == 1


def test_perception_failure_at_home_can_retry(system):
    fake, finder, bridge, client = system()
    wait_for(lambda: client.ready and bridge._ready)

    def failed_vision(*unused):
        raise VisionError('模拟没找到物体')

    finder._vision_grounder.locate = failed_vision
    assert client.publish_command('白色杯子')
    wait_for(lambda: fake.saw('FAILED'))
    wait_for(lambda: client.ready and bridge._ready)
    assert not fake.goals


def test_invalid_utf8_is_rejected_without_crashing(system):
    fake, finder, bridge, client = system()
    wait_for(lambda: client.ready and bridge._ready)
    assert not client.publish_command('找到\udce8物体')
    assert client.ready
    assert not fake.goals


def test_missing_frame_rejection_does_not_lock_clients(system):
    fake, finder, bridge, client = system()
    wait_for(lambda: client.ready and bridge._ready)
    fake.publish_frames = False
    fake.timer.cancel()
    with finder._frame_lock:
        finder._latest_frame = None
    assert client.publish_command('白色杯子')
    wait_for(lambda: fake.saw('WAITING_FRAME'))
    wait_for(lambda: client.ready and bridge._ready)
    assert not fake.goals
