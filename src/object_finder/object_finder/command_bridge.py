"""输入桥接节点：校验终端文字或 ASR 结果，并转发到统一找物指令话题."""

import math
import signal

import rclpy
from rclpy.executors import ExternalShutdownException
from rclpy.node import Node
from std_msgs.msg import Bool, String

from .readiness import ready_qos


class CommandBridge(Node):
    """把文字输入或 ASR 文本统一转成找物指令，并根据 READY 状态控制接收。"""

    def __init__(self, *, parameter_overrides=None) -> None:
        """读取 launch/YAML 参数，连接输入、输出和任务就绪话题。

        parameter_overrides 主要供测试或启动配置覆盖默认参数；初始化后只
        注册 ROS 回调，真正的输入转发由 spin 中的回调触发。
        """
        super().__init__(
            'object_finder_command_bridge',
            parameter_overrides=parameter_overrides,
        )
        # input_topic 随文字/语音模式改变；output_topic 是任务节点的稳定入口。
        self.declare_parameter('input_topic', '/object_finder/text_input')
        self.declare_parameter('output_topic', '/object_finder/command')
        self.declare_parameter('ready_topic', '/object_finder/ready')
        # source_name 仅用于日志；initial_command 用于无人工输入的联调测试。
        self.declare_parameter('source_name', '文字')
        self.declare_parameter('initial_command', '')
        # 定时尝试发送测试指令；真正是否允许发送，仍由 READY 门控决定。
        self.declare_parameter('initial_delay_sec', 5.0)

        self._input_topic = str(self.get_parameter('input_topic').value)
        self._output_topic = str(self.get_parameter('output_topic').value)
        self._source_name = str(self.get_parameter('source_name').value)
        self._ready = False  # 收到任务节点的 READY=True 前，不接受找物指令。
        self._validate_parameters()

        # 输入源和任务节点解耦；以后替换 ASR 不需改动任务逻辑。
        self._publisher = self.create_publisher(
            String,
            self._output_topic,
            10,
        )
        self._subscription = self.create_subscription(
            String,
            self._input_topic,
            self._input_callback,
            10,
        )
        # 外部 ASR 可同样订阅 ready_topic，归位/执行期间暂停录音或识别。
        self._ready_subscription = self.create_subscription(
            Bool,
            str(self.get_parameter('ready_topic').value),
            self._ready_callback,
            ready_qos(),
        )

        initial_command = str(
            self.get_parameter('initial_command').value
        ).strip()
        self._initial_timer = None
        if initial_command:
            self._initial_command = initial_command
            delay = float(self.get_parameter('initial_delay_sec').value)
            self._initial_timer = self.create_timer(
                delay,
                self._publish_initial_command,
            )

        self.get_logger().info(
            f'{self._source_name}输入桥已启动（等待 Home 就绪）：'
            f'{self._input_topic} -> {self._output_topic}'
        )

    def _validate_parameters(self) -> None:
        """校验输入/输出话题与测试指令定时周期；配置不合法时抛出 ValueError。"""
        if not self._input_topic or not self._output_topic:
            raise ValueError('输入和输出话题不能为空。')
        if self._input_topic == self._output_topic:
            raise ValueError('输入和输出话题不能相同，否则会循环发布。')
        delay = float(self.get_parameter('initial_delay_sec').value)
        if not math.isfinite(delay) or delay <= 0.0:
            raise ValueError('initial_delay_sec 必须大于 0。')

    def _input_callback(self, message: String) -> None:
        """收到文字或 ASR 的 String 消息后，把正文交给统一校验和转发流程。"""
        self._forward(message.data)

    def _ready_callback(self, message: Bool) -> None:
        """记录任务节点的就绪状态；True 开放输入，False 表示当前不接收任务。"""
        self._ready = message.data

    def _publish_initial_command(self) -> None:
        """定时检查 READY，就绪后取消定时器并尝试发送一次 initial_command。

        未就绪时保留定时器，等下一个周期重试；指令本身仍需通过 _forward
        的内容校验，因此“尝试发送一次”不等于任务节点已接受该指令。
        """
        if not self._ready:
            return
        if self._initial_timer is not None:
            self._initial_timer.cancel()
        self._forward(self._initial_command)

    def _forward(self, raw_command: str) -> None:
        """校验原始文本，发布 String 指令，然后暂时关闭本地输入门控。

        raw_command 可以来自终端、ASR 或初始测试指令。未就绪、空文本、
        超长文本或无法编码为 UTF-8 的文本会直接被忽略，不缓存等待执行。
        """
        if not self._ready:
            self.get_logger().warning('尚未准备好或任务执行中，本次输入已忽略。')
            return
        command = raw_command.strip()
        if not command:
            self.get_logger().warning('收到空指令，已忽略。')
            return
        if len(command) > 500:
            self.get_logger().warning('指令超过 500 个字符，已拒绝。')
            return
        try:
            command.encode('utf-8')
        except UnicodeEncodeError:
            self.get_logger().warning('输入包含无效 UTF-8 字符，请重新输入。')
            return

        output = String()
        output.data = command
        self._publisher.publish(output)
        # 发布一条后关闭本地门控，等任务节点再次明确宣布 READY。
        self._ready = False
        self.get_logger().info(
            f'已转发{self._source_name}指令：{command}'
        )


def main(args=None) -> None:
    """ROS 可执行入口：创建输入桥并持续处理回调，退出时释放节点和上下文。"""
    rclpy.init(args=args)
    node = CommandBridge()
    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, ExternalShutdownException):
        pass
    finally:
        # ros2 launch 可能向子进程重复转发 SIGINT。
        signal.signal(signal.SIGINT, signal.SIG_IGN)
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
