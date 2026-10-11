import numpy as np
import pytest

from object_finder.models import BoundingBox, Detection, VisionError
from object_finder.vision import (
    DisabledVisionGrounder,
    FixedBoundingBoxGrounder,
    QwenProviderConfig,
    create_vision_grounder,
    select_single_detection,
)


def test_normalized_box_is_clipped_and_converted():
    box = BoundingBox(-0.1, 0.25, 1.1, 0.75, normalized=True)
    pixels = box.to_pixels(640, 480)
    assert pixels.x_min == 0.0
    assert pixels.y_min == 120.0
    assert pixels.x_max == 640.0
    assert pixels.y_max == 360.0


def test_disabled_provider_fails_safely():
    provider = DisabledVisionGrounder()
    with pytest.raises(VisionError):
        provider.locate(np.zeros((10, 10, 3), dtype=np.uint8), 'cup')


def test_fixed_provider_returns_one_candidate():
    provider = FixedBoundingBoxGrounder(
        BoundingBox(0.2, 0.2, 0.8, 0.8, normalized=True)
    )
    detections = provider.locate(
        np.zeros((10, 10, 3), dtype=np.uint8),
        'cup',
    )
    assert len(detections) == 1
    assert detections[0].label == 'cup'


def test_selection_rejects_nonfinite_confidence():
    detection = Detection(
        label='cup',
        confidence=np.nan,
        bounding_box=BoundingBox(0.2, 0.2, 0.8, 0.8, normalized=True),
    )
    with pytest.raises(VisionError):
        select_single_detection([detection], 0.6, 'require_single')


def test_selection_rejects_ambiguous_candidates():
    detections = [
        Detection(
            label='cup',
            confidence=0.9,
            bounding_box=BoundingBox(
                0.1,
                0.1,
                0.3,
                0.3,
                normalized=True,
            ),
        ),
        Detection(
            label='cup',
            confidence=0.8,
            bounding_box=BoundingBox(
                0.6,
                0.6,
                0.8,
                0.8,
                normalized=True,
            ),
        ),
    ]
    with pytest.raises(VisionError):
        select_single_detection(detections, 0.6, 'require_single')


def test_qwen_provider_rejects_incomplete_configuration():
    config = QwenProviderConfig(
        model='',
        base_url='',
        api_key_environment_variable='DASHSCOPE_API_KEY',
        timeout_sec=30.0,
        max_retries=2,
    )
    with pytest.raises(VisionError):
        create_vision_grounder('qwen', [0.2, 0.2, 0.8, 0.8], config)
