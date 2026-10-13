# object_finder 代码不足与改进建议

本次审查基于当前工作区源码，配合代码阅读指南使用。代码已补充中文函数说明和关键步骤注释；下面记录的问题尚未实施功能修复。

文档将可以从代码确认的缺陷与当前设计限制分开描述。优先级表示建议处理顺序，不代表已在真机上复现。

## 1. 优先关注的问题

| 优先级 | 问题 | 主要位置 | 类型 |
|---|---|---|---|
| 高 | Action 超时、取消无法保证物理停止 | `task_node._send_action`、MG400 Action 驱动 | 运动控制能力限制 |
| 高 | 只检查目标端点，缺少完整路径碰撞检查 | `task_node._execute_or_simulate_motion` | 安全能力限制 |
| 高 | 固定测试框可与真机运动同时开启 | `task_node._validate_configuration`、`vision.FixedBoundingBoxGrounder` | 缺少配置联锁 |
| 高 | README 对运动默认值的描述与 YAML 不一致 | 包 README、`config/object_finder.yaml` | 文档与配置不一致 |
| 中 | 模型推理较慢时，拍照时刻 TF 超出缓存 | `task_node._run_task`、`_transform_to_base` | 调用时序缺陷 |
| 中 | 深度有效比例高，仍可能定位到背景 | `depth.localize_depth_region` | 感知算法限制 |
| 中 | 图像新鲜度主要按接收时刻判断 | `task_node._submit_task` | 时间校验不足 |
| 中 | 输入端没有命令接收确认或恢复超时 | `text_input.publish_command`、`command_bridge._forward` | 通信协议不足 |
| 中 | 使能确认可能复用旧状态并重复请求 | `enable_robot.RobotPowerClient` | 状态确认缺陷 |
| 中 | 退出时工作线程可能尚未结束 | `task_node.stop_terminal_input` | 资源清理不足 |
| 中 | 自定义话题和命名空间没有统一传递 | `launch_support`、`start_system.sh` | 配置集成不足 |
| 低 | 高度参数及任务入口校验不够统一 | `safety.validate_height_parameters`、`task_node._submit_task` | 输入健壮性不足 |

## 2. 运动超时不等于机械臂停止

阅读位置：

- [task_node.py](src/object_finder/object_finder/task_node.py)：`_send_action()`、`_wait_for_future()`。
- [mov_j.cpp](src/MG400_ROS2/mg400_plugin/src/motion_api/mov_j.cpp)：`handle_cancel()`、`execute()`。
- [mov_l.cpp](src/MG400_ROS2/mg400_plugin/src/motion_api/mov_l.cpp)：`handle_cancel()`、`execute()`。

上层等待超时后会抛出异常，停止下发后续动作，但没有实现可靠的停止命令。驱动的 `handle_cancel()` 虽然返回 ACCEPT，实际停止运动仍是 TODO。

同时，上层配置的 Action 结果等待时间是 30 秒，MovJ/MovL 驱动内部却使用固定 10 秒超时，并要求在约 300 ms 内观察到 RUNNING。低速、长距离运动或反馈延迟可能触发驱动自己的失败分支；仅调大上层的等待时间无法延长底层窗口。

因此，日志中的 FAILED 或 Action ABORTED 不能直接证明机械臂已经停住。当前“失败后不再补发归位”的做法有必要保留。

建议：先在驱动层定义并验证停止协议、停止后状态确认、断连处理，再统一上下层超时策略。运动时长应结合距离、速度和控制器反馈评估。

驱动还有一个异常处理不足：发送 `commander_->movJ/movL` 抛异常时，仅记录日志后返回，没有在这个分支显式调用 `abort(result)` 并填充失败原因。不能据此断言 Action 一定永远不返回，但失败结果和诊断信息不明确，应显式结束 Action 并保留错误原因。

## 3. 工作区端点合法，不代表整条路径安全

阅读位置：[task_node.py](src/object_finder/object_finder/task_node.py) 的 `_execute_or_simulate_motion()`，以及 [safety.py](src/object_finder/object_finder/safety.py) 的 `WorkspaceBounds.validate()`。

当前检查的是法兰接近点、最终点和 Home。并没有检查从当前位置到 Home、从 Home 到接近点的完整关节轨迹，也没有相机、支架、线缆和障碍物的碰撞模型。

例如，Home 和目标都在允许工作区内，但 MovJ 的关节插补途中仍可能经过障碍物。0.440 m 半径检查也是必要条件，不能代替完整可达性和碰撞判断。

这是当前运动能力的边界，不能只靠打开 `clearance_verified` 就认定软件已经证明路径安全。

建议：先约束和验证有限数量的归位、接近路径，记录相机支架占用空间；后续再引入完整碰撞模型和路径检查。

