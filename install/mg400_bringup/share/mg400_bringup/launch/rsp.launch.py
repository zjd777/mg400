"""rsp.launch.py."""

# Copyright 2025 HarvestX Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from pathlib import Path
from typing import Dict
from typing import List
from typing import Tuple

from ament_index_python.packages import get_package_share_path
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import Command
from launch.substitutions import FindExecutable
from launch.substitutions import LaunchConfiguration
from launch.substitutions import PathJoinSubstitution
from launch.substitutions import TextSubstitution
from launch_ros.actions import Node


def load_robot_description(xacro_filepath: Path, xacro_options: List[Tuple] = None) -> Dict:
    """Load robot description."""
    if xacro_options is None:
        xacro_options = []

    if 'xacro' in str(xacro_filepath):
        params = []
        if xacro_options:
            for xacro_option in xacro_options:
                params.append(f' {xacro_option[0]}:=')
                params.append(xacro_option[1])
        command = [
            FindExecutable(name='xacro'),
            # PathJoinSubstitution([FindExecutable(name='xacro')]),
            ' ',
            str(xacro_filepath),
        ]
        robot_description_content = Command(command + params)
    else:
        try:
            with open(str(xacro_filepath), 'r', encoding='utf-8') as file:
                robot_description_content = file.read()
        except EnvironmentError:
            exit(1)
    return {'robot_description': robot_description_content}


def generate_launch_description():
    """Launch robot state publisher."""
    # Declare launch arguments
    ns_arg = DeclareLaunchArgument(
        'namespace',
        default_value=TextSubstitution(text=''),
        description='Set the robot resource namespace.',
    )
    workspace_visible_arg = DeclareLaunchArgument(
        'workspace_visible',
        default_value=TextSubstitution(text='False'),
        description='true : MG400 workspace is visible in rviz',
    )

    # Set launch configuration
    ns = LaunchConfiguration('namespace')
    workspace_visible = LaunchConfiguration('workspace_visible')

    # Load URDF using xacro
    xacro_filepath_ = get_package_share_path('mg400_description') / 'urdf' / 'mg400.urdf.xacro'
    robot_description = load_robot_description(
        xacro_filepath=xacro_filepath_,
        xacro_options=[
            ('workspace_visible', workspace_visible),
        ],
    )

    # Create node
    rsp_node = Node(
        package='robot_state_publisher',
        executable='robot_state_publisher',
        namespace=ns,
        output='log',
        parameters=[robot_description],
    )

    # Create launch description
    ld = LaunchDescription()
    # Add arguments
    ld.add_action(ns_arg)
    ld.add_action(workspace_visible_arg)
    # Add nodes
    ld.add_action(rsp_node)

    return ld
