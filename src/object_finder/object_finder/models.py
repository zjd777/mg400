"""Data contracts shared by vision, depth localization and task control."""

from dataclasses import dataclass
from typing import Optional

import numpy as np


# 所有可预期的任务失败都继承该异常，任务节点捕获后会停止下发后续动作。
class ObjectFinderError(RuntimeError):
    """Base class for expected task failures."""


class VisionError(ObjectFinderError):
    """Raised when the configured vision provider cannot locate a target."""


class DepthError(ObjectFinderError):
    """Raised when a reliable 3-D point cannot be recovered."""


class SafetyError(ObjectFinderError):
    """Raised when a requested motion has not passed safety validation."""


class MotionError(ObjectFinderError):
    """Raised when an MG400 action is rejected, aborted or times out."""


@dataclass(frozen=True)
class BoundingBox:
    """Axis-aligned bounding box, in pixels or normalized coordinates."""

    x_min: float
    y_min: float
    x_max: float
    y_max: float
    normalized: bool = False

    def to_pixels(self, width: int, height: int) -> 'BoundingBox':
        """Return a clipped pixel-space bounding box."""
        if width <= 0 or height <= 0:
            raise VisionError('图像尺寸无效。')

        # 视觉模型建议返回 0~1 归一化坐标，避免模型分辨率与相机分辨率耦合。
        if self.normalized:
            values = (
                self.x_min * width,
                self.y_min * height,
                self.x_max * width,
                self.y_max * height,
            )
        else:
            values = (self.x_min, self.y_min, self.x_max, self.y_max)

        x_min, y_min, x_max, y_max = values
        if not all(np.isfinite(values)):
            raise VisionError('视觉模型返回了非有限边界框。')

        x_min = max(0.0, min(float(width), x_min))
        x_max = max(0.0, min(float(width), x_max))
        y_min = max(0.0, min(float(height), y_min))
        y_max = max(0.0, min(float(height), y_max))

        if x_max - x_min < 2.0 or y_max - y_min < 2.0:
            raise VisionError('视觉模型返回的边界框为空或过小。')

        return BoundingBox(x_min, y_min, x_max, y_max, normalized=False)


@dataclass(frozen=True)
class Detection:
    """A semantically grounded object candidate."""

    # LLM/视觉模型只允许产生图像空间结果，不能在这里携带机械臂坐标。
    label: str
    confidence: float
    bounding_box: BoundingBox
    mask: Optional[np.ndarray] = None


@dataclass(frozen=True)
class CameraIntrinsics:
    """Pinhole camera intrinsics used for depth deprojection."""

    fx: float
    fy: float
    cx: float
    cy: float

    @classmethod
    def from_camera_matrix(cls, values) -> 'CameraIntrinsics':
        """Create intrinsics from a ROS CameraInfo K matrix."""
        if len(values) != 9:
            raise DepthError('CameraInfo.k 必须包含 9 个元素。')
        intrinsics = cls(
            fx=float(values[0]),
            fy=float(values[4]),
            cx=float(values[2]),
            cy=float(values[5]),
        )
        if not all(
            np.isfinite(value)
            for value in (
                intrinsics.fx,
                intrinsics.fy,
                intrinsics.cx,
                intrinsics.cy,
            )
        ):
            raise DepthError('相机内参包含非有限数值。')
        if intrinsics.fx <= 0.0 or intrinsics.fy <= 0.0:
            raise DepthError('相机焦距参数无效。')
        return intrinsics


@dataclass(frozen=True)
class LocalizedPoint:
    """A robust target surface estimate in an optical coordinate frame."""

    # x/y/z 属于相机光学坐标系；pixel_u/pixel_v 用于定位结果诊断。
    x: float
    y: float
    z: float
    pixel_u: float
    pixel_v: float
    valid_ratio: float
    sample_count: int
