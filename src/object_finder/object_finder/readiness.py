"""输入就绪话题 QoS：后启动的文字客户端和语音桥也能收到最新状态."""

from rclpy.qos import DurabilityPolicy, QoSProfile, ReliabilityPolicy


def ready_qos() -> QoSProfile:
    """返回保存最新状态的可靠 QoS，供 READY/状态话题的发布与订阅复用。

    depth=1 只保留最后一条；TRANSIENT_LOCAL 让兼容且后启动的订阅端也能
    收到发布端保留的状态，RELIABLE 要求可靠传输。该配置只描述消息交付，
    不保证机器人当前仍处于该状态，实际就绪判断由任务节点负责。
    """
    return QoSProfile(
        depth=1,
        reliability=ReliabilityPolicy.RELIABLE,
        durability=DurabilityPolicy.TRANSIENT_LOCAL,
    )
