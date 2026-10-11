"""输入就绪话题 QoS：后启动的文字客户端和语音桥也能收到最新状态."""

from rclpy.qos import DurabilityPolicy, QoSProfile, ReliabilityPolicy


def ready_qos() -> QoSProfile:
    """Use reliable latched delivery for the latest input readiness state."""
    return QoSProfile(
        depth=1,
        reliability=ReliabilityPolicy.RELIABLE,
        durability=DurabilityPolicy.TRANSIENT_LOCAL,
    )
