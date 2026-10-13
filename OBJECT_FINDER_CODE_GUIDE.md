# object_finder 代码阅读指南

本文档用于帮助第一次阅读 `object_finder` 功能包的开发者，沿着程序真实运行顺序理解代码入口、ROS 话题、函数调用、坐标转换、安全检查和机械臂运动流程。

源码中的函数已补充中文说明，包含作用、关键输入输出、单位/坐标系和失败处理。阅读中发现的问题与改进顺序见 [代码不足与改进建议](OBJECT_FINDER_CODE_REVIEW.md)。定位代码时优先搜索本文的函数名，新增注释后行号会变化。

## 1. 系统目标

`object_finder` 完成的主要任务是：

1. 接收用户输入的目标描述，例如“红色杯子”。
2. 使用彩色图像和千问视觉模型定位目标二维边界框。
3. 从对齐深度图中计算目标表面的相机三维坐标。
4. 通过 TF 将目标点转换到 `mg400_origin_link`。
5. 检查 Home、工作空间、机械臂状态和运动参数。
6. 控制 MG400 接近目标、下降、抬升并返回 Home。
7. 返回 Home 并获得稳定的新图像后，重新开放输入。

整体数据流如下：

```text
终端文字 / 外部 ASR
        ↓
/object_finder/text_input 或 /object_finder/asr_text
        ↓
command_bridge.py
        ↓ /object_finder/command
task_node.py
        ├─ vision.py：文字 + 彩色图 → 二维目标框
        ├─ depth.py：目标框 + 深度图 → 相机三维点
        ├─ TF：相机光学坐标 → mg400_origin_link
        ├─ safety.py：工作区、半径、高度和联锁检查
        └─ MG400 Action：MovJ → MovL → MovL → MovJ
```

## 2. 先区分两类“调用”

阅读本包时会遇到两类代码跳转。

### 2.1 普通 Python 调用

例如：

```text
task_node._run_task()
    → vision_grounder.locate()
    → depth_to_meters()
    → localize_depth_region()
    → task_node._transform_to_base()
```

这种调用可以在 IDE 中直接跳转到函数定义。

### 2.2 ROS 通信跳转

例如：

```text
text_input.py 发布 /object_finder/text_input
    ↓ ROS 话题
command_bridge.py 接收
    ↓ 再发布 /object_finder/command
task_node.py 接收
```

发布器和订阅器之间没有直接 Python 调用。阅读时需要根据话题名称查找对应的 publisher 和 subscription。

本项目使用的 ROS 通信形式包括：

- Topic：命令、状态、图像、机器人模式和关节状态。
- Action：`MovJ`、`MovL`、`JointMovJ`。
- Service：机器人使能和失能。
- TF：相机、法兰和基座之间的坐标转换。

## 3. 第一阶段：从系统入口开始阅读

### 3.1 查看 ROS 可执行入口

先阅读：

```text
src/object_finder/setup.py
```

`entry_points` 定义了 `ros2 run` 命令与 Python 函数的对应关系：

```text
object_finder_node
    → object_finder.task_node:main

object_finder_command_bridge
    → object_finder.command_bridge:main

object_finder_enable_robot
    → object_finder.enable_robot:main

object_finder_disable_robot
    → object_finder.enable_robot:disable_main

object_finder_text_input
    → object_finder.text_input:main

object_finder_voice_monitor
    → object_finder.text_input:voice_main
```

看到下面的命令时：

```bash
ros2 run object_finder object_finder_node
```

就应该跳转到：

```python
object_finder/task_node.py:main()
```

### 3.2 文字和语音的一键入口

文字模式入口：

```text
src/object_finder/scripts/start_text_system.sh
```

它本身只负责调用：

```text
start_system.sh text
```

语音模式入口：

```text
src/object_finder/scripts/start_voice_system.sh
```

它调用：

```text
start_system.sh voice
```

两种模式共用相机、TF、视觉、深度、安全和运动代码，区别只在输入话题：

```text
text  → /object_finder/text_input
voice → /object_finder/asr_text
```

