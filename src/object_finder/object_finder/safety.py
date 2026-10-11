"""Deterministic motion safety checks."""

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
    """Axis-aligned allowed region expressed in the MG400 origin frame."""

    x_min: float
    x_max: float
    y_min: float
    y_max: float
    z_min: float
    z_max: float

    @classmethod
    def from_sequence(cls, values: Sequence[float]) -> 'WorkspaceBounds':
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
        """Raise if a point falls outside the configured safe region."""
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
    """Reject points beyond the official 440 mm working radius."""
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
    """Validate the relative target heights used by the motion sequence."""
    if final_standoff_m <= 0.0:
        raise SafetyError('motion.final_standoff_m 必须大于 0。')
    if approach_height_m <= final_standoff_m:
        raise SafetyError(
            'motion.approach_height_m 必须大于最终悬停高度。'
        )


def validate_mg400_joint_angles(values: Sequence[float]) -> None:
    """Validate a four-joint Home pose using the driver's constraints."""
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
    """Validate a percentage using the MG400 driver's actual limits."""
    # 速度/加速度下限为 1；只有连续轨迹参数 CP 允许设置为 0。
    minimum = 0 if allow_zero else 1
    if not minimum <= value <= 100:
        raise SafetyError(f'{name} 必须位于 [{minimum}, 100]。')
