#!/usr/bin/env bash

# 本脚本一键启动 MG400、D435、TF、RViz，并等待外部 ASR 发布识别文字。
# ROS 2 的 setup.bash 会读取若干可能尚未定义的环境变量，因此先不开启 -u。
set -Eeo pipefail

# OBJECT_FINDER_WS 和 MG400_IP_ADDRESS 可在运行脚本前临时覆盖。
object_finder_ws="${OBJECT_FINDER_WS:-/root/ros2_ws}"
mg400_ip="${MG400_IP_ADDRESS:-192.168.1.6}"

if [[ ! -f /opt/ros/humble/setup.bash ]]; then
    echo '错误：未找到 /opt/ros/humble/setup.bash。' >&2
    exit 1
fi
if [[ ! -f "${object_finder_ws}/install/setup.bash" ]]; then
    echo "错误：未找到 ${object_finder_ws}/install/setup.bash，请先 colcon build。" >&2
    exit 1
fi

source /opt/ros/humble/setup.bash
source "${object_finder_ws}/install/setup.bash"
set -u

if [[ -z "${DASHSCOPE_API_KEY:-}" ]]; then
    echo '警告：DASHSCOPE_API_KEY 未加载，图像和 RViz 仍可运行，但找物会失败。' >&2
fi

echo "正在启动语音找物系统，MG400 IP=${mg400_ip}"
echo '当前未集成 ASR；系统启动后将等待 /object_finder/asr_text。'

exec ros2 launch object_finder object_finder_voice.launch.py \
    start_robot_driver:=true \
    robot_ip:="${mg400_ip}" \
    start_camera:=true \
    start_camera_tf:=true \
    start_robot_state_publisher:=true \
    start_rviz:=true \
    "$@"
