"""MG400 上/下使能客户端：根据机器人状态安全地切换伺服使能."""

import math
import time
from typing import Any, Optional, Tuple

from mg400_msgs.msg import RobotMode as RobotModeMsg
from mg400_msgs.srv import DisableRobot, EnableRobot
import rclpy
from rclpy.client import Client
from rclpy.node import Node
from rclpy.qos import DurabilityPolicy, QoSProfile, ReliabilityPolicy


class RobotPowerClient(Node):
    """查询 MG400 当前模式，并按需执行使能或失能."""

    # 这些模式都说明机器人已经处于伺服使能状态，不能重复发送 EnableRobot。
    _ENABLED_MODES = {
        RobotModeMsg.ENABLE,
        RobotModeMsg.RUNNING,
        RobotModeMsg.PAUSE,
        RobotModeMsg.JOG,
    }

    def __init__(self, operation: str) -> None:
        super().__init__(f'object_finder_{operation}_robot')
        self.declare_parameter('total_timeout_sec', 30.0)
        self.declare_parameter('call_timeout_sec', 3.0)
        self.declare_parameter('retry_interval_sec', 0.5)

        self._total_timeout = float(
            self.get_parameter('total_timeout_sec').value
        )
        self._call_timeout = float(
            self.get_parameter('call_timeout_sec').value
        )
        self._retry_interval = float(
            self.get_parameter('retry_interval_sec').value
        )
        self._validate_parameters()

        self._latest_mode: Optional[int] = None
        self._latest_mode_time = 0.0
        # MG400 驱动以 BEST_EFFORT 发布实时状态，订阅端必须使用兼容 QoS。
        mode_qos = QoSProfile(
            depth=1,
            reliability=ReliabilityPolicy.BEST_EFFORT,
            durability=DurabilityPolicy.VOLATILE,
        )
        self._mode_subscription = self.create_subscription(
            RobotModeMsg,
            '/mg400/robot_mode',
            self._on_mode,
            mode_qos,
        )
        self._enable_client = self.create_client(
            EnableRobot,
            '/mg400/enable_robot',
        )
        self._disable_client = self.create_client(
            DisableRobot,
            '/mg400/disable_robot',
        )

    def _validate_parameters(self) -> None:
        for name, value in (
            ('total_timeout_sec', self._total_timeout),
            ('call_timeout_sec', self._call_timeout),
            ('retry_interval_sec', self._retry_interval),
        ):
            if not math.isfinite(value) or value <= 0.0:
                raise ValueError(f'{name} 必须是大于 0 的有限数值。')

    def _call(
        self,
        client: Client,
        request: Any,
        deadline: float,
    ) -> Tuple[Optional[Any], str]:
        """在总截止时间内调用一个服务，并返回响应或错误说明."""
        remaining = deadline - time.monotonic()
        if remaining <= 0.0:
            return None, '等待服务超时'
        if not client.wait_for_service(timeout_sec=min(1.0, remaining)):
            return None, f'服务 {client.srv_name} 尚未就绪'

        future = client.call_async(request)
        rclpy.spin_until_future_complete(
            self,
            future,
            timeout_sec=min(self._call_timeout, remaining),
        )
        if not future.done():
            future.cancel()
            return None, f'服务 {client.srv_name} 响应超时'
        if future.exception() is not None:
            return None, f'服务 {client.srv_name} 异常：{future.exception()}'
        return future.result(), ''

    def _on_mode(self, message: RobotModeMsg) -> None:
        """保存 MG400 实时状态话题中的最新模式."""
        self._latest_mode = int(message.robot_mode)
        self._latest_mode_time = time.monotonic()

    def _get_mode(self, deadline: float) -> Tuple[Optional[int], str]:
        """等待一个新鲜的实时状态，避免调用可能阻塞的 Dashboard 查询."""
        wait_deadline = min(deadline, time.monotonic() + 1.0)
        while rclpy.ok() and time.monotonic() < wait_deadline:
            now = time.monotonic()
            if (
                self._latest_mode is not None
                and now - self._latest_mode_time <= 1.0
            ):
                return self._latest_mode, ''
            rclpy.spin_once(
                self,
                timeout_sec=min(0.2, wait_deadline - now),
            )
        return None, '尚未收到 /mg400/robot_mode 实时状态'

    def _pause_before_retry(self, deadline: float) -> None:
        remaining = deadline - time.monotonic()
        if remaining > 0.0:
            time.sleep(min(self._retry_interval, remaining))

    def enable(self) -> bool:
        """仅在未使能时发送 EnableRobot，并确认最终状态."""
        deadline = time.monotonic() + self._total_timeout
        last_error = 'MG400 服务尚未就绪'

        while rclpy.ok() and time.monotonic() < deadline:
            mode, error = self._get_mode(deadline)
            if mode in self._ENABLED_MODES:
                self.get_logger().info(
                    f'MG400 已处于使能状态（robot_mode={mode}），无需重复使能。'
                )
                return True
            if mode == RobotModeMsg.ERROR:
                self.get_logger().error(
                    'MG400 当前处于 ERROR 状态，请先排除故障并清除报警。'
                )
                return False
            if mode != RobotModeMsg.DISABLED:
                last_error = error or f'等待可使能状态，当前 robot_mode={mode}'
                self._pause_before_retry(deadline)
                continue

            request = EnableRobot.Request()
            # NO_PARAM=0：当前没有末端负载参数，其他字段保持 0。
            request.num_of_params = EnableRobot.Request.NO_PARAM
            request.load = 0.0
            request.center_x = 0.0
            request.center_y = 0.0
            request.center_z = 0.0
            response, error = self._call(
                self._enable_client,
                request,
                deadline,
            )
            if response is not None and response.result:
                last_error = 'EnableRobot 已接受，等待状态变为 ENABLE'
            elif response is not None:
                last_error = (
                    '控制器拒绝使能，'
                    f'error_id={response.error_id}'
                )
            else:
                last_error = error
            self._pause_before_retry(deadline)

        self.get_logger().error(
            f'{self._total_timeout:.1f} 秒内未能使能 MG400：{last_error}。'
        )
        return False

    def disable(self) -> bool:
        """发送 DisableRobot，并确认机器人最终进入 DISABLED 状态."""
        deadline = time.monotonic() + self._total_timeout
        last_error = 'MG400 服务尚未就绪'

        while rclpy.ok() and time.monotonic() < deadline:
            mode, error = self._get_mode(deadline)
            if mode == RobotModeMsg.DISABLED:
                self.get_logger().info('MG400 已失能（robot_mode=4）。')
                return True
            if mode is None:
                last_error = error
                self._pause_before_retry(deadline)
                continue

            response, error = self._call(
                self._disable_client,
                DisableRobot.Request(),
                deadline,
            )
            if response is not None and response.result:
                last_error = 'DisableRobot 已接受，等待状态变为 DISABLED'
            elif response is not None:
                last_error = (
                    '控制器拒绝失能，'
                    f'error_id={response.error_id}'
                )
            else:
                last_error = error
            self._pause_before_retry(deadline)

        self.get_logger().error(
            f'{self._total_timeout:.1f} 秒内未能确认 MG400 失能：{last_error}。'
        )
        return False


def _run(operation: str, args=None) -> int:
    rclpy.init(args=args)
    node = RobotPowerClient(operation)
    try:
        succeeded = node.enable() if operation == 'enable' else node.disable()
        return 0 if succeeded else 1
    except KeyboardInterrupt:
        return 130
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


def main(args=None) -> int:
    """运行一次状态感知的使能流程."""
    return _run('enable', args)


def disable_main(args=None) -> int:
    """运行一次状态感知的失能流程."""
    return _run('disable', args)


if __name__ == '__main__':
    raise SystemExit(main())
