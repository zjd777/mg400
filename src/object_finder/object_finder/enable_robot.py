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
        """建立机器人模式订阅和上/下使能服务客户端，读取等待与重试参数。

        operation 用于生成节点名，实际选择 enable/disable 的位置在 _run。
        初始化不直接改变伺服状态，必须随后调用 enable() 或 disable()。
        """
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
        self._latest_mode_time = 0.0  # 本地接收时刻，用来拒绝超过 1 秒的旧状态。
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
        """确保总等待、单次调用和重试间隔都是正有限数；否则抛出 ValueError。"""
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
        """等待服务可用并异步发出 request，返回 (响应, 错误文本)。

        client 决定调用哪个 ROS 服务，deadline 是 monotonic 的总截止时刻。
        失败时响应为 None；成功时错误文本为空。spin_until_future_complete
        在等待服务响应时继续处理本节点的机器人模式等回调。
        """
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
            # 取消本地 Future 的等待；这不会撤销控制器已经收到的服务请求。
            future.cancel()
            return None, f'服务 {client.srv_name} 响应超时'
        if future.exception() is not None:
            return None, f'服务 {client.srv_name} 异常：{future.exception()}'
        return future.result(), ''

    def _on_mode(self, message: RobotModeMsg) -> None:
        """缓存最新 robot_mode 与本地接收时刻，供使能/失能流程判断状态新鲜度。"""
        self._latest_mode = int(message.robot_mode)
        self._latest_mode_time = time.monotonic()

    def _get_mode(self, deadline: float) -> Tuple[Optional[int], str]:
        """返回 1 秒内接收到的模式，最多等待 1 秒且不超过总 deadline。

        返回 (模式, '') 或 (None, 错误文本)。已有新鲜缓存时立即返回，
        否则通过 spin_once 等待话题回调，不额外调用 Dashboard 查询服务。
        """
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
        """在剩余时间内暂停一个重试间隔，避免连续调用服务；此处不处理回调。"""
        remaining = deadline - time.monotonic()
        if remaining > 0.0:
            time.sleep(min(self._retry_interval, remaining))

    def enable(self) -> bool:
        """确认伺服使能，成功返回 True，ERROR 或总等待超时返回 False。

        已经处于 ENABLE/RUNNING/PAUSE/JOG 时直接成功；只有 DISABLED
        才发送 EnableRobot。服务接受请求后继续查模式，状态确认才算完成。
        """
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
        """请求失能并等待 DISABLED，确认成功返回 True，总等待超时返回 False。

        已经失能时不重复请求；没有新鲜模式时先重试等待，其他已知模式
        下调用 DisableRobot。服务返回成功仍需等待实际状态变为 DISABLED。
        """
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
    """创建一次性客户端并执行上/下使能，返回 0 成功、1 失败或 130 中断。"""
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
    """object_finder_enable_robot 入口：执行使能流程并返回退出码。"""
    return _run('enable', args)


def disable_main(args=None) -> int:
    """object_finder_disable_robot 入口：执行失能流程并返回退出码。"""
    return _run('disable', args)


if __name__ == '__main__':
    raise SystemExit(main())
