"""兼容旧命令的启动入口；实际复用完整的文字输入模式启动配置."""

from object_finder.launch_support import build_application_launch


def generate_launch_description():
    """按旧文件名启动文字找物模式."""
    return build_application_launch('text')
