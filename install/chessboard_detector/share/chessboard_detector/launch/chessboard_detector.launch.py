from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    chessboard_detector = Node(
        package='chessboard_detector',
        executable='chessboard_detector',
        name='chessboard_detector',
        output='screen',

        parameters=[{

            # D435 RGB image
            'image_topic':
                '/camera/camera/color/image_raw',

            # D435 RGB camera info
            'camera_info_topic':
                '/camera/camera/color/camera_info',

            # D435 optical frame
            'camera_frame':
                # 'camera_color_optical_frame',
                'camera_link',

            # Chessboard TF frame
            'board_frame':
                'chessboard',

            # 4x4 squares -> 3x3 inner corners
            'pattern_width':
                3,

            'pattern_height':
                3,

            # 25 mm
            'square_size':
                0.025,
        }]
    )

    return LaunchDescription([
        chessboard_detector
    ])
