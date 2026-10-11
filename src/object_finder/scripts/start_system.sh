#!/usr/bin/env bash

# 文字/语音共用的生命周期：启动硬件、使能、等待 Home、处理输入、退出失能。
# ROS 2 的 setup.bash 会读取若干可能尚未定义的环境变量，因此先不开启 -u。
set -Eeo pipefail

input_mode="${1:-text}"
if [[ $# -gt 0 ]]; then
    shift
fi
case "${input_mode}" in
    text) input_name='文字'; input_executable='object_finder_text_input' ;;
    voice) input_name='语音'; input_executable='object_finder_voice_monitor' ;;
    *) echo '错误：输入模式只能是 text 或 voice。' >&2; exit 1 ;;
esac

# 可通过同名环境变量覆盖工作空间、机械臂 IP 和后台 launch 日志位置。
object_finder_ws="${OBJECT_FINDER_WS:-/root/ros2_ws}"
mg400_ip="${MG400_IP_ADDRESS:-192.168.1.6}"
launch_log="${OBJECT_FINDER_LAUNCH_LOG:-/tmp/object_finder_${input_mode}_launch.log}"
# 默认在驱动启动后调用 MG400 使能服务；设为 false 可关闭自动使能。
auto_enable_robot="${OBJECT_FINDER_AUTO_ENABLE:-true}"
instance_lock="${OBJECT_FINDER_LOCK_FILE:-/tmp/object_finder_system.lock}"

if [[ ! -f /opt/ros/humble/setup.bash ]]; then
    echo '错误：未找到 /opt/ros/humble/setup.bash。' >&2
    exit 1
fi
if [[ ! -f "${object_finder_ws}/install/setup.bash" ]]; then
    echo "错误：未找到 ${object_finder_ws}/install/setup.bash，请先 colcon build。" >&2
    exit 1
fi
if ! command -v setsid >/dev/null || ! command -v flock >/dev/null; then
    echo '错误：启动脚本需要系统命令 setsid 和 flock。' >&2
    exit 1
fi

# 两种输入模式共用锁，防止文字/语音系统同时控制同一台机械臂。
exec 9>"${instance_lock}"
if ! flock -n 9; then
    echo '错误：已有一套找物系统正在运行，请先退出旧系统。' >&2
    exit 1
fi

source /opt/ros/humble/setup.bash
source "${object_finder_ws}/install/setup.bash"
set -u

# 兼容修复前启动、因而没有持有 flock 的旧进程；发现关键节点时拒绝再启动。
existing_nodes="$(timeout 3s ros2 node list 2>/dev/null || true)"
if grep -Eq \
    '^/(object_finder|mg400/mg400_node|camera/camera)$' \
    <<<"${existing_nodes}"; then
    echo '错误：检测到上一套系统的节点仍在运行，请先在旧终端输入 quit。' >&2
    exit 1
fi

if [[ -z "${DASHSCOPE_API_KEY:-}" ]]; then
    echo '警告：DASHSCOPE_API_KEY 未加载，图像和 RViz 仍可运行，但找物会失败。' >&2
fi

echo "正在启动${input_name}找物系统，MG400 IP=${mg400_ip}"
echo "启动日志：${launch_log}"

# launch 在独立进程组中管理硬件和 RViz，当前终端留给文字输入客户端。
# 这样终端信号不会抢先杀死驱动，cleanup 仍可先发送失能命令。
setsid ros2 launch object_finder "object_finder_${input_mode}.launch.py" \
    start_robot_driver:=true \
    robot_ip:="${mg400_ip}" \
    start_camera:=true \
    start_camera_tf:=true \
    start_robot_state_publisher:=true \
    start_rviz:=true \
    "$@" >"${launch_log}" 2>&1 &
application_pid=$!
application_pgid=$application_pid
enable_requested=false
cleanup_started=false

launch_group_running() {
    kill -0 -- "-${application_pgid}" 2>/dev/null
}

cleanup() {
    # 防止信号触发退出时重复执行清理流程。
    if [[ "${cleanup_started}" == 'true' ]]; then
        return
    fi
    cleanup_started=true
    trap - EXIT INT TERM HUP

    # 先通过状态感知客户端确认失能，再关闭提供服务的 MG400 驱动。
    if [[ "${enable_requested}" == 'true' ]] \
        && launch_group_running; then
        echo '正在取消 MG400 使能……'
        if ! timeout 35s ros2 run \
            object_finder object_finder_disable_robot; then
            echo '警告：自动取消 MG400 使能失败，请在控制器上确认状态。' >&2
        fi
    fi
    if launch_group_running; then
        # SIGINT 让 ros2 launch 按正常 Ctrl+C 路径回收所有子节点。
        kill -INT -- "-${application_pgid}" 2>/dev/null || true
        for _ in {1..50}; do
            if ! launch_group_running; then
                break
            fi
            sleep 0.1
        done
    fi
    if launch_group_running; then
        kill -TERM -- "-${application_pgid}" 2>/dev/null || true
        for _ in {1..30}; do
            if ! launch_group_running; then
                break
            fi
            sleep 0.1
        done
    fi
    if launch_group_running; then
        echo '警告：部分节点未正常退出，正在强制清理本次启动的进程组。' >&2
        kill -KILL -- "-${application_pgid}" 2>/dev/null || true
    fi
    wait "${application_pid}" 2>/dev/null || true
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
trap 'exit 129' HUP

# 提前发现 IP、驱动或 launch 配置错误，避免给出假的输入提示。
for _ in {1..12}; do
    if ! kill -0 "${application_pid}" 2>/dev/null; then
        echo '错误：系统启动失败，最近日志如下：' >&2
        tail -n 200 "${launch_log}" >&2 || true
        exit 1
    fi
    sleep 0.25
done

# DDS 节点发现偶尔晚于节点自己的启动日志，因此同时检查 ROS 图和 IDLE 日志。
# 任一方式确认任务节点成功初始化后，才允许请求机械臂使能。
task_node_ready=false
for _ in {1..60}; do
    if ros2 node list 2>/dev/null | grep -Fxq '/object_finder' \
        || grep -Fq '[IDLE] 节点已启动' "${launch_log}"; then
        task_node_ready=true
        break
    fi
    if ! kill -0 "${application_pid}" 2>/dev/null; then
        break
    fi
    sleep 0.25
done
if [[ "${task_node_ready}" != 'true' ]]; then
    echo '错误：15 秒内未确认找物任务节点启动，相关日志如下：' >&2
    tail -n 200 "${launch_log}" >&2 || true
    exit 1
fi

case "${auto_enable_robot}" in
    true|TRUE|1)
        enable_requested=true
        echo '正在等待 MG400 连接并自动使能……'
        if ! ros2 run object_finder object_finder_enable_robot; then
            echo '错误：MG400 自动使能失败，系统将关闭。' >&2
            tail -n 200 "${launch_log}" >&2 || true
            exit 1
        fi
        ;;
    false|FALSE|0)
        echo '已通过 OBJECT_FINDER_AUTO_ENABLE=false 跳过 MG400 自动使能。'
        ;;
    *)
        echo '错误：OBJECT_FINDER_AUTO_ENABLE 只能是 true 或 false。' >&2
        exit 1
        ;;
esac

echo '节点已启动，正在等待 Home 就绪后开放输入。'
ros2 run object_finder "${input_executable}"
