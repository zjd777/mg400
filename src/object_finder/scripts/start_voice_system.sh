#!/usr/bin/env bash

# 语音入口：与文字共用使能/归位/失能流程，Home 就绪后接收外部 ASR 文字。
# 当前尚无麦克风/ASR 实现；识别模块应订阅 /object_finder/ready 决定何时工作。
set -e
task_script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec bash "${task_script_dir}/start_system.sh" voice "$@"
