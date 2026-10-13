"""机械臂安全校验：检查 MG400 工作半径、关节限制、工作区和运动参数."""

from dataclasses import dataclass
import math
from typing import Sequence

from .models import SafetyError


# 官方 MG400 规格给出的最大工作半径为 440 mm。
MG400_MAXIMUM_REACH_M = 0.440

# 以下范围与当前 MG400_ROS2 驱动的运动学约束保持一致，并预留 1.05° 边界。
# 官方资料中的 J4 范围存在版本差异，自动控制采用当前驱动较保守的 ±160°。
_DEG_TO_RAD = math.pi / 180.0
_MG400_MARGIN = 1.05 * _DEG_TO_RAD
_MG400_JOINT_LIMITS = (
    (-160.0 * _DEG_TO_RAD, 160.0 * _DEG_TO_RAD),
    (-25.0 * _DEG_TO_RAD, 85.0 * _DEG_TO_RAD),
    (-25.0 * _DEG_TO_RAD, 105.0 * _DEG_TO_RAD),
    (-160.0 * _DEG_TO_RAD, 160.0 * _DEG_TO_RAD),
)


@dataclass(frozen=True)
class WorkspaceBounds:
    """MG400 origin 坐标系中的轴对齐允许区间，所有边界单位均为米。"""

    x_min: float
    x_max: float
    y_min: float
    y_max: float
    z_min: float
    z_max: float

    @classmethod
    def from_sequence(cls, values: Sequence[float]) -> 'WorkspaceBounds':
        """按 [xmin, xmax, ymin, ymax, zmin, zmax] 读取并校验工作区。

        返回不可变边界对象；元素数量、有限性或每轴大小关系非法时
        抛出 SafetyError，不代表工作区内的所有轨迹都已验证无碰撞。
        """
        if len(values) != 6:
            raise SafetyError('safety.workspace_bounds 必须包含 6 个数值。')
        bounds = cls(*[float(value) for value in values])
        if not all(math.isfinite(value) for value in values):
            raise SafetyError('工作空间边界包含非有限值。')
        if (
            bounds.x_min >= bounds.x_max
            or bounds.y_min >= bounds.y_max
            or bounds.z_min >= bounds.z_max
        ):
            raise SafetyError('工作空间边界的最小值必须小于最大值。')
        return bounds

    def validate(self, x: float, y: float, z: float, name: str) -> None:
        """检查米制点是否落在各轴闭区间内，name 用于标识报错对象。

        坐标非有限或越界时抛出 SafetyError；只检查给定点，不采样路径。
        """
        # 这是用户确认过的保守工作区，不等同于机械臂理论最大可达空间。
        if not all(math.isfinite(value) for value in (x, y, z)):
            raise SafetyError(f'{name}包含非有限坐标。')
        if not (
            self.x_min <= x <= self.x_max
            and self.y_min <= y <= self.y_max
            and self.z_min <= z <= self.z_max
        ):
            raise SafetyError(
                f'{name}超出安全工作空间：'
                f'({x:.4f}, {y:.4f}, {z:.4f}) m。'
            )


def validate_mg400_radial_reach(x: float, y: float) -> None:
    """检查 origin 坐标系米制 XY 的水平半径是否超过 0.440 m。

    非有限值或超半径时抛出 SafetyError；这是必要范围检查，不能
    单独证明某个高度、末端朝向或整段轨迹在运动学上可达。
    """
    if not all(math.isfinite(value) for value in (x, y)):
        raise SafetyError('目标水平坐标包含非有限数值。')
    radius = math.hypot(x, y)
    if radius > MG400_MAXIMUM_REACH_M:
        raise SafetyError(
            '目标超出 MG400 官方 0.440 m 工作半径：'
            f'当前水平半径为 {radius:.4f} m。'
        )


def validate_height_parameters(
    final_standoff_m: float,
    approach_height_m: float,
) -> None:
    """检查相对物体表面的米制高度：最终悬停为正，接近高度更高。

    不满足大小关系时抛出 SafetyError；目标基坐标中的绝对高度还需
    由 WorkspaceBounds.validate 检查。本函数未单独检查 NaN/Inf，
    完整的参数校验还需保证数值有限。
    """
    if final_standoff_m <= 0.0:
        raise SafetyError('motion.final_standoff_m 必须大于 0。')
    if approach_height_m <= final_standoff_m:
        raise SafetyError(
            'motion.approach_height_m 必须大于最终悬停高度。'
        )


def validate_mg400_joint_angles(values: Sequence[float]) -> None:
    """检查 Home 的四个关节角（弧度），包括 J3-J2 平行连杆约束。

    J4 先归一到 [-π, π]，关节上下限留出 1.05° 余量；J3<=0 时收紧
    J2 下限。数量、有限性或约束不合格时抛出 SafetyError。
    """
    if len(values) != 4:
        raise SafetyError('home.joint_angles_rad 必须包含 4 个关节角。')
    angles = tuple(float(value) for value in values)
    if not all(math.isfinite(value) for value in angles):
        raise SafetyError('home.joint_angles_rad 包含非有限关节角。')

    j1, j2, j3, j4 = angles
    j4 = math.atan2(math.sin(j4), math.cos(j4))
    normalized = (j1, j2, j3, j4)
    limits = list(_MG400_JOINT_LIMITS)
    # 驱动在 J3<=0 时会收紧 J2 下限，以避免机械结构自碰撞。
    if j3 <= 0.0:
        limits[1] = (-19.0 * _DEG_TO_RAD, limits[1][1])

    names = ('J1', 'J2', 'J3', 'J4')
    for name, value, (minimum, maximum) in zip(
        names,
        normalized,
        limits,
    ):
        if not minimum + _MG400_MARGIN <= value <= maximum - _MG400_MARGIN:
            raise SafetyError(f'Home {name} 超出 MG400 安全关节范围。')

    # MG400 是平行连杆结构，除单关节范围外还必须检查 J3-J2。
    linkage_angle = j3 - j2
    linkage_minimum = -60.0 * _DEG_TO_RAD + _MG400_MARGIN
    linkage_maximum = 60.0 * _DEG_TO_RAD - _MG400_MARGIN
    if not linkage_minimum <= linkage_angle <= linkage_maximum:
        raise SafetyError('Home J3-J2 超出 MG400 连杆约束。')


def validate_percentage(
    value: int,
    name: str,
    allow_zero: bool = False,
) -> None:
    """检查驱动百分比参数是否处于 1~100，allow_zero 时允许 0~100。

    name 用于指出具体配置项；超出范围时抛出 SafetyError。参数已由
    ROS 整数参数读出，此处仅判断范围，不进行整数类型转换。
    """
    # 速度/加速度下限为 1；只有连续轨迹参数 CP 允许设置为 0。
    minimum = 0 if allow_zero else 1
    if not minimum <= value <= 100:
        raise SafetyError(f'{name} 必须位于 [{minimum}, 100]。')