## 4. fixed_bbox 与真机运动之间缺少代码联锁

阅读位置：[vision.py](src/object_finder/object_finder/vision.py) 的 `FixedBoundingBoxGrounder.locate()`，以及 [task_node.py](src/object_finder/object_finder/task_node.py) 的 `_validate_configuration()`。

固定框后端不识别目标文字，只返回配置的图像区域。包 README 说明固定框只能用于几何联调，不能与真机运动同时使用，但参数校验没有拒绝以下组合：

```yaml
vision.provider: fixed_bbox
motion.enabled: true
```

如果其他联锁和机器人状态均满足，程序仍会按这个固定区域的深度坐标生成运动，用户输入的物体描述并没有真正参与定位。

`test_home_lifecycle.py` 使用这个组合连接模拟硬件，说明该组合目前确实是代码允许的；测试中的模拟运动不能视为真机识别验证。

建议：生产入口拒绝固定框与真机运动同时开启；模拟测试通过明确的测试依赖注入提供固定结果，避免沿用生产运动开关。

## 5. 长时间视觉推理会使历史 TF 失效

阅读位置：[task_node.py](src/object_finder/object_finder/task_node.py) 的 `__init__()`、`_run_task()`、`_transform_to_base()`，以及 [vision.py](src/object_finder/object_finder/vision.py) 的 `_post_json()`。

调用顺序是：

```text
选定图像
    → 千问 HTTP 请求
    → 深度计算
    → 查询拍照时刻的相机到基座 TF
```

TF Buffer 只保存 15 秒动态历史；千问单次请求超时为 30 秒，可重试 2 次。在机械臂持续发布动态 TF 时，推理超过缓存窗口，拍照时刻的动态变换可能已经被清除。

后果是二维定位成功，却在 `_transform_to_base()` 因历史 TF 不可用而失败。当前异常处理会阻止目标运动，这属于可靠性问题。

本地隔离诊断确认：15 秒缓存中连续插入 100、116、117 秒的动态变换后，再查询 100 秒，得到 `ExtrapolationException`。是否在具体任务中发生，取决于动态 TF 更新和模型实际耗时。

建议：在调用模型之前按拍照时间查询并保存所需变换；推理后将选定图像的三维点应用到这份变换。增大缓存只能缓解，保存与观测绑定的变换更明确。

## 6. 有效深度不一定来自目标

阅读位置：[depth.py](src/object_finder/object_finder/depth.py) 的 `localize_depth_region()`，以及 [vision.py](src/object_finder/object_finder/vision.py) 的 `_parse_qwen_detections()`。

深度算法把目标框缩小后，筛选有效量程内的像素，再对 X、Y、Z 分别取中值。目前千问适配器只返回边界框，没有实际生成前景 mask，也没有做目标与背景深度分群。

当目标较小、目标框偏大、杯口中能看见背景，或透明物体深度缺失时，剩余有效像素可能大多数属于背景。

本地复现：10×10 深度图背景为 1 m，中心 2×2 目标为 0.5 m。默认每边收缩 20% 后，取样区域有 36 个有效像素，算法返回 `z=1.0 m`、有效比例 100%。这里的“100%”只表示深度数值满足范围，不表示它们属于目标。

XYZ 分别取中值还有一个边界：最终三维点不保证对应同一个实际像素。

建议：增加前景分割或深度聚类，检查深度分布的离散程度；对于混合深度和透明、反光物体明确拒绝。补充“背景占多数”的测试，比仅验证均匀深度更有意义。

## 7. 图像接收时间与采集时间没有充分区分

阅读位置：[task_node.py](src/object_finder/object_finder/task_node.py) 的 `FrameBundle`、`_submit_task()`、`_validate_frame_bundle()`。

`received_monotonic` 是本机收到同步消息的时间。任务入口计算帧龄时使用这个时间，而不是消息 `header.stamp`。

如果旧图像经过缓冲、延迟或回放才到达，接收帧龄仍可能很小。坐标系和分辨率检查也没有拒绝零时间戳；零时间戳用于 TF 查询时会退化为查询最新变换，失去“拍照时刻”的含义。

启动真机归位后的取图流程会检查新帧时间戳，这提供了部分保护；但系统 READY 之后每次提交的帧仍需要独立核验。

另外，模型推理后不会重新观察目标。机械臂仍在 Home 的检查只能证明机器人没有离开 Home，不能证明物体没有移动。这在项目限定的静态场景中是已知限制。

建议：同时检查本机接收时间和 ROS 采集时间，拒绝零时间戳，明确回放与仿真时间的规则；设置最大观测到执行延迟，必要时运动前二次确认目标。

## 8. 输入门控缺少命令接收确认

