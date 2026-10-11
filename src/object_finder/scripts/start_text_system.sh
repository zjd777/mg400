#!/usr/bin/env bash

# 本脚本一键启动 MG400 驱动、D435、TF、RViz 和文字找物终端。
# ROS 2 的 setup.bash 会读取若干可能尚未定义的环境变量，因此先不开启 -u。
set -Eeo pipefail

# 可通过同名环境变量覆盖工作空间、机械臂 IP 和后台 launch 日志位置。
object_finder_ws="${OBJECT_FINDER_WS:-/root/ros2_ws}"
mg400_ip="${MG400_IP_ADDRESS:-192.168.1.6}"
launch_log="${OBJECT_FINDER_LAUNCH_LOG:-/tmp/object_finder_text_launch.log}"

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

echo "正在启动文字找物系统，MG400 IP=${mg400_ip}"
echo "启动日志：${launch_log}"

# launch 在后台管理硬件和 RViz，当前终端留给文字输入客户端。
(
    # Bash 会让普通后台任务忽略 SIGINT；这里恢复信号，退出时 ROS launch
    # 才能按 Ctrl+C 的正常流程关闭并回收相机、机械臂和可视化节点。
    trap - INT TERM
    exec ros2 launch object_finder object_finder_text.launch.py \
        start_robot_driver:=true \
        robot_ip:="${mg400_ip}" \
        start_camera:=true \
        start_camera_tf:=true \
        start_robot_state_publisher:=true \
        start_rviz:=true \
        "$@"
) >"${launch_log}" 2>&1 &
application_pid=$!

cleanup() {
    if kill -0 "${application_pid}" 2>/dev/null; then
        kill -INT "${application_pid}" 2>/dev/null || true
        wait "${application_pid}" 2>/dev/null || true
    fi
}
trap cleanup EXIT

# 提前发现 IP、驱动或 launch 配置错误，避免给出假的输入提示。
for _ in {1..12}; do
    if ! kill -0 "${application_pid}" 2>/dev/null; then
        echo '错误：系统启动失败，最近日志如下：' >&2
        tail -n 60 "${launch_log}" >&2 || true
        exit 1
    fi
    sleep 0.25
done

echo '系统已启动。输入 quit 可退出并关闭整套节点。'
ros2 run object_finder object_finder_text_input
