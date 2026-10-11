import numpy as np
import pytest

from object_finder.depth import depth_to_meters, localize_depth_region
from object_finder.models import BoundingBox, CameraIntrinsics, DepthError


def test_depth_to_meters_for_16uc1():
    image = np.array([[0, 1000], [1500, 2000]], dtype=np.uint16)
    converted = depth_to_meters(image, '16UC1', 0.001)
    assert converted.tolist() == [[0.0, 1.0], [1.5, 2.0]]


def test_localize_depth_region_rejects_too_few_valid_samples():
    depth = np.zeros((10, 10), dtype=np.float64)
    depth[5, 5] = 0.5
    with pytest.raises(DepthError):
        localize_depth_region(
            depth,
            CameraIntrinsics(100.0, 100.0, 5.0, 5.0),
            BoundingBox(0.0, 0.0, 1.0, 1.0, normalized=True),
            roi_shrink_ratio=0.0,
            minimum_valid_ratio=0.2,
        )


def test_localize_depth_region_returns_optical_point():
    depth = np.full((10, 10), 1.0, dtype=np.float64)
    result = localize_depth_region(
        depth,
        CameraIntrinsics(100.0, 100.0, 5.0, 5.0),
        BoundingBox(0.4, 0.4, 0.6, 0.6, normalized=True),
        roi_shrink_ratio=0.0,
        minimum_valid_ratio=1.0,
    )
    assert result.z == pytest.approx(1.0)
    assert result.x == pytest.approx(-0.005)
    assert result.y == pytest.approx(-0.005)
    assert result.valid_ratio == pytest.approx(1.0)


def test_mask_ratio_uses_only_mask_pixels_as_candidates():
    depth = np.zeros((10, 10), dtype=np.float64)
    mask = np.zeros((10, 10), dtype=bool)
    mask[4:6, 4:6] = True
    depth[mask] = 1.0

    result = localize_depth_region(
        depth,
        CameraIntrinsics(100.0, 100.0, 5.0, 5.0),
        BoundingBox(0.0, 0.0, 1.0, 1.0, normalized=True),
        mask=mask,
        roi_shrink_ratio=0.0,
        minimum_valid_ratio=1.0,
        minimum_valid_samples=4,
    )

    assert result.sample_count == 4
    assert result.valid_ratio == pytest.approx(1.0)


def test_localize_depth_region_enforces_minimum_sample_count():
    depth = np.ones((4, 4), dtype=np.float64)
    with pytest.raises(DepthError):
        localize_depth_region(
            depth,
            CameraIntrinsics(100.0, 100.0, 2.0, 2.0),
            BoundingBox(0.0, 0.0, 1.0, 1.0, normalized=True),
            roi_shrink_ratio=0.0,
            minimum_valid_ratio=1.0,
            minimum_valid_samples=17,
        )


def test_camera_intrinsics_reject_nonfinite_values():
    matrix = [100.0, 0.0, np.nan, 0.0, 100.0, 5.0, 0.0, 0.0, 1.0]
    with pytest.raises(DepthError):
        CameraIntrinsics.from_camera_matrix(matrix)
