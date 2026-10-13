"""公共数据模型：约束视觉、深度定位与任务控制模块之间传递的数据."""

from dataclasses import dataclass
from typing import Optional

import numpy as np


# 所有可预期的任务失败都继承该异常，任务节点捕获后会停止下发后续动作。
class ObjectFinderError(RuntimeError):
    """可预期任务失败的基类，供任务节点统一捕获和发布错误状态。"""


class VisionError(ObjectFinderError):
    """视觉配置、模型响应或候选选择无法支持可靠定位时抛出的异常。"""


class DepthError(ObjectFinderError):
    """深度图、相机内参或区域采样无法产生可靠三维点时抛出的异常。"""


class SafetyError(ObjectFinderError):
    """工作区、运动参数或联锁条件未通过安全校验时抛出的异常。"""


class MotionError(ObjectFinderError):
    """MG400 动作被拒绝、执行失败、被取消或等待超时时抛出的异常。"""


@dataclass(frozen=True)
class BoundingBox:
    """图像轴对齐框：[左、上、右、下]，normalized 标记是否为归一化坐标。

    normalized=True 时通常使用 [0, 1]，否则单位为像素；此类不在
    构造时校验，使用 to_pixels 时才转换、裁剪并检查有效尺寸。
    """

    x_min: float
    y_min: float
    x_max: float
    y_max: float
    normalized: bool = False

    def to_pixels(self, width: int, height: int) -> 'BoundingBox':
        """按图像宽高把框转为像素坐标，并裁剪到 [0, width/height]。

        返回新的 normalized=False 框，边界仍允许浮点像素坐标；
        尺寸非正、坐标非有限或裁剪后宽/高小于 2 像素时失败。
        """
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
    """视觉候选：标签、[0, 1] 置信度、目标框及可选同尺寸前景掩码。

    数据类本身不校验字段，视觉解析器和候选选择器负责检查；mask
    是图像空间数据，深度定位会将非零值当作允许参与采样的像素。
    """

    # LLM/视觉模型只允许产生图像空间结果，不能在这里携带机械臂坐标。
    label: str
    confidence: float
    bounding_box: BoundingBox
    mask: Optional[np.ndarray] = None


@dataclass(frozen=True)
class CameraIntrinsics:
    """针孔相机内参：fx/fy 为焦距，cx/cy 为主点，四者单位均为像素。"""

    fx: float
    fy: float
    cx: float
    cy: float

    @classmethod
    def from_camera_matrix(cls, values) -> 'CameraIntrinsics':
        """从 ROS CameraInfo.k 的 9 个行优先元素提取 fx、fy、cx、cy。

        K = [[fx, 0, cx], [0, fy, cy], [0, 0, 1]]；长度错误、提取值
        非有限或焦距非正时抛出 DepthError。这里不处理镜头畸变，
        调用方须提供与所用图像/对齐深度对应的内参。
        """
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
    """区域中值表面点：XYZ 为米，使用相机光学坐标轴（右、下、前）。

    pixel_u/v 为参与采样像素的中值位置，valid_ratio 为有效深度
    比例，sample_count 为有效深度数量；它们供任务状态和调试使用。
    """

    # x/y/z 属于相机光学坐标系；pixel_u/pixel_v 用于定位结果诊断。
    x: float
    y: float
    z: float
    pixel_u: float
    pixel_v: float
    valid_ratio: float
    sample_count: int
