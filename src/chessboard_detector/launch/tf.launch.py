from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node


def generate_launch_description():
    """Publish the provisional wrist-shell to camera transform."""
    arguments = [
        DeclareLaunchArgument('x', default_value='0.04'),
        DeclareLaunchArgument('y', default_value='0.0'),
        DeclareLaunchArgument('z', default_value='-0.02'),
        DeclareLaunchArgument('roll', default_value='0.0'),
        DeclareLaunchArgument('pitch', default_value='1.5707963'),
        DeclareLaunchArgument('yaw', default_value='3.1415926'),
        DeclareLaunchArgument(
            'parent_frame', default_value='mg400_link5'
        ),
        DeclareLaunchArgument('child_frame', default_value='camera_link'),
    ]

    # 默认值来自手工测量；完成外参标定和实物点位验证前禁止自动运动。
    publisher = Node(
        package='tf2_ros',
        executable='static_transform_publisher',
        name='wrist_shell_to_camera_link',
        arguments=[
            '--x', LaunchConfiguration('x'),
            '--y', LaunchConfiguration('y'),
            '--z', LaunchConfiguration('z'),
            '--roll', LaunchConfiguration('roll'),
            '--pitch', LaunchConfiguration('pitch'),
            '--yaw', LaunchConfiguration('yaw'),
            '--frame-id', LaunchConfiguration('parent_frame'),
            '--child-frame-id', LaunchConfiguration('child_frame'),
        ],
    )

    return LaunchDescription(arguments + [publisher])
