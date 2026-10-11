#!/usr/bin/env bash

# 文字入口：共用硬件启动、使能、启动归位和退出失能流程，归位后开放终端。
set -e
task_script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec bash "${task_script_dir}/start_system.sh" text "$@"