当前语音模式没有实现麦克风或 ASR，只接收外部模块发布的识别文字。

### 3.3 阅读公共启动脚本

接下来完整阅读：

```text
src/object_finder/scripts/start_system.sh
```

建议按以下顺序理解：

1. 读取 `text` 或 `voice` 模式。
2. 设置工作空间、MG400 IP、日志和锁文件。
3. 使用 `flock` 防止两套系统同时控制机械臂。
4. 加载 ROS 2 和工作空间环境。
5. 检查是否已有旧节点运行。
6. 通过 `ros2 launch` 启动整套节点。
7. 等待 `/object_finder` 主节点启动。
8. 调用 `object_finder_enable_robot` 使能 MG400。
9. 启动文字输入或语音状态监视客户端。
10. 退出时先请求机器人失能，再关闭 launch 进程组。

关键调用链：

```text
start_system.sh
    → ros2 launch object_finder object_finder_text.launch.py
    → ros2 run object_finder object_finder_enable_robot
    → ros2 run object_finder object_finder_text_input
```

正常退出流程：

```text
用户输入 quit 或按 Ctrl+C
    → 输入客户端退出
    → start_system.sh 的 EXIT trap 执行 cleanup()
    → object_finder_disable_robot
    → 向 ros2 launch 进程组发送 SIGINT
    → 必要时依次使用 SIGTERM 和 SIGKILL 清理本次进程组
```

## 4. 第二阶段：理解 launch 如何组装系统

三个 launch 文件是很薄的入口：

```text
src/object_finder/launch/object_finder.launch.py
src/object_finder/launch/object_finder_text.launch.py
src/object_finder/launch/object_finder_voice.launch.py
```

它们最终都调用：

```python
object_finder.launch_support.build_application_launch(...)
```

主要代码位于：

```text
src/object_finder/object_finder/launch_support.py
```

`build_application_launch()` 组装以下节点或子 launch：

```text
MG400 真机驱动
MG400 RobotStatePublisher
RealSense D435
mg400_link5 → camera_link 静态 TF
object_finder_command_bridge
object_finder_node
RViz
```

具体关系：

```text
object_finder_text.launch.py
    → build_application_launch('text')
    → input_topic=/object_finder/text_input

object_finder_voice.launch.py
    → build_application_launch('voice')
    → input_topic=/object_finder/asr_text
```

`command_bridge` 的输出始终是：

```text
/object_finder/command
```

因此任务主节点不需要知道输入来自文字还是语音。

## 5. 第三阶段：阅读参数文件

参数集中在：

```text
src/object_finder/config/object_finder.yaml
```

建议将它放在 `task_node.py` 旁边对照阅读。

主要参数组如下：

| 参数组 | 作用 |
|---|---|
| `topics.*` | 图像、指令、状态和机器人反馈话题 |
| `frames.*` | 基坐标系与法兰坐标系 |
| `sync.*` | RGB、深度和相机内参同步 |
| `tf.*` | TF 查询超时 |
| `depth.*` | 深度范围、采样比例和 ROI |
| `vision.*` | 视觉后端、置信度和千问参数 |
| `motion.*` | 运动开关、速度、加速度和目标高度 |
| `actions.*` | MG400 Action 名称 |
| `home.*` | Home 位、到位容差和启动归位 |
| `safety.*` | 工作区和实物间隙联锁 |

当前配置中：

```yaml
motion.enabled: true
vision.provider: qwen
home.mode: pose
home.pose: [0.350, 0.000, 0.150, 0.000]
```

这表示当前 YAML 允许真机运动。仅阅读代码没有风险，但运行一键脚本前必须确认现场安全、外参、路径和急停状态。

## 6. 第四阶段：理解输入链路

### 6.1 终端客户端

阅读：

```text
src/object_finder/object_finder/text_input.py
```

文字模式调用链：

```text
main()
    → _run_input('text')
    → TextInputNode()
    → wait_until_ready()
    → 读取 stdin
    → publish_command()
    → 发布 /object_finder/text_input
```

`wait_until_ready()` 会等待主任务节点发布：

