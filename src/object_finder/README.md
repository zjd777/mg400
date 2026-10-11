# object_finder

MG400 视觉找物任务的安全上层节点。当前版本提供接口骨架和完整的
RGB-D/TF/运动状态机，但默认禁止真机运动。

## 启动顺序

先启动彩色与深度对齐的 D435：

```bash
ros2 launch realsense2_camera rs_launch.py \
  enable_color:=true enable_depth:=true enable_sync:=true \
  align_depth.enable:=true
```

再启动 MG400、相机静态 TF 和本节点。开发阶段可以只启动本节点做
感知链路测试：

```bash
ros2 launch object_finder object_finder.launch.py
```

`vision.provider=disabled` 时所有任务都会安全失败。需要验证深度与 TF
链路时，可临时改成 `fixed_bbox`，节点会使用配置中的归一化边界框，
但由于 `motion.enabled=false`，只会输出规划坐标。

深度定位同时要求有效深度比例和最少有效样本数。使用目标掩码时，
有效比例只在掩码内部计算，避免小目标被掩码外背景像素错误稀释。

也可以通过 ROS 话题输入目标：

```bash
ros2 topic pub --once /object_finder/command std_msgs/msg/String \
  "{data: '红色杯子'}"
```

状态同时输出到终端和 `/object_finder/status`。定位后的基坐标点发布到
`/object_finder/target_point`。

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
