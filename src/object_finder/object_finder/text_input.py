"""终端客户端：等待 Home 后开放文字输入，或只显示外部语音模式的任务状态."""

import select
import signal
import sys
import time
import math

import rclpy
from rclpy.node import Node
from rclpy.executors import ExternalShutdownException
from std_msgs.msg import Bool, String

from .readiness import ready_qos


class TextInputNode(Node):
    """在终端显示任务状态，并在任务节点就绪时发布文字指令。"""

    def __init__(self, *, parameter_overrides=None) -> None:
        """创建输入发布器与状态订阅，校验首次等待 READY 的超时时间。

        parameter_overrides 可覆盖节点参数。ready 负责控制是否可提交任务，
        startup_failed 负责让首次启动等待在收到归位失败状态后及时结束。
        """
        super().__init__(
            'object_finder_text_input',
            parameter_overrides=parameter_overrides,
        )
        # input_topic 发给输入桥；status_topic 接收主任务节点的状态播报。
        self.declare_parameter(
            'input_topic',
            '/object_finder/text_input',
        )
        self.declare_parameter('status_topic', '/object_finder/status')
        self.declare_parameter('ready_topic', '/object_finder/ready')
        # 覆盖使能等待、单段归位和图像确认的最长组合，避免提前关闭硬件。
        self.declare_parameter('ready_timeout_sec', 120.0)
        ready_timeout = float(self.get_parameter('ready_timeout_sec').value)
        if not math.isfinite(ready_timeout) or ready_timeout <= 0.0:
            raise ValueError('ready_timeout_sec 必须是正有限数。')
        self.ready = False
        self.startup_failed = False

        input_topic = str(self.get_parameter('input_topic').value)
        status_topic = str(self.get_parameter('status_topic').value)
        self._publisher = self.create_publisher(String, input_topic, 10)
        self._subscription = self.create_subscription(
            String,
            status_topic,
            self._status_callback,
            ready_qos(),
        )
        self._ready_subscription = self.create_subscription(
            Bool,
            str(self.get_parameter('ready_topic').value),
            self._ready_callback,
            ready_qos(),
        )

    def publish_command(self, command: str) -> bool:
        """清理并校验一条终端指令；成功发布返回 True，被拒绝返回 False。

        发布目标是输入桥的话题，而不是任务节点。发布后立即关闭本地
        ready，等任务节点再次发出 READY，避免用户连续提交多个任务。
        """
        normalized = command.strip()
        if not normalized:
            return False
        if not self.ready:
            print('系统尚未准备好或任务执行中，请等待返回 Home。', flush=True)
            return False
        if len(normalized) > 500:
            print('指令超过 500 个字符，请缩短后重新输入。', flush=True)
            return False
        try:
            normalized.encode('utf-8')
        except UnicodeEncodeError:
            print('输入的中文编码无效，请重新输入或完整粘贴指令。', flush=True)
            return False
        output = String()
        output.data = normalized
        self._publisher.publish(output)
        self.ready = False
        return True

    def _status_callback(self, message: String) -> None:
        """显示任务节点的状态文本，并识别 STARTUP_FAILED 以终止启动等待。"""
        if message.data.startswith('[STARTUP_FAILED]'):
            self.startup_failed = True
        print(f'\n{message.data}', flush=True)

    def _ready_callback(self, message: Bool) -> None:
        """同步任务节点的输入门控；终端是否显示提示、是否允许发布都使用它。"""
        self.ready = message.data

    def wait_until_ready(self) -> bool:
        """处理 ROS 回调直到首次 READY，成功返回 True，超时或启动失败返回 False。

        monotonic 计时不受系统时钟调整影响；spin_once 让状态和就绪订阅
        在等待期间继续更新，避免单纯阻塞等待却收不到 READY。
        """
        deadline = time.monotonic() + float(
            self.get_parameter('ready_timeout_sec').value
        )
        print('等待机械臂归位和相机准备完成……', flush=True)
        while rclpy.ok() and not self.startup_failed:
            rclpy.spin_once(self, timeout_sec=0.1)
            if self.ready:
                return True
            if time.monotonic() >= deadline:
                print('等待 Home 就绪超时，请检查启动日志。', flush=True)
                return False
        return False


def _run_input(mode: str, args=None) -> int:
    """运行文字输入或语音状态监视；返回进程退出码 0、1 或 130。

    mode='text' 时交替处理 ROS 回调和终端输入，'voice' 时仅显示状态，
    等待外部 ASR 发布文本。首次等待失败返回 1，Ctrl+C 返回 130。
    """
    rclpy.init(args=args)
    node = TextInputNode()
    prompt = '请输入要寻找的物体（quit 退出）：'
    try:
        if not node.wait_until_ready():
            return 1
        if mode == 'voice':
            print('Home 已就绪，等待 /object_finder/asr_text 的识别结果。', flush=True)
            print('按 Ctrl+C 退出。', flush=True)
            rclpy.spin(node)
            return 0
        prompt_shown = False
        while rclpy.ok():
            rclpy.spin_once(node, timeout_sec=0.1)
            if node.ready and not prompt_shown:
                print(prompt, end='', flush=True)
                prompt_shown = True
            # 短时间轮询 stdin，使终端等待输入时仍能及时处理任务状态回调。
            readable, _, _ = select.select([sys.stdin], [], [], 0.1)
            if not readable:
                continue
            line = sys.stdin.readline()
            if line == '':
                break
            command = line.strip()
            if command.lower() in {'quit', 'exit'}:
                break
            # 发布成功后等下一次 READY 再提示；校验失败但仍就绪时立即再提示。
            if node.publish_command(command) or node.ready:
                prompt_shown = False
    except (KeyboardInterrupt, ExternalShutdownException):
        return 130
    finally:
        signal.signal(signal.SIGINT, signal.SIG_IGN)
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()
    return 0


def main(args=None) -> int:
    """文字模式入口：等待归位和相机就绪后，反复读取并提交终端指令。"""
    return _run_input('text', args)


def voice_main(args=None) -> int:
    """语音模式入口：等待 READY 并显示状态；语音识别由外部 ASR 节点完成。"""
    return _run_input('voice', args)


if __name__ == '__main__':
    main()