```text
/object_finder/ready = true
```

机械臂没有完成启动归位之前，不会显示正常输入提示。

### 6.2 输入桥

阅读：

```text
src/object_finder/object_finder/command_bridge.py
```

调用链：

```text
_input_callback()
    → _forward()
    → 检查 ready
    → 去掉首尾空白
    → 检查空字符串、长度和 UTF-8
    → 发布 /object_finder/command
```

发布一条命令后，输入桥会立即把本地 `_ready` 设置为 `False`。只有主任务节点完成任务并再次发布 READY，输入桥才会接受下一条指令。

## 7. 第五阶段：分段阅读 task_node.py

核心文件是：

```text
src/object_finder/object_finder/task_node.py
```

文件较长，不建议第一次从第 1 行连续读到结尾。按照以下顺序阅读更容易理解。

### 7.1 数据结构和节点初始化

先看：

```text
FrameBundle
JointStateSnapshot
ObjectFinderNode.__init__()
```

`FrameBundle` 冻结同一次观测的：

```text
color + depth + camera_info
```

`ObjectFinderNode.__init__()` 完成：

1. 参数声明和校验。
2. 创建 TF Buffer 和 Listener。
3. 创建状态、ready、目标点和调试图发布器。
4. 订阅命令、关节状态和机器人模式。
5. 配置 RGB-D 同步器。
6. 创建视觉提供器。
7. 创建 MG400 Action 客户端。
8. 启动 `_prepare_startup()` 线程。

程序入口 `main()` 使用 `MultiThreadedExecutor(num_threads=4)`，因为节点需要同时处理相机、TF、机器人状态和 Action 回调。

### 7.2 RGB-D 同步

阅读：

```text
_configure_rgbd_sync()
_rgbd_callback()
```

同步器接收：

```text
/camera/camera/color/image_raw
/camera/camera/aligned_depth_to_color/image_raw
/camera/camera/color/camera_info
```

`_rgbd_callback()` 只保存最新的完整观测，不在相机回调中执行耗时的千问请求。

### 7.3 启动归位

阅读：

```text
_prepare_startup()
```

调用过程：

```text
等待机器人处于 ENABLE
    → _home_reached()
    → 如果不在 Home：
        _validate_action_servers()
        _send_home()
    → _wait_for_home()
    → 发布 HOME_REACHED
    → _refresh_home_observation()
    → 发布 READY
    → _set_input_ready(True)
```

`_refresh_home_observation()` 会：

1. 等待机械臂和相机稳定。
2. 清除运动期间缓存的旧图像。
3. 等待一组新的 RGB-D 数据。
4. 再次确认图像与 Home 位姿一致。

### 7.4 接收任务

阅读：

```text
_command_callback()
_submit_task()
```

ROS 消息调用链：

```text
/object_finder/command
    → _command_callback(message)
    → _submit_task(message.data)
```

`_submit_task()` 检查：

- 输入是否为空。
- 是否已有任务执行。
- 系统是否处于 READY。
- 是否已经收到 RGB-D 图像。
- 最近图像是否过期。

检查通过后，它会冻结当前 `FrameBundle`、关闭输入并创建独立任务线程。

### 7.5 单次任务的核心函数

重点精读：

```text
_run_task()
```

完整调用顺序：

```text
发布 LOCALIZING
    → ROS Image 转 OpenCV 图像
    → _validate_frame_bundle()
    → _validate_observation_pose()
    → vision_grounder.locate(color_bgr, query)
    → select_single_detection()
    → _publish_debug_detection()
    → depth_to_meters()
    → CameraIntrinsics.from_camera_matrix()
    → localize_depth_region()
    → 构造相机坐标系 PointStamped
    → _transform_to_base()
    → 发布 /object_finder/target_point
    → 发布 TARGET_READY
    → _execute_or_simulate_motion()
```

单次任务最终在 `_run_task()` 的 `finally` 中结束：

- 成功：重新开放输入。
- 运动前的感知失败，并且机械臂仍在 Home：允许重新输入。
- 已经开始运动后失败：锁住输入，不自动归位，等待人工检查。

