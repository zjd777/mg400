"""Launch the object finder with its safe default configuration."""

import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    """Create the task-node launch description."""
    # 参数集中放在 YAML 中，后续更换模型或实机参数时无需修改启动代码。
    default_config = os.path.join(
        get_package_share_directory('object_finder'),
        'config',
        'object_finder.yaml',
    )
    config_argument = DeclareLaunchArgument(
        'config_file',
        default_value=default_config,
        description='Object finder parameter file.',
    )
    node = Node(
        package='object_finder',
        executable='object_finder_node',
        name='object_finder',
        output='screen',
        emulate_tty=True,
        parameters=[LaunchConfiguration('config_file')],
    )
    return LaunchDescription([config_argument, node])
