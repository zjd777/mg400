from setuptools import setup

package_name = 'chessboard_detector'

setup(
    name=package_name,
    version='0.0.1',

    packages=[package_name],

    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name]
        ),
        (
            'share/' + package_name,
            ['package.xml']
        ),
        (
            'share/' + package_name + '/launch',
            ['launch/chessboard_detector.launch.py']
        ),
    ],

    install_requires=[
        'setuptools',
    ],

    zip_safe=True,

    description='Chessboard detector for ROS2 hand-eye calibration',

    entry_points={
        'console_scripts': [
            'chessboard_detector = '
            'chessboard_detector.node:main',
        ],
    },
)
