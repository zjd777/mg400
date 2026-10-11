"""视觉模块单元测试：覆盖目标框、模型输出解析、重试与目标选择策略."""

import numpy as np
import pytest

from object_finder.models import BoundingBox, Detection, VisionError
from object_finder.vision import (
    DisabledVisionGrounder,
    FixedBoundingBoxGrounder,
    QwenProviderConfig,
    QwenVisionGrounder,
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


def _configured_qwen(api_key_env='OBJECT_FINDER_TEST_API_KEY'):
    return QwenVisionGrounder(
        QwenProviderConfig(
            model='qwen3-vl-plus',
            base_url=(
                'https://dashscope.aliyuncs.com/compatible-mode/v1'
            ),
            api_key_environment_variable=api_key_env,
            timeout_sec=30.0,
            max_retries=0,
        )
    )


def test_qwen_provider_requires_api_key(monkeypatch):
    variable = 'OBJECT_FINDER_TEST_MISSING_API_KEY'
    monkeypatch.delenv(variable, raising=False)
    provider = _configured_qwen(variable)

    with pytest.raises(VisionError, match=variable):
        provider.locate(
            np.zeros((10, 10, 3), dtype=np.uint8),
            '绿色杯子',
        )


def test_qwen_provider_encodes_and_parses_response(monkeypatch):
    variable = 'OBJECT_FINDER_TEST_API_KEY'
    monkeypatch.setenv(variable, 'test-key-not-a-real-secret')
    provider = _configured_qwen(variable)
    captured = {}

    def fake_post_json(payload, api_key):
        captured['payload'] = payload
        captured['api_key'] = api_key
        return {
            'choices': [
                {
                    'message': {
                        'content': (
                            '{"detections":[{"label":"绿色杯子",'
                            '"confidence":0.92,'
                            '"bbox_2d":[100,200,500,800]}]}'
                        )
                    }
                }
            ]
        }

    monkeypatch.setattr(provider, '_post_json', fake_post_json)
    detections = provider.locate(
        np.zeros((20, 30, 3), dtype=np.uint8),
        '绿色杯子',
    )

    assert captured['api_key'] == 'test-key-not-a-real-secret'
    content = captured['payload']['messages'][1]['content']
    assert content[0]['image_url']['url'].startswith(
        'data:image/jpeg;base64,'
    )
    assert len(detections) == 1
    assert detections[0].label == '绿色杯子'
    assert detections[0].confidence == 0.92
    assert detections[0].bounding_box == BoundingBox(
        0.1,
        0.2,
        0.5,
        0.8,
        normalized=True,
    )


def test_qwen_provider_rejects_out_of_range_box(monkeypatch):
    variable = 'OBJECT_FINDER_TEST_API_KEY'
    monkeypatch.setenv(variable, 'test-key-not-a-real-secret')
    provider = _configured_qwen(variable)

    monkeypatch.setattr(
        provider,
        '_post_json',
        lambda payload, api_key: {
            'choices': [
                {
                    'message': {
                        'content': (
                            '{"detections":[{"label":"cup",'
                            '"confidence":0.9,'
                            '"bbox_2d":[-1,10,500,800]}]}'
                        )
                    }
                }
            ]
        },
    )

    with pytest.raises(VisionError, match='\\[0, 1000\\]'):
        provider.locate(
            np.zeros((20, 30, 3), dtype=np.uint8),
            'cup',
        )
