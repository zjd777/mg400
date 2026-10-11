"""object_finder 的 setuptools/ament_python 安装规则和 ROS 2 可执行入口."""

from glob import glob
import os

from setuptools import find_packages, setup


package_name = 'object_finder'


setup(
    name=package_name,
    version='0.1.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        (
            'share/ament_index/resource_index/packages',
            ['resource/' + package_name],
        ),
        ('share/' + package_name, ['package.xml']),
        (
            os.path.join('share', package_name, 'launch'),
            glob('launch/*.launch.py'),
        ),
        (
            os.path.join('share', package_name, 'config'),
            glob('config/*.yaml'),
        ),
        (
            os.path.join('share', package_name, 'rviz'),
            glob('rviz/*.rviz'),
        ),
        ('share/' + package_name, ['README.md']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='zjd777',
    maintainer_email='3359712304@qq.com',
    description='Terminal-driven RGB-D object finding task for Dobot MG400.',
    license='Apache-2.0',
    tests_require=['pytest'],
    scripts=glob('scripts/*.sh'),
    entry_points={
        'console_scripts': [
            'object_finder_node = object_finder.task_node:main',
            'object_finder_command_bridge = '
            'object_finder.command_bridge:main',
            'object_finder_enable_robot = '
            'object_finder.enable_robot:main',
            'object_finder_disable_robot = '
            'object_finder.enable_robot:disable_main',
            'object_finder_text_input = object_finder.text_input:main',
            'object_finder_voice_monitor = object_finder.text_input:voice_main',
        ],
    },
)