## 8. 第六阶段：视觉模块

阅读：

```text
src/object_finder/object_finder/vision.py
```

推荐顺序：

1. `VisionGrounder`
2. `DisabledVisionGrounder`
3. `FixedBoundingBoxGrounder`
4. `QwenProviderConfig`
5. `QwenVisionGrounder.locate()`
6. `_make_qwen_payload()`
7. `_parse_qwen_detections()`
8. `select_single_detection()`

千问调用链：

```text
QwenVisionGrounder.locate(image, query)
    → 检查 query
    → 从 DASHSCOPE_API_KEY 读取密钥
    → _encode_jpeg_data_url()
    → _make_qwen_payload()
    → _post_json()
    → _extract_assistant_content()
    → _parse_qwen_detections()
    → 返回 Detection 列表
```

视觉模型只允许返回：

```text
label
confidence
bbox_2d
```

它不负责深度、三维坐标或机械臂动作。

`select_single_detection()` 会：

1. 验证候选类型和置信度。
2. 删除低置信度候选。
3. 没有候选时失败。
4. 多于一个候选时要求用户进一步说明。
5. 只有一个候选时继续任务。

## 9. 第七阶段：二维目标变成三维点

### 9.1 公共数据模型

先阅读：

```text
src/object_finder/object_finder/models.py
```

主要类型：

| 类型 | 含义 |
|---|---|
| `BoundingBox` | 像素或归一化目标框 |
| `Detection` | 视觉检测候选 |
| `CameraIntrinsics` | 相机内参 `fx/fy/cx/cy` |
| `LocalizedPoint` | 相机光学坐标系三维点 |

主要异常：

```text
ObjectFinderError
    ├─ VisionError
    ├─ DepthError
    ├─ SafetyError
    └─ MotionError
```

这些可预期异常会被 `_run_task()` 捕获，然后停止下发后续运动命令。

### 9.2 深度定位

阅读：

```text
src/object_finder/object_finder/depth.py
```

`depth_to_meters()` 将：

- `16UC1`/`MONO16` 按比例转换为米。
- `32FC1` 直接转换为浮点米。

`localize_depth_region()` 的流程：

```text
目标框转换成像素坐标
    → 向中心收缩 ROI
    → 过滤 NaN、过近和过远深度
    → 可选应用目标 mask
    → 检查有效比例和最少样本数
    → 将有效像素反投影到相机坐标系
    → 对 X、Y、Z 分别取中值
```

针孔相机反投影公式：

```text
X = (u - cx) × Z / fx
Y = (v - cy) × Z / fy
Z = 深度值
```

返回的 `LocalizedPoint` 属于相机光学坐标系，不能直接发送给机械臂。

## 10. 第八阶段：TF 坐标转换

在 `task_node.py` 中阅读：

```text
_transform_to_base()
```

它使用深度图消息的时间戳查询：

```text
camera_color_optical_frame → mg400_origin_link
```

必须使用拍照时刻的 TF，不能使用开始运动后的当前 TF。原因是相机安装在机械臂腕部，机械臂运动后相机位姿会改变。

预期 TF 链：

```text
mg400_origin_link
    → MG400 各连杆
    → mg400_link5
    → camera_link
    → camera_color_frame
    → camera_color_optical_frame
```

其中：

- MG400 连杆 TF 来自 RobotStatePublisher。
- `mg400_link5 → camera_link` 来自 `chessboard_detector/launch/tf.launch.py`。
- `camera_link → camera_color_optical_frame` 来自 RealSense 驱动。

## 11. 第九阶段：安全检查和运动状态机

### 11.1 安全工具

阅读：

```text
src/object_finder/object_finder/safety.py
```

主要检查：

- `WorkspaceBounds.validate()`：目标是否位于配置工作区。
- `validate_mg400_radial_reach()`：是否超过 0.440 m 水平半径。
- `validate_height_parameters()`：接近高度必须高于最终悬停高度。
- `validate_mg400_joint_angles()`：Home 关节角和连杆约束。
- `validate_percentage()`：速度、加速度和 CP 百分比。

### 11.2 运动入口

回到 `task_node.py` 阅读：

```text
_execute_or_simulate_motion()
```

目标高度计算：

```text
final_z    = 目标表面 z + motion.final_standoff_m
approach_z = 目标表面 z + motion.approach_height_m
```

当前配置对应：

```text
最终点：物体表面上方 0.05 m
接近点：物体表面上方 0.15 m
```

`radial` yaw 策略使用：

```python
atan2(y, x)
```

### 11.3 dry-run 分支

如果：

```yaml
motion.enabled: false
```

程序仍然会执行视觉、深度、TF 和工作区计算，但只发布规划结果，不创建运动目标。

### 11.4 真机运动顺序

真实运动调用顺序：

```text
_send_mov_j(approach_pose)
    ↓ 关节插补到目标上方安全点

_send_mov_l(target_pose)
    ↓ 垂直下降到物体表面上方 5 cm

发布 FOUND

_send_mov_l(approach_pose)
    ↓ 沿原直线路径垂直抬升

_send_home()
    ↓ MovJ 返回观察/Home 位置

_wait_for_home()
    ↓ 确认到位

_refresh_home_observation()
    ↓ 获取稳定的新 RGB-D 图像

发布 SUCCEEDED 和 READY
```

### 11.5 Action 的统一发送函数

继续阅读：

```text
_send_mov_j()
_send_mov_l()
_send_home()
_send_action()
```

所有运动最终都进入 `_send_action()`：

```text
确认机器人仍处于 ENABLE
    → 等待 Action Server
    → send_goal_async()
    → 检查目标是否被接受
    → get_result_async()
    → 检查 Action 状态
    → 检查驱动返回 result
    → _wait_for_robot_idle()
```

任意一段失败都会抛出 `MotionError`，不会继续发送下一段动作。

注意：当前底层 Action cancel 不能保证机械臂真正停止。因此 Action 超时后，上层只保证“不再发送后续动作”，不能把它等同于物理停机。

## 12. 完整状态机

正常状态变化：

```text
IDLE
    → WAITING_ENABLE
    → HOMING（如果启动时不在 Home）
    → HOME_REACHED
    → READY
    → LOCALIZING
    → TARGET_READY
    → MOVING_APPROACH
    → MOVING_TARGET
    → FOUND
    → RETREATING
    → RETURNING_HOME
    → SUCCEEDED
    → READY
```

主要失败分支：

```text
启动失败
    → STARTUP_FAILED
    → 不开放输入

视觉、深度、TF 或安全检查失败
    → FAILED
    → 如果没有开始运动且仍在 Home，可再次 READY

运动已经开始后失败
    → FAILED
    → 不自动返回 Home
    → 输入保持关闭
    → 人工检查后重启
```

## 13. 并发模型

系统中同时存在多种并发单位。

### 13.1 独立进程

launch 会启动多个 ROS 进程，例如：

```text
MG400 驱动
RealSense 驱动
command_bridge
object_finder 主节点
RViz
```

`start_system.sh` 又在当前终端单独启动 `text_input` 或 `voice_monitor`。

### 13.2 task_node 内部线程

`task_node.py` 内部包含：

- ROS `MultiThreadedExecutor`。
- 启动归位线程 `_startup_thread`。
- 单次任务线程 `_task_thread`。
- 直接运行主节点时可选的终端线程 `_terminal_thread`。

共享状态通过以下锁保护：

```text
_frame_lock
_joint_state_lock
_task_lock
```

这样千问 HTTP 请求和等待机械臂 Action 时，ROS Executor 仍能继续接收相机、TF 和机器人状态消息。

## 14. 跨包代码跳转

理解完 `object_finder` 后，再阅读外部依赖。

### 14.1 相机外参

```text
object_finder/launch_support.py
    → chessboard_detector/launch/tf.launch.py
    → static_transform_publisher
    → mg400_link5 → camera_link
```

当前默认外参仍是临时值，不能仅根据程序运行成功判断坐标准确。

### 14.2 MG400 Action

```text
task_node._send_action()
    → /mg400/mov_j 或 /mg400/mov_l
    → MG400_ROS2/mg400_plugin 对应 Action Server
    → command_queue
    → MG400 控制器
```

建议继续查看：

```text
src/MG400_ROS2/mg400_plugin/src/motion_api/mov_j.cpp
src/MG400_ROS2/mg400_plugin/src/motion_api/mov_l.cpp
src/MG400_ROS2/mg400_plugin/src/motion_api/joint_mov_j.cpp
src/MG400_ROS2/mg400_plugin/src/motion_api/command_queue.cpp
```

## 15. 测试文件如何配合阅读

代码测试位于：

```text
src/object_finder/test
```

建议对应阅读：

| 测试文件 | 对应代码 |
|---|---|
| `test_vision.py` | `vision.py`、`models.BoundingBox` |
| `test_depth.py` | `depth.py`、`CameraIntrinsics` |
| `test_safety.py` | `safety.py` |
| `test_home_lifecycle.py` | `task_node.py` 的启动、任务和失败状态机 |

`test_home_lifecycle.py` 最适合检查自己是否真正理解：

- 启动时是否需要归位。
- Home 前是否禁止输入。
- 连续任务是否每次都返回 Home。
- 感知失败是否允许重试。
- 运动失败是否保持锁定。
- 无图像时是否会错误锁死输入。

## 16. 推荐的两轮阅读路线

### 第一轮：只理解主流程

按以下顺序阅读：

1. `src/object_finder/README.md`
2. `src/object_finder/setup.py`
3. `src/object_finder/scripts/start_text_system.sh`
4. `src/object_finder/scripts/start_system.sh`
5. `src/object_finder/launch/object_finder_text.launch.py`
6. `src/object_finder/object_finder/launch_support.py`
7. `src/object_finder/config/object_finder.yaml`
8. `src/object_finder/object_finder/text_input.py`
9. `src/object_finder/object_finder/command_bridge.py`
10. `task_node.py` 中的 `ObjectFinderNode.__init__()`
11. `task_node.py` 中的 `_prepare_startup()`
12. `task_node.py` 中的 `_submit_task()`
13. `task_node.py` 中的 `_run_task()`
14. `vision.py`
15. `depth.py`
16. `task_node.py` 中的 `_execute_or_simulate_motion()`
17. `safety.py`
18. `task_node.py` 中的 `_send_action()`

第一轮结束后，应该能够回答：

- 输入如何进入主节点？
- 图像什么时候被冻结？
- 千问返回的结果是什么？
- 深度坐标如何计算？
- 为什么必须使用拍照时刻 TF？
- 哪些条件满足后才会运动？
- 一次任务从哪里开始，到哪里结束？

### 第二轮：理解异常和安全边界

第二轮重点阅读：

1. `_validate_configuration()`
2. `_validate_frame_bundle()`
3. `_validate_observation_pose()`
4. `_validate_real_motion_configuration()`
5. `_validate_home_configuration()`
6. `_wait_for_robot_idle()`
7. `_wait_for_future()`
8. `_run_task()` 的两个 `except` 和 `finally`
9. `test_home_lifecycle.py`

第二轮结束后，应该能够解释每一种失败发生后，机械臂是否运动、输入是否重新开放，以及为什么。

## 17. 一次任务的开始与结束

如果只跟踪“一次找物任务”，起点是：

```text
task_node._command_callback()
```

随后进入：

```text
_submit_task()
    → 新建 _task_thread
    → _run_task()
```

正常逻辑终点是：

```text
_execute_or_simulate_motion()
    → 返回 Home
    → 获得新的 Home 图像
    → 发布 SUCCEEDED
```

资源和状态收尾终点是：

```text
_run_task() 的 finally
    → _task_active=False
    → _set_input_ready(True)
    → 发布 READY
```

整个系统不会在这里退出，而是等待下一条命令。只有用户退出输入客户端或终止脚本后，`start_system.sh` 才会执行失能和进程清理。

