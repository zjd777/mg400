"""深度定位工具：筛选对齐深度、反投影目标像素并输出相机坐标系三维点."""

from typing import Optional

import numpy as np

from .models import (
    BoundingBox,
    CameraIntrinsics,
    DepthError,
    LocalizedPoint,
)


def depth_to_meters(
    depth_image: np.ndarray,
    encoding: str,
    depth_scale: float,
) -> np.ndarray:
    """把二维 ROS 深度图统一转为 float64 米制数组，不在此处筛选坏点。

    16UC1/MONO16 的原始值乘以 depth_scale（米/原始单位）；32FC1
    按 ROS 常用约定直接视为米。非单通道、不支持的编码或 16 位图的
    scale 不为正时抛出 DepthError。
    """
    if depth_image.ndim != 2:
        raise DepthError('深度图必须是单通道图像。')

    normalized_encoding = encoding.upper()
    if normalized_encoding in ('16UC1', 'MONO16'):
        # D435 常见的 16UC1 深度单位是毫米，通过 scale 统一转换为米。
        if depth_scale <= 0.0:
            raise DepthError('16 位深度图的 depth_scale 必须大于 0。')
        return depth_image.astype(np.float64) * depth_scale
    if normalized_encoding == '32FC1':
        return depth_image.astype(np.float64)

    raise DepthError(f'不支持的深度图编码：{encoding}')


def localize_depth_region(
    depth_m: np.ndarray,
    intrinsics: CameraIntrinsics,
    bounding_box: BoundingBox,
    mask: Optional[np.ndarray] = None,
    roi_shrink_ratio: float = 0.2,
    minimum_depth_m: float = 0.1,
    maximum_depth_m: float = 2.0,
    minimum_valid_ratio: float = 0.2,
    minimum_valid_samples: int = 1,
) -> LocalizedPoint:
    """在目标框中心取有效深度，反投影后输出相机光学坐标系中的点。

    depth_m 单位为米，必须与检测图像对齐；intrinsics 的焦距/主点
    单位为像素，mask（可选）须与深度图同尺寸。roi_shrink_ratio 是
    每侧收缩比例，例如 0.2 保留约 60% 的宽和高，而非保留 80%。

    输出 XYZ 单位为米，光学坐标轴为右 X、下 Y、前 Z；pixel_u/v 和
    有效样本统计用于诊断。参数、框、掩码或有效深度不合格时失败。
    本方法假设内参已校验，后续由任务节点用 TF 转换到机械臂坐标。
    """
    if depth_m.ndim != 2:
        raise DepthError('深度数组必须是二维数组。')
    validate_depth_parameters(
        roi_shrink_ratio=roi_shrink_ratio,
        minimum_depth_m=minimum_depth_m,
        maximum_depth_m=maximum_depth_m,
        minimum_valid_ratio=minimum_valid_ratio,
        minimum_valid_samples=minimum_valid_samples,
    )

    height, width = depth_m.shape
    pixel_box = bounding_box.to_pixels(width, height)
    box_width = pixel_box.x_max - pixel_box.x_min
    box_height = pixel_box.y_max - pixel_box.y_min

    # 向框中心收缩，减少目标框边缘处背景深度混入目标表面。
    x0 = int(np.floor(pixel_box.x_min + box_width * roi_shrink_ratio))
    x1 = int(np.ceil(pixel_box.x_max - box_width * roi_shrink_ratio))
    y0 = int(np.floor(pixel_box.y_min + box_height * roi_shrink_ratio))
    y1 = int(np.ceil(pixel_box.y_max - box_height * roi_shrink_ratio))

    x0 = max(0, min(width - 1, x0))
    x1 = max(x0 + 1, min(width, x1))
    y0 = max(0, min(height - 1, y0))
    y1 = max(y0 + 1, min(height, y1))

    region = depth_m[y0:y1, x0:x1]
    valid = (
        np.isfinite(region)
        & (region >= minimum_depth_m)
        & (region <= maximum_depth_m)
    )

    candidate_count = int(region.size)
    if mask is not None:
        if mask.shape != depth_m.shape:
            raise DepthError('目标掩码尺寸与深度图不一致。')
        region_mask = mask[y0:y1, x0:x1].astype(bool)
        # 有掩码时，比例分母必须是掩码像素数，不能包含掩码外背景。
        candidate_count = int(np.count_nonzero(region_mask))
        if candidate_count == 0:
            raise DepthError('目标掩码在取样区域内没有有效像素。')
        valid &= region_mask

    sample_count = int(np.count_nonzero(valid))
    # 有效比例只表示传感器有可用深度，不证明这些像素都属于目标物体；
    # 没有前景 mask 时，框内的背景/桌面也可能通过下面的范围筛选。
    valid_ratio = sample_count / candidate_count if candidate_count else 0.0
    if (
        sample_count < minimum_valid_samples
        or valid_ratio < minimum_valid_ratio
    ):
        raise DepthError(
            '目标区域有效深度不足：'
            f'{sample_count}/{candidate_count} ({valid_ratio:.1%})，'
            f'至少需要 {minimum_valid_samples} 个样本。'
        )

    local_v, local_u = np.nonzero(valid)
    pixel_u = local_u.astype(np.float64) + x0
    pixel_v = local_v.astype(np.float64) + y0
    z_values = region[valid]
    # 针孔模型反投影：由像素 (u,v) 和深度 Z 得到相机光学坐标 XYZ。
    x_values = (pixel_u - intrinsics.cx) * z_values / intrinsics.fx
    y_values = (pixel_v - intrinsics.cy) * z_values / intrinsics.fy

    # 分别对 X、Y、Z 和像素坐标取中值，而不是挑出某一个原始像素。
    # 因此输出是区域表面的统计估计，并不保证来自同一个深度样本。
    # 中值比均值更不容易被少量飞点和深度空洞边缘影响。
    return LocalizedPoint(
        x=float(np.median(x_values)),
        y=float(np.median(y_values)),
        z=float(np.median(z_values)),
        pixel_u=float(np.median(pixel_u)),
        pixel_v=float(np.median(pixel_v)),
        valid_ratio=float(valid_ratio),
        sample_count=sample_count,
    )


def validate_depth_parameters(
    roi_shrink_ratio: float,
    minimum_depth_m: float,
    maximum_depth_m: float,
    minimum_valid_ratio: float,
    minimum_valid_samples: int,
) -> None:
    """校验深度范围（米）、ROI 收缩比例和有效样本门槛，不依赖图像。

    收缩比例须在 [0, 0.5)，有效比例在 (0, 1]，有效样本数至少为 1；
    浮点阈值非有限或范围关系非法时抛出 DepthError。
    """
    values = (
        roi_shrink_ratio,
        minimum_depth_m,
        maximum_depth_m,
        minimum_valid_ratio,
    )
    if not all(np.isfinite(value) for value in values):
        raise DepthError('深度参数必须是有限数值。')
    if not 0.0 <= roi_shrink_ratio < 0.5:
        raise DepthError('depth.roi_shrink_ratio 必须位于 [0, 0.5) 内。')
    if not 0.0 < minimum_valid_ratio <= 1.0:
        raise DepthError('depth.minimum_valid_ratio 必须位于 (0, 1] 内。')
    if minimum_depth_m <= 0.0 or maximum_depth_m <= minimum_depth_m:
        raise DepthError('深度有效范围配置错误。')
    if minimum_valid_samples < 1:
        raise DepthError('depth.minimum_valid_samples 必须大于等于 1。')