阅读位置：[text_input.py](src/object_finder/object_finder/text_input.py) 的 `publish_command()`，以及 [command_bridge.py](src/object_finder/object_finder/command_bridge.py) 的 `_forward()`。

终端和输入桥分别订阅 READY。终端发出消息后马上设置 `ready=False`，输入桥转发后也会关闭自己的门控。两者都依赖主节点以后再发布 READY 才恢复。

一种可能的时序是：

```text
终端先收到 READY=True
    → 用户提交，终端设置 ready=False
    → 输入桥还未收到 READY=True，忽略输入
    → 主节点没有收到任务，也不会产生新的完成/READY 消息
    → 终端可能持续等待
```

这是独立节点、独立话题之间的时序窗口，不应把可靠 QoS 理解为业务层确认。

注意：主节点的 WAITING_FRAME、STALE_FRAME 分支已经重发 READY，这两种拒绝并不是上述问题的证据。

建议：给命令增加 ID，并返回 accepted/rejected 确认；未收到确认时进行状态查询和超时恢复。恢复只应查询状态，避免无条件重发导致重复运动。

## 9. 使能确认可能复用旧状态，超时预算也不精确

阅读位置：[enable_robot.py](src/object_finder/object_finder/enable_robot.py) 的 `_get_mode()`、`_pause_before_retry()`、`enable()`、`_call()`。

`_get_mode()` 会直接返回 1 秒内缓存的机器人模式，`_pause_before_retry()` 只 sleep、不处理 ROS 回调。因此 EnableRobot 服务已经接受后，下一轮仍可能读取旧 DISABLED，再提交一次请求。

隔离模拟中，模式更新等待回调处理时，默认重试间隔导致使能请求在 100.0、100.5、101.0 秒提交三次。实际是否发生取决于服务期间的回调处理和状态反馈时序；不能假定每次都会重复。

建议：服务接受后等待“请求之后收到”的模式反馈，不再把调用前的 DISABLED 当作新的请求依据。

`_call()` 还有一个较小的问题：在等待服务可用前计算 `remaining`，等待服务后没有重新计算，再用旧预算等待响应，可能越过总 deadline。隔离模拟中截止时间为 100.5，实际等待到 100.9。

建议：每个阻塞步骤后重新计算剩余时间，统一使用总截止时间。

## 10. 退出流程没有确保所有工作线程结束

阅读位置：[task_node.py](src/object_finder/object_finder/task_node.py) 的 `stop_terminal_input()`、`main()`，以及 [vision.py](src/object_finder/object_finder/vision.py) 的 `_post_json()`、[start_system.sh](src/object_finder/scripts/start_system.sh) 的 `cleanup()`。

关闭时只为每个工作线程等待最多 1 秒；HTTP 请求可以等待 30 秒，并且重试过程不读取节点的关闭事件。

如果任务正在等待模型，主线程可能先销毁 ROS 节点，任务线程仍然继续处理返回结果或尝试调用发布接口。任务线程是 daemon，进程退出也可能直接终止它，不能理解为完成了有序收尾。

脚本层还存在退出时序窗口：`cleanup()` 先请求失能，再向 launch 进程组发送 SIGINT。在前一个阶段，任务节点尚未设置关闭事件，仍可能准备下一段动作。ENABLE 核验降低了风险，但核验状态与发送目标并不是原子操作。

建议：先通知任务执行器停止接收和提交新目标，再执行失能与进程回收；让视觉请求和重试识别关闭事件，并在请求返回后的每个关键阶段检查关闭状态。明确资源销毁必须等待哪些工作完成。

## 11. 自定义话题、命名空间不能只改一处

阅读位置：[launch_support.py](src/object_finder/object_finder/launch_support.py)、[start_system.sh](src/object_finder/scripts/start_system.sh)、[text_input.py](src/object_finder/object_finder/text_input.py)、[enable_robot.py](src/object_finder/object_finder/enable_robot.py)。

虽然 launch 支持 `input_topic` 和 `robot_namespace`，相关参数没有统一传给全部节点：

- 改 launch 的 `input_topic` 只改变输入桥；一键脚本启动的终端客户端仍使用默认输入话题。
- 改 YAML 的 `topics.command` 不会改变 launch 中桥接器硬编码的 `/object_finder/command`。
- 改 YAML 的 ready/status 话题，也不会自动改变终端客户端和桥接器的默认话题。
- 改驱动命名空间，不会自动改变使能客户端硬编码的 `/mg400/...` 接口。

后果可能是命令无接收者、输入端一直等待，或使能服务找不到。默认配置下可以运行，但参数化不完整。

建议：由同一个配置入口统一传递话题、状态接口和机器人命名空间，并增加一组非默认命名空间的集成测试。

