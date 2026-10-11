"""公共启动构造器：组装 MG400、D435、静态 TF、输入桥、任务节点和 RViz."""

import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription
from launch.conditions import IfCondition
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare


def build_application_launch(input_mode: str) -> LaunchDescription:
    """根据 text/voice 输入模式生成其余部分完全一致的启动描述."""
    if input_mode not in {'text', 'voice'}:
        raise ValueError(f'Unsupported input mode: {input_mode}')

    package_share = get_package_share_directory('object_finder')
    default_config = os.path.join(
        package_share,
        'config',
        'object_finder.yaml',
    )
    default_rviz = os.path.join(
        package_share,
        'rviz',
        'object_finder.rviz',
    )
    default_input_topic = (
        '/object_finder/text_input'
        if input_mode == 'text'
        else '/object_finder/asr_text'
    )
    source_name = '文字' if input_mode == 'text' else '语音识别'

    # 以下参数均可在 ros2 launch 命令末尾用 name:=value 覆盖。
    arguments = [
        # 不能叫 config_file：RealSense 启动文件也使用该名称。
        DeclareLaunchArgument(
            'object_finder_config',
            default_value=default_config,
            description='找物任务参数 YAML 路径。',
        ),
        DeclareLaunchArgument(
            'rviz_config',
            default_value=default_rviz,
            description='RViz 显示配置路径。',
        ),
        DeclareLaunchArgument(
            'start_rviz',
            default_value='true',
            description='是否启动 RViz。',
        ),
        DeclareLaunchArgument(
            'start_robot_state_publisher',
            default_value='true',
            description='是否发布 MG400 RobotModel 和关节 TF。',
        ),
        DeclareLaunchArgument(
            'start_robot_driver',
            default_value='false',
            description='是否连接并启动 MG400 真机驱动；默认关闭以防误连。',
        ),
        DeclareLaunchArgument(
            'robot_ip',
            default_value='192.168.1.6',
            description='MG400 控制器 IP。',
        ),
        DeclareLaunchArgument(
            'robot_namespace',
            default_value='mg400',
            description='MG400 ROS 命名空间。',
        ),
        DeclareLaunchArgument(
            'workspace_visible',
            default_value='false',
            description='是否在 RobotModel 中显示官方工作空间。',
        ),
        DeclareLaunchArgument(
            'start_camera',
            default_value='true',
            description='是否启动 D435 彩色、深度对齐和点云。',
        ),
        DeclareLaunchArgument(
            'start_camera_tf',
            default_value='true',
            description='是否发布腕部外壳到 camera_link 的标定静态 TF。',
        ),
        DeclareLaunchArgument(
            'input_topic',
            default_value=default_input_topic,
            description='文字或 ASR 识别结果的输入话题。',
        ),
        DeclareLaunchArgument(
            'initial_command',
            default_value='',
            description='启动后可选自动发送的一次性查找指令。',
        ),
        DeclareLaunchArgument(
            'initial_delay_sec',
            default_value='5.0',
            description='自动发送初始指令前的等待时间。',
        ),
    ]

    # 真机驱动默认不启动，避免 IP 未确认时连接错误设备。
    robot_driver = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution(
                [FindPackageShare('mg400_bringup'), 'launch', 'mg400.launch.py']
            )
        ),
        condition=IfCondition(LaunchConfiguration('start_robot_driver')),
        launch_arguments={
            'namespace': LaunchConfiguration('robot_namespace'),
            'ip_address': LaunchConfiguration('robot_ip'),
        }.items(),
    )
    robot_state_publisher = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution(
                [FindPackageShare('mg400_bringup'), 'launch', 'rsp.launch.py']
            )
        ),
        condition=IfCondition(
            LaunchConfiguration('start_robot_state_publisher')
        ),
        launch_arguments={
            'namespace': LaunchConfiguration('robot_namespace'),
            'workspace_visible': LaunchConfiguration('workspace_visible'),
        }.items(),
    )

    # 默认直接启动 D435，同时开启深度对齐和 RViz 点云。
    camera = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution(
                [FindPackageShare('realsense2_camera'), 'launch', 'rs_launch.py']
            )
        ),
        condition=IfCondition(LaunchConfiguration('start_camera')),
        launch_arguments={
            'enable_color': 'true',
            'enable_depth': 'true',
            'enable_sync': 'true',
            'align_depth.enable': 'true',
            'pointcloud.enable': 'true',
        }.items(),
    )
    camera_tf = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution(
                [FindPackageShare('chessboard_detector'), 'launch', 'tf.launch.py']
            )
        ),
        condition=IfCondition(LaunchConfiguration('start_camera_tf')),
    )

    # launch 子进程没有交互 stdin，因此统一从 ROS 话题接收输入。
    command_bridge = Node(
        package='object_finder',
        executable='object_finder_command_bridge',
        name=f'object_finder_{input_mode}_bridge',
        output='screen',
        parameters=[
            {
                'input_topic': LaunchConfiguration('input_topic'),
                'output_topic': '/object_finder/command',
                'source_name': source_name,
                'initial_command': LaunchConfiguration('initial_command'),
                'initial_delay_sec': LaunchConfiguration(
                    'initial_delay_sec'
                ),
            }
        ],
    )
    task_node = Node(
        package='object_finder',
        executable='object_finder_node',
        name='object_finder',
        output='screen',
        emulate_tty=True,
        parameters=[
            LaunchConfiguration('object_finder_config'),
            {'terminal.enabled': False},
        ],
    )
    rviz = Node(
        package='rviz2',
        executable='rviz2',
        name='object_finder_rviz',
        output='screen',
        condition=IfCondition(LaunchConfiguration('start_rviz')),
        arguments=['-d', LaunchConfiguration('rviz_config')],
    )

    return LaunchDescription(
        arguments
        + [
            robot_driver,
            robot_state_publisher,
            camera,
            camera_tf,
            command_bridge,
            task_node,
            rviz,
        ]
    )
