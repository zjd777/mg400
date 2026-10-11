"""文字找物启动入口；机械臂、相机、TF 和 RViz 由公共构造器统一配置."""

from object_finder.launch_support import build_application_launch


def generate_launch_description():
    """生成使用文字输入话题的完整系统启动描述."""
    return build_application_launch('text')
