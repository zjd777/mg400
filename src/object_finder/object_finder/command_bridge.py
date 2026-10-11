"""输入桥接节点：校验终端文字或 ASR 结果，并转发到统一找物指令话题."""

import math
import signal

import rclpy
from rclpy.node import Node
from std_msgs.msg import String


class CommandBridge(Node):
    """Validate an input string before forwarding it as a find command."""

    def __init__(self) -> None:
        super().__init__('object_finder_command_bridge')
        # input_topic 随文字/语音模式改变；output_topic 是任务节点的稳定入口。
        self.declare_parameter('input_topic', '/object_finder/text_input')
        self.declare_parameter('output_topic', '/object_finder/command')
        # source_name 仅用于日志；initial_command 用于无人工输入的联调测试。
        self.declare_parameter('source_name', '文字')
        self.declare_parameter('initial_command', '')
        # 延时应覆盖相机、TF 和模型节点的启动时间。
        self.declare_parameter('initial_delay_sec', 5.0)

        self._input_topic = str(self.get_parameter('input_topic').value)
        self._output_topic = str(self.get_parameter('output_topic').value)
        self._source_name = str(self.get_parameter('source_name').value)
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
            f'{self._source_name}输入已启用：'
            f'{self._input_topic} -> {self._output_topic}'
        )

    def _validate_parameters(self) -> None:
        if not self._input_topic or not self._output_topic:
            raise ValueError('输入和输出话题不能为空。')
        if self._input_topic == self._output_topic:
            raise ValueError('输入和输出话题不能相同，否则会循环发布。')
        delay = float(self.get_parameter('initial_delay_sec').value)
        if not math.isfinite(delay) or delay <= 0.0:
            raise ValueError('initial_delay_sec 必须大于 0。')

    def _input_callback(self, message: String) -> None:
        self._forward(message.data)

    def _publish_initial_command(self) -> None:
        if self._initial_timer is not None:
            self._initial_timer.cancel()
        self._forward(self._initial_command)

    def _forward(self, raw_command: str) -> None:
        command = raw_command.strip()
        if not command:
            self.get_logger().warning('收到空指令，已忽略。')
            return
        if len(command) > 500:
            self.get_logger().warning('指令超过 500 个字符，已拒绝。')
            return

        output = String()
        output.data = command
        self._publisher.publish(output)
        self.get_logger().info(
            f'已转发{self._source_name}指令：{command}'
        )


def main(args=None) -> None:
    """Run the command bridge."""
    rclpy.init(args=args)
    node = CommandBridge()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        # ros2 launch 可能向子进程重复转发 SIGINT。
        signal.signal(signal.SIGINT, signal.SIG_IGN)
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
