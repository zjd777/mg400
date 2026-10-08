from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    return LaunchDescription([

        Node(
            package='tf2_ros',
            executable='static_transform_publisher',
            name='j4_to_camera_link',

            arguments=[
                '0.04',
                '0.0',
                '-0.02',

                '3.1415926',
                '0.0',
                '0.0',

                'mg400_link5',
                'camera_link'
            ]
        )

    ])