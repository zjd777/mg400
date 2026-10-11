# object_finder

<!-- 本文件说明 object_finder 功能包的架构、启动方法、安全条件和参数配置。 -->

MG400 视觉找物任务的上层原型节点。当前版本已经连通文字输入、千问
视觉定位、D435 深度、TF 坐标转换、运动安全门控和 MG400 Action 接口，
但尚未完成实机标定与整机安全验收，默认禁止真机运动。

> **安全警告：** 当前代码不能代替机械臂控制器急停、碰撞检测或人工监护。
> 在“待提供的信息”全部确认以前，不要把 `motion.enabled` 改为 `true`。

## 当前状态

| 功能 | 当前状态 |
|---|---|
| 终端文字找物 | 已实现 |
| D435 彩色、对齐深度和相机内参 | 已接入，等待实机话题核对 |
| 千问 VL 二维目标框 | 已实现接口，等待真实 API 和现场图片验收 |
| 深度反投影与 TF 基座坐标 | 已实现，外参仍是手工临时值 |
| MG400 接近、下降、抬升、返回 Home | 已实现代码，默认由安全联锁禁用 |
| 多个同类物体 | 当前拒绝任务并要求补充描述 |
| ASR 语音识别 | 未实现，只预留 `/object_finder/asr_text` |
| 语音播报 TTS | 未实现，当前只输出终端和状态话题 |
| LLM Tool Calling | 未实现；当前所有合法输入都按“找物体”处理 |
| 软件停止/急停 | 未实现，必须依赖控制器急停和人工监护 |
| 抓取 | 不在当前范围，当前只让法兰到物体表面上方 5 cm |

## 数据流与职责边界

```text
终端文字 / 未来 ASR 文字
            ↓
输入桥：校验字符串并发布 /object_finder/command
            ↓
千问 VL：目标描述 + 彩色图 → 二维目标框
            ↓
D435：目标框中央区域 → 物体可见表面三维点
            ↓
拍照时刻 TF：相机光学坐标 → mg400_origin_link
            ↓
工作区、Home、间隙和运动开关检查
            ↓
MG400：高处接近 → 垂直下降 → 报告找到 → 抬升 → 返回 Home
```

千问只负责二维目标定位，不能直接生成深度、机械臂坐标或运动命令。
机械臂坐标必须来自 D435 深度和 TF。当前没有独立文本 LLM，也没有
`find_object` Tool；只有以后增加“停止、回家、查询状态”等多种意图时，
才需要加入 Tool 声明、Tool Call 解析和 ROS 执行器。

## 编译与测试

```bash
cd /root/ros2_ws
source /opt/ros/humble/setup.bash
colcon build --packages-select object_finder --symlink-install
source /root/ros2_ws/install/setup.bash
```

当前不连接硬件的单元测试：

```bash
cd /root/ros2_ws
PYTHONPATH=/root/ros2_ws/src/object_finder \
  python3 -m pytest -q src/object_finder/test
```

单元测试覆盖目标框解析、Qwen 响应校验、深度反投影和部分安全边界，
尚未覆盖真实 Qwen API、D435、MG400 Action、TF 标定误差和碰撞路径。

## 启动方式

工程分为文字和语音两个启动入口。两者默认都会启动 D435、
MG400 RobotStatePublisher、相机静态 TF、任务节点和 RViz。
为避免在 IP 未确认时连接真机，MG400 驱动默认不启动。

`start_robot_driver` 与 `motion.enabled` 是两层不同开关：前者只决定是否
连接驱动，后者才决定找物节点能否下发动作。一键脚本会把
`start_robot_driver` 设为 `true`，但配置文件中的 `motion.enabled=false`
仍会阻止找物运动。

### 文字输入

```bash
ros2 launch object_finder object_finder_text.launch.py
```

再打开一个终端，启动可重复输入的文字客户端：

```bash
source /root/ros2_ws/install/setup.bash
ros2 run object_finder object_finder_text_input
```

也可以不启动文字客户端，直接发布一次指令：

```bash
ros2 topic pub --once /object_finder/text_input std_msgs/msg/String \
  "{data: '红色杯子'}"
```

### 语音输入

```bash
ros2 launch object_finder object_finder_voice.launch.py
```

当前尚未确定麦克风和 ASR 模型，因此该 launch 不会伪造语音识别结果。
以后的 ASR 节点只需将识别文字作为 `std_msgs/msg/String`
发布到 `/object_finder/asr_text`。在没有语音模块时可以这样模拟：

```bash
ros2 topic pub --once /object_finder/asr_text std_msgs/msg/String \
  "{data: '寻找绿色杯子'}"
```

两个输入桥都会校验文字，然后统一转发到
`/object_finder/command`；深度、TF、安全检查和运动逻辑不需要知道输入来源。

### 可选启动参数

确认机械臂 IP 后，可以让启动文件同时启动驱动：

```bash
ros2 launch object_finder object_finder_text.launch.py \
  start_robot_driver:=true robot_ip:=192.168.1.6
```

如果 D435、RobotStatePublisher 或静态 TF 已经由其他终端启动，
必须关闭重复节点：

```bash
ros2 launch object_finder object_finder_text.launch.py \
  start_camera:=false \
  start_robot_state_publisher:=false \
  start_camera_tf:=false
```

RViz 默认展示 MG400 模型与关节状态、完整 TF 树、D435 点云、
彩色图、对齐深度图、带检测框的目标图以及目标三维点。

### 一键启动脚本

文字脚本会同时启动机械臂驱动、D435、TF、RViz 和任务节点，
然后在当前终端进入文字输入：

```bash
/root/ros2_ws/src/object_finder/scripts/start_text_system.sh
```

语音脚本启动同一套硬件，并等待 ASR 向
`/object_finder/asr_text` 发布文字：

```bash
/root/ros2_ws/src/object_finder/scripts/start_voice_system.sh
```

默认机械臂 IP 为 `192.168.1.6`。临时修改可执行：

```bash
MG400_IP_ADDRESS=192.168.1.10 \
  /root/ros2_ws/src/object_finder/scripts/start_text_system.sh
```

脚本后面还可追加任意 launch 参数，后面的参数会覆盖脚本默认值。

文字脚本把后台 launch 输出写到：

```text
/tmp/object_finder_text_launch.log
```

遇到相机、驱动或 RViz 启动失败时应先查看该日志。

## 模型与主要参数

默认使用 `qwen3-vl-plus` 定位二维目标框，API Key 只从
`DASHSCOPE_API_KEY` 环境变量读取。不要把 Key 写入 YAML、源码或 Git。
启动节点前在同一个终端执行：

```bash
read -rsp '请输入新的 DASHSCOPE_API_KEY: ' DASHSCOPE_API_KEY
echo
export DASHSCOPE_API_KEY
```

默认 Base URL 是华北2（北京）共享接入点。Key 如果在新加坡、美国、
中国香港等其他地域创建，必须同时修改
`vision.qwen.base_url`，否则会返回鉴权失败。

未设置环境变量或 API 调用失败时，任务会安全失败，不会下发机械臂动作。
需要验证深度与 TF 链路时，可临时把 `vision.provider` 改成
`fixed_bbox`，节点会使用配置中的归一化边界框。由于
`motion.enabled=false`，默认只会输出规划坐标。

深度定位同时要求有效深度比例和最少有效样本数。使用目标掩码时，
有效比例只在掩码内部计算，避免小目标被掩码外背景像素错误稀释。

状态同时输出到终端和 `/object_finder/status`。定位后的基坐标点发布到
`/object_finder/target_point`，检测框图发布到
`/object_finder/debug_image`。

关键参数位于 `config/object_finder.yaml`：

| 参数 | 当前值 | 作用 |
|---|---:|---|
| `vision.provider` | `qwen` | `disabled`、`fixed_bbox` 或真实 `qwen` |
| `vision.minimum_confidence` | `0.60` | 过滤低置信度候选；模型置信度并非安全证明 |
| `depth.roi_shrink_ratio` | `0.20` | 缩小目标框采样区，减少背景深度 |
| `motion.final_standoff_m` | `0.05` | 法兰中心停在可见表面上方 5 cm |
| `motion.approach_height_m` | `0.15` | 垂直下降前的高处接近距离 |
| `motion.yaw_policy` | `radial` | 径向 yaw；需要固定 J4 时改为 `fixed` |
| `home.mode` | `joint` | 使用四个物理关节角定义观察/Home 位 |
| `safety.workspace_bounds` | 临时值 | 基座坐标系中的保守长方体工作区 |

`fixed_bbox` 只用于验证 RGB-D 和 TF 数据链，绝不能与
`motion.enabled=true` 一起用于真机运动。

## ROS 接口

| 方向 | 话题 | 类型 | 用途 |
|---|---|---|---|
| 输入 | `/object_finder/text_input` | `std_msgs/msg/String` | 终端文字模式 |
| 输入 | `/object_finder/asr_text` | `std_msgs/msg/String` | 未来 ASR 输出 |
| 内部 | `/object_finder/command` | `std_msgs/msg/String` | 统一找物指令 |
| 输出 | `/object_finder/status` | `std_msgs/msg/String` | 状态与终端播报文本 |
| 输出 | `/object_finder/target_point` | `geometry_msgs/msg/PointStamped` | 基座坐标系目标表面点 |
| 输出 | `/object_finder/debug_image` | `sensor_msgs/msg/Image` | 带目标框的调试图像 |
| 输入 | `/mg400/joint_states` | `sensor_msgs/msg/JointState` | 观察位校验和机器人模型 |

## 真机运动联锁

只有以下参数全部满足时才会发送 MG400 Action：

- `motion.enabled=true`
- `home.configured=true`
- `safety.workspace_configured=true`
- `safety.clearance_verified=true`

真机模式还会确认拍照时机械臂位于观察/Home 位置：

- `home.mode=joint` 时，按 `home.joint_names` 从
  `/mg400/joint_states` 提取四个物理关节并与 Home 比较。
- `home.mode=pose` 时，在拍照时间查询
  `mg400_origin_link -> mg400_end_effector_flange` TF 并与 Home 比较。

默认关节名映射为：

```text
物理 J1 -> mg400_j1
物理 J2 -> mg400_j2_1
物理 J3 -> mg400_j4_2
物理 J4 -> mg400_j5
```

这是当前 MG400 驱动发布的 URDF 关节映射；如果以后设置驱动 frame
prefix，需要同步修改 `home.joint_names`。

这些开关不能替代实际 TF、工作空间、支架间隙和急停测试。

## 推荐验收顺序

1. 保持 `motion.enabled=false`，确认 D435 三个话题及 RViz 图像正常。
2. 使用 `fixed_bbox` 检查深度单位、相机内参、TF 连通性和目标点方向。
3. 完成 `mg400_link5 -> camera_link` 六自由度外参标定，并用多个已知点
   测量基座坐标误差。六自由度外参描述两个刚体坐标系之间的三维位置和
   三维方向，与 MG400 本体只有四个运动关节并不矛盾。
4. 保持 `motion.enabled=false`，使用真实 Qwen 和现场图片验证目标框、
   多目标失败逻辑以及深度点是否落在物体顶面。
5. 测量并填写 Home、工作区、桌面高度、相机支架和法兰的碰撞余量。
6. 验证控制器急停、清错、使能和断网行为；测试时由人员看守急停。
7. 清空工作区，以最低速度进行单点运动测试，再逐步验证完整流程。

## 已知问题与限制

以下问题中，前六项是开启自动运动前必须解决或现场验证的事项：

1. `chessboard_detector/launch/tf.launch.py` 中外参仍是手工测量临时值：
   `(x=0.04, y=0, z=-0.02, roll=0, pitch=0, yaw=π)`，尤其相机方向尚未
   验证。错误外参会直接产生错误的机械臂目标坐标。
2. 代码假定相机固定在 `mg400_link5`。从当前 URDF 看，该 link 位于
   J4 对应旋转关节 `mg400_j5` 之前，符合“不随 J4 旋转”的设计；仍需
   对照真实支架确认安装位置确实属于这个刚体。
3. 当前安全检查只验证接近点、最终点和 Home 参数，不计算整条 MovJ
   路径与相机支架、桌面、物体或环境的碰撞。
4. 当前没有软件急停或可靠运动取消。MG400 ROS 2 驱动的 Action cancel
   仍标有“不能停止实际运动”的待办项。因此运动开始后即使上层超时，
   当前动作也可能继续，不能保证立即“保持不动”。
5. 节点尚未把 `/mg400/mg400_connected`、`/mg400/robot_mode` 和
   `/mg400/error_id` 纳入运动前联锁，目前主要依赖 Action 服务和目标结果。
6. Home、工作区和 `clearance_verified` 仍是未确认值；四个联锁开关不能
   在没有实物测量的情况下直接改成 `true`。
7. 视觉只使用单帧和二维框中央深度，没有分割、桌面平面剔除、运动前
   二次确认或动态目标跟踪。杯口、透明/反光物体和背景深度可能造成偏差。
8. Qwen 返回的 confidence 是模型自报值，不能视为标定过的检测概率。
   当前还需要用实际场景图片统计成功率和误检率。
9. 多个合格候选会直接失败；“最近、最大、最左”等选择策略尚未实现。
10. 输入桥不做自然语言意图分类。“停止”“回家”等文字目前也会被当作
    待寻找物体，尚未实现 Tool Calling 或专门命令协议。
11. 语音 launch 只有 ASR 文字入口，没有麦克风采集、ASR、TTS 和播报
    打断机制。
12. 当前测试均为无硬件测试，还没有记录整套 D435 + MG400 真机验收结果。

## 仓库维护注意事项

仓库的 `.gitignore` 已忽略 `build/`、`install/` 和 `log/`，但这些目录中
仍有大量历史生成文件已经进入 Git 索引；`.gitignore` 不会自动取消跟踪
已经提交过的文件。它们会造成提交体积过大，并把不同电脑的绝对路径和
构建缓存带入仓库。

确认远端只需要源码、并先检查当前提交后，可以单独执行以下操作取消
Git 跟踪。`--cached` 只修改 Git 索引，不删除本机文件：

```bash
cd /root/ros2_ws
git rm -r --cached build install log
git status --short
```

这一步没有由本工程脚本自动执行，避免在未确认提交范围时大规模改动索引。

## 待提供的信息

为了继续完善工程，请按下面模板提供。暂时没有的数据可以写“待测”，
API Key 不需要再次提供：

```text
[MG400]
控制器 IP：
硬件/固件版本：
ROS 驱动连接、使能和清错是否已实测：
固定观察/Home 关节角 [J1,J2,J3,J4]：
关节角单位（degree 或 radian）：
J4 是否允许自动改变 yaw：

[D435 与安装]
相机序列号（只有一台时可暂不填）：
实际彩色/深度分辨率和帧率：
相机是否确定刚性固定在 mg400_link5 对应外壳：
标定后 xyz（米）：
标定后 roll,pitch,yaw（弧度）：
外参验证误差或已知点测量结果：
相机和支架外形尺寸、最低点相对法兰的位置：

[工作环境]
基座坐标系中的桌面高度 z：
允许工作区 [x_min,x_max,y_min,y_max,z_min,z_max]（米）：
禁止进入区域或固定障碍物：
物体大致高度范围：
法兰到物体 5 cm 时，相机/支架是否确认不会碰撞：

[任务策略]
多个同类物体时的选择规则：
运动中失败时希望人工急停，还是接入控制器 EmergencyStop：
是否需要“停止、回家、查询状态、描述画面”等命令：
目标坐标允许误差、最长响应时间和期望成功率：

[模型与语音]
DashScope Key 所属地域（不要填写 Key 内容）：
准备采用的 Qwen 型号：
现场测试图片和常见目标名称：
未来麦克风型号：
期望的 ASR/TTS 模型或服务：
```

## MG400 官方运动范围

越疆官方产品参数给出的 MG400 最大工作半径为 `440 mm`，关节范围为：

| 关节 | 官方产品参数 |
|---|---:|
| J1 | ±160° |
| J2 | -25°～85° |
| J3 | -25°～105° |
| J4 | 不同官方版本存在差异，见下文 |

官方当前产品页面将 J4 标为 ±360°，较早的硬件手册 V1.1 标为
`-180°～180°`，而本项目中的 MG400 ROS 2 驱动运动学限制为 ±160°。
因此自动控制采用当前驱动的较保守范围并保留 1.05° 边界，不依据网页
参数放宽 J4。

代码还会强制拒绝水平半径大于 `0.440 m` 的目标。YAML 中的
`safety.workspace_bounds` 是针对实际桌面、支架和任务设置的更小安全
区域，不能因为目标位于官方最大工作半径内就认为路径安全。

官方资料：

- [越疆 MG400 中文产品参数](https://www.dobot.cn/products/desktop-four-axis/mg400.html?lang=cn)
- [MG400 Hardware User Guide V1.1](https://download.dobot.cc/mg400/en/Dobot%20MG400%20Hardware%20User%20Guide%20V1.1.pdf)

官方标注的 `300°/s` 是关节最大物理速度；本项目的
`motion.speed_j`、`motion.speed_l` 是 MG400 指令使用的百分比参数，
两者不能直接当成相同单位。
