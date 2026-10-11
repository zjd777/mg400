"""文字模式终端客户端：读取多次用户输入、发布找物指令并显示任务状态."""

import select
import signal
import sys

import rclpy
from rclpy.node import Node
from std_msgs.msg import String


class TextInputNode(Node):
    """Publish repeated terminal input and print task status messages."""

    def __init__(self) -> None:
        super().__init__('object_finder_text_input')
        # input_topic 发给输入桥；status_topic 接收主任务节点的状态播报。
        self.declare_parameter(
            'input_topic',
            '/object_finder/text_input',
        )
        self.declare_parameter('status_topic', '/object_finder/status')

        input_topic = str(self.get_parameter('input_topic').value)
        status_topic = str(self.get_parameter('status_topic').value)
        self._publisher = self.create_publisher(String, input_topic, 10)
        self._subscription = self.create_subscription(
            String,
            status_topic,
            self._status_callback,
            10,
        )

    def publish_command(self, command: str) -> None:
        """Publish one non-empty command from the terminal."""
        normalized = command.strip()
        if not normalized:
            return
        output = String()
        output.data = normalized
        self._publisher.publish(output)

    @staticmethod
    def _status_callback(message: String) -> None:
        print(f'\n{message.data}', flush=True)


def main(args=None) -> None:
    """Run an interruptible terminal loop alongside ROS callbacks."""
    rclpy.init(args=args)
    node = TextInputNode()
    prompt = '请输入要寻找的物体（quit 退出）：'
    print(prompt, end='', flush=True)
    try:
        while rclpy.ok():
            rclpy.spin_once(node, timeout_sec=0.1)
            readable, _, _ = select.select([sys.stdin], [], [], 0.1)
            if not readable:
                continue
            line = sys.stdin.readline()
            if line == '':
                break
            command = line.strip()
            if command.lower() in {'quit', 'exit'}:
                break
            node.publish_command(command)
            print(prompt, end='', flush=True)
    except KeyboardInterrupt:
        pass
    finally:
        signal.signal(signal.SIGINT, signal.SIG_IGN)
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
