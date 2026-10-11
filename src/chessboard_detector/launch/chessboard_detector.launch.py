from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():

    chessboard_detector = Node(
        package='chessboard_detector',
        executable='chessboard_detector',
        name='chessboard_detector',
        output='screen',

        parameters=[{

            # D435 彩色图像。
            'image_topic':
                '/camera/camera/color/image_raw',

            # 与彩色图像对应的相机内参。
            'camera_info_topic':
                '/camera/camera/color/camera_info',

            # solvePnP 的结果属于彩色相机光学坐标系，不能写成 camera_link。
            'camera_frame':
                'camera_color_optical_frame',

            # 棋盘格坐标系名称。
            'board_frame':
                'chessboard',

            # 4x4 个方格对应 3x3 个内部角点。
            'pattern_width':
                3,

            'pattern_height':
                3,

            # 单个方格边长 25 mm，程序内部统一使用米。
            'square_size':
                0.025,
        }]
    )

    return LaunchDescription([
        chessboard_detector
    ])
