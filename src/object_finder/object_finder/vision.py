"""Pluggable visual grounding providers.

The provider contract deliberately returns only semantic image-space results.
It never returns robot coordinates or motion commands.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
import math
from typing import List, Sequence

import numpy as np

from .models import BoundingBox, Detection, VisionError


# 视觉接口只负责“文字 + 图像 -> 目标框/掩码”，不允许生成机器人动作。
class VisionGrounder(ABC):
    """Interface implemented by Qwen or any future grounding backend."""

    @abstractmethod
    def locate(self, image_bgr: np.ndarray, query: str) -> Sequence[Detection]:
        """Return all candidates matching *query* in *image_bgr*."""


class DisabledVisionGrounder(VisionGrounder):
    """Safe default used until a real model is configured."""

    def locate(self, image_bgr: np.ndarray, query: str) -> Sequence[Detection]:
        # 未选择真实模型时采用失败关闭策略，绝不返回猜测目标。
        del image_bgr, query
        raise VisionError(
            '视觉模型尚未配置。请设置 vision.provider，'
            '当前不会发送任何机械臂运动命令。'
        )


class FixedBoundingBoxGrounder(VisionGrounder):
    """Deterministic dry-run provider for testing RGB-D geometry."""

    def __init__(self, bounding_box: BoundingBox, confidence: float = 1.0):
        self._bounding_box = bounding_box
        self._confidence = float(confidence)

    def locate(self, image_bgr: np.ndarray, query: str) -> Sequence[Detection]:
        # 仅用于验证 RGB-D、TF 和坐标计算，不代表真实目标识别结果。
        if image_bgr.ndim < 2:
            raise VisionError('输入图像无效。')
        return [
            Detection(
                label=query,
                confidence=self._confidence,
                bounding_box=self._bounding_box,
            )
        ]


@dataclass(frozen=True)
class QwenProviderConfig:
    """Configuration reserved for the future Qwen implementation."""

    model: str
    base_url: str
    api_key_environment_variable: str
    timeout_sec: float
    max_retries: int

    def validate(self) -> None:
        """Reject incomplete or unsafe provider settings."""
        if not self.model.strip():
            raise VisionError('vision.qwen.model 不能为空。')
        if not self.base_url.strip():
            raise VisionError('vision.qwen.base_url 不能为空。')
        if not self.api_key_environment_variable.strip():
            raise VisionError('vision.qwen.api_key_env 不能为空。')
        if not math.isfinite(self.timeout_sec) or self.timeout_sec <= 0.0:
            raise VisionError('vision.qwen.timeout_sec 必须大于 0。')
        if self.max_retries < 0:
            raise VisionError('vision.qwen.max_retries 不能小于 0。')


class QwenVisionGrounder(VisionGrounder):
    """Stable interface boundary for the not-yet-selected Qwen model."""

    def __init__(self, config: QwenProviderConfig):
        self._config = config

    def locate(self, image_bgr: np.ndarray, query: str) -> Sequence[Detection]:
        # 型号和 API 尚未确定，因此当前明确报错，避免伪造模型输出。
        del image_bgr, query
        model = self._config.model or '<未指定>'
        raise VisionError(
            f'Qwen 接口已预留，但模型 {model} 的适配器尚未实现。'
        )


def create_vision_grounder(
    provider: str,
    fixed_bbox: List[float],
    qwen_config: QwenProviderConfig,
) -> VisionGrounder:
    """Construct a provider from ROS parameter values."""
    normalized_provider = provider.strip().lower()
    if normalized_provider == 'disabled':
        return DisabledVisionGrounder()
    if normalized_provider == 'fixed_bbox':
        if len(fixed_bbox) != 4:
            raise VisionError('vision.fixed_bbox 必须包含 4 个数值。')
        bounding_box = BoundingBox(
            *[float(value) for value in fixed_bbox],
            normalized=True,
        )
        bounding_box.to_pixels(1000, 1000)
        return FixedBoundingBoxGrounder(bounding_box)
    if normalized_provider == 'qwen':
        qwen_config.validate()
        return QwenVisionGrounder(qwen_config)
    raise VisionError(f'不支持的视觉提供器：{provider}')


def select_single_detection(
    detections: Sequence[Detection],
    minimum_confidence: float,
    policy: str,
) -> Detection:
    """Validate provider output and select one unambiguous candidate."""
    if not math.isfinite(minimum_confidence) or not (
        0.0 <= minimum_confidence <= 1.0
    ):
        raise VisionError('vision.minimum_confidence 必须位于 [0, 1]。')
    if policy != 'require_single':
        raise VisionError(f'尚未实现目标选择策略：{policy}')
    if detections is None:
        raise VisionError('视觉模型没有返回候选目标列表。')

    # 对外部模型输出逐项做类型与数值校验；异常输出不能进入深度计算。
    candidates = []
    for detection in detections:
        if not isinstance(detection, Detection):
            raise VisionError('视觉模型返回了非法候选目标类型。')
        confidence = float(detection.confidence)
        if not math.isfinite(confidence) or not 0.0 <= confidence <= 1.0:
            raise VisionError('视觉模型返回了非法置信度。')
        if not isinstance(detection.bounding_box, BoundingBox):
            raise VisionError('视觉模型返回了非法目标框。')
        if confidence >= minimum_confidence:
            candidates.append(detection)

    if not candidates:
        raise VisionError('没有检测到满足置信度要求的目标。')
    # 第一版不擅自选择多个同类目标，等待用户补充描述。
    if len(candidates) > 1:
        raise VisionError(
            f'检测到 {len(candidates)} 个候选目标，请进一步说明。'
        )
    return candidates[0]