## 12. 输入与参数校验还可以统一

阅读位置：[safety.py](src/object_finder/object_finder/safety.py) 的 `validate_height_parameters()`，以及 [task_node.py](src/object_finder/object_finder/task_node.py) 的 `_validate_configuration()`、`_submit_task()`。

高度校验没有检查 NaN/Inf。`validate_height_parameters(float('nan'), 0.15)` 可以通过；启动参数的有限性检查列表也没有单独覆盖两个高度参数。

真机流程后续的工作区检查会拒绝非有限目标，所以不能据此声称 NaN 会直接发送给机械臂；问题是配置错误没有尽早被指出，dry-run 也可能输出非有限规划值。

任务入口 `_submit_task()` 只统一检查非空；500 字长度限制和 UTF-8 检查分散在终端、输入桥和千问适配器。直接发布 command 或使用内置终端时，入口规则与经过桥接的输入并不完全一致。本地模拟确认 `_submit_task()` 可以把 501 字符传给任务线程；千问后端会进一步拒绝，但固定框后端没有这项约束。

建议：在主任务入口统一校验描述，并保留输入端校验用于及时提示；数值校验函数统一检查有限性、单位和范围。

## 13. 当前 README 与运行配置存在冲突

阅读位置：[包 README](src/object_finder/README.md)、[YAML](src/object_finder/config/object_finder.yaml)、[主节点默认参数](src/object_finder/object_finder/task_node.py)。

当前工作区 README 写着“默认禁止真机运动”、`motion.enabled=false`、`home.mode=joint`，但实际 YAML 为：

```yaml
motion.enabled: true
home.configured: true
home.mode: pose
home.pose: [0.350, 0.000, 0.150, 0.000]
safety.workspace_configured: true
safety.clearance_verified: true
```

这里需要区分：不加载 YAML 的节点默认参数确实禁止运动，但正常 launch 加载当前 YAML 后会允许运动。一键脚本还会主动启动驱动并请求使能。

README 还写机器人模式尚未纳入联锁，但当前主节点已经检查 `/mg400/robot_mode` 的新鲜度和空闲 ENABLE 状态。

包 README 在本次任务开始前已有工作区修改，本次保留该文件，在这里记录冲突。

建议：清楚分开“代码默认值”“当前现场 YAML”“实际启动时加载的配置”，让运行行为与说明一致。

## 14. 如何安排后续改进

建议按以下顺序逐项处理，并为对应触发场景补充测试：

1. 统一文档和实际运动配置，限制固定测试框的生产使用。
2. 完善驱动停止协议和运动超时，明确失败后的物理状态。
3. 将拍照图像、采集时间和 TF 变换绑定为同一份观测。
4. 加强深度前景判断、图像时效和输入接收确认。
5. 完善使能状态确认、参数传递和退出时的线程收尾。
6. 将主节点逐步拆分为观测管理、感知定位、运动执行三个职责，减少阅读和修改时的耦合。

现有测试主要覆盖几何计算、视觉响应解析、安全边界和模拟 Home 生命周期，不能证明真实相机标定、模型可靠性、运动停止或碰撞路径已经验收。

## 15. 本次验证结果与测试中的时序问题

本次补充了 10 个 Python 模块中全部 99 个函数/方法的说明，以及公共启动脚本的函数注释。语法检查、函数说明完整性检查和 `git diff --check` 均通过。

将模块、类和函数的 docstring 去除后，修改前后的 AST 完全一致；启动脚本去掉注释行后也一致，说明本次没有改动执行逻辑。

现有测试的运行结果：

| 检查 | 结果 |
|---|---|
| 当前注释版本首次全量测试 | 32 通过、1 失败 |
| 内存中加载 Git HEAD 原始模块进行全量测试 | 33 通过 |
| 当前注释版本再次全量测试 | 33 通过 |

首次失败的是 `test_missing_frame_rejection_does_not_lock_clients`。测试清空 `_latest_frame` 后预期收到 WAITING_FRAME，但日志显示实际进入了 LOCALIZING 和模拟运动。

该用例取消发布定时器和清空缓存，没有确保已进入 DDS/同步器的图像回调全部处理完；后续回调可以再次填充 `_latest_frame`。测试还取消了同一个用于发布机器人模式和 TF 的定时器，因此意外进入运动后又会因为状态过期失败。

这说明该测试存在时序不稳定的窗口，不能把单次通过或失败当作确定的门控结论。建议分别控制图像发布与机器人反馈，等待已排队的回调处理完成，再构造缺帧条件。

上述测试使用隔离 ROS domain 197 和模拟 Action；视觉请求由测试替代。本次没有启动真机，也没有调用真实千问接口。
