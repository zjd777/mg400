"""运动安全单元测试：覆盖工作区、悬停高度、关节约束和速度范围."""

import pytest

from object_finder.models import SafetyError
from object_finder.safety import (
    WorkspaceBounds,
    validate_height_parameters,
    validate_mg400_joint_angles,
    validate_mg400_radial_reach,
    validate_percentage,
)


def test_workspace_accepts_inside_point():
    bounds = WorkspaceBounds(-0.4, 0.4, -0.4, 0.4, 0.0, 0.3)
    bounds.validate(0.2, -0.1, 0.1, 'test')


def test_workspace_rejects_outside_point():
    bounds = WorkspaceBounds(-0.4, 0.4, -0.4, 0.4, 0.0, 0.3)
    with pytest.raises(SafetyError):
        bounds.validate(0.5, 0.0, 0.1, 'test')


def test_approach_height_must_exceed_standoff():
    with pytest.raises(SafetyError):
        validate_height_parameters(0.05, 0.05)


def test_valid_mg400_home_joint_angles_are_accepted():
    validate_mg400_joint_angles([0.0, 0.0, 0.5, 0.0])


def test_mg400_home_rejects_linkage_constraint_violation():
    with pytest.raises(SafetyError):
        validate_mg400_joint_angles([0.0, -0.3, 1.7, 0.0])


def test_motion_speed_cannot_be_zero_but_cp_can():
    with pytest.raises(SafetyError):
        validate_percentage(0, 'motion.speed_j')
    validate_percentage(0, 'motion.cp', allow_zero=True)


def test_mg400_official_radial_reach_is_enforced():
    validate_mg400_radial_reach(0.30, 0.20)
    with pytest.raises(SafetyError):
        validate_mg400_radial_reach(0.40, 0.30)
