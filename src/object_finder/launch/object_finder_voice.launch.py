"""语音找物启动入口；接收外部 ASR 节点输出的文字，不直接采集音频."""

from object_finder.launch_support import build_application_launch


def generate_launch_description():
    """生成使用语音识别文字话题的完整系统启动描述."""
    return build_application_launch('voice')
