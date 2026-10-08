// Copyright 2026 HarvestX Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MG400_COMMON__KINEMATICS_HPP_
#define MG400_COMMON__KINEMATICS_HPP_

#include <algorithm>
#include <cmath>
#include <stdexcept>

#include <eigen3/Eigen/Dense>

namespace mg400_common
{
namespace kinematics
{

inline double rad2deg(double rad)
{
  return rad * 180.0 / M_PI;
}

inline double deg2rad(double deg)
{
  return deg * M_PI / 180.0;
}

inline double normalize_angle(double angle)
{
  return std::atan2(std::sin(angle), std::cos(angle));
}

const double J1_MIN = deg2rad(-160);
const double J1_MAX = deg2rad(160);

const double J2_MIN = deg2rad(-25);
const double J2_MIN_NO_COLLISION = deg2rad(-19);
const double J2_MAX = deg2rad(85);

const double J3_MIN = deg2rad(-25);
const double J3_MAX = deg2rad(105);

const double J3_1_MIN = deg2rad(-60);
const double J3_1_MAX = deg2rad(60);

const double J4_MIN = deg2rad(-160);
const double J4_MAX = deg2rad(160);

const double MARGIN = deg2rad(1.05);

const Eigen::Vector3d LINK1(0.0435, 0.0, 0.0);
const Eigen::Vector3d LINK2(0.0, 0.0, 0.175);
const Eigen::Vector3d LINK3(0.1750, 0.0, 0.0);
const Eigen::Vector3d LINK4(0.0660, 0.0, -0.053);

struct ConstraintResult
{
  bool j1_valid;
  bool j2_valid;
  bool j3_valid;
  bool j4_valid;
  bool j3_1_valid;

  bool allValid() const
  {
    return j1_valid && j2_valid && j3_valid && j4_valid && j3_1_valid;
  }
};

struct ConstraintOptions
{
  double j1_min = J1_MIN;
  double j1_max = J1_MAX;
  double j2_min = J2_MIN;
  double j2_min_no_collision = J2_MIN_NO_COLLISION;
  double j2_max = J2_MAX;
  double j3_min = J3_MIN;
  double j3_max = J3_MAX;
  double j3_1_min = J3_1_MIN;
  double j3_1_max = J3_1_MAX;
  double j4_min = J4_MIN;
  double j4_max = J4_MAX;
  double margin = MARGIN;
};

inline ConstraintResult check_constraints(
  const Eigen::Vector4d & theta,
  const ConstraintOptions & options = ConstraintOptions())
{
  ConstraintResult result;

  const double j1 = theta(0);
  const double j2 = theta(1);
  const double j3 = theta(2);
  const double j4 = normalize_angle(theta(3));
  const double j3_1 = j3 - j2;

  auto within_limits = [](double value, double min, double max, double margin) {
      return (value >= min + margin) && (value <= max - margin);
    };

  result.j1_valid = within_limits(j1, options.j1_min, options.j1_max, options.margin);
  result.j2_valid = (j3 > 0) ?
    within_limits(j2, options.j2_min, options.j2_max, options.margin) :
    within_limits(j2, options.j2_min_no_collision, options.j2_max, options.margin);
  result.j3_valid = within_limits(j3, options.j3_min, options.j3_max, options.margin);
  result.j4_valid = within_limits(j4, options.j4_min, options.j4_max, options.margin);
  result.j3_1_valid = within_limits(
    j3_1, options.j3_1_min, options.j3_1_max, options.margin);

  return result;
}

namespace detail
{

inline Eigen::Matrix4d create_homotrans_matrix(
  const Eigen::Vector3d & rotation_axis,
  const double rotation_angle,
  const Eigen::Vector3d & translation)
{
  Eigen::Matrix4d transform = Eigen::Matrix4d::Identity();
  const Eigen::Vector3d normalized_axis = rotation_axis.normalized();
  const Eigen::Matrix3d rotation_matrix =
    Eigen::AngleAxisd(rotation_angle, normalized_axis).toRotationMatrix();

  transform.block<3, 3>(0, 0) = rotation_matrix;
  transform.block<3, 1>(0, 3) = rotation_matrix * translation;

  return transform;
}

inline Eigen::Matrix4d create_homotrans_matrix_x(
  const double angle,
  const Eigen::Vector3d & translation)
{
  return create_homotrans_matrix(Eigen::Vector3d::UnitX(), angle, translation);
}

inline Eigen::Matrix4d create_homotrans_matrix_y(
  const double angle,
  const Eigen::Vector3d & translation)
{
  return create_homotrans_matrix(Eigen::Vector3d::UnitY(), angle, translation);
}

inline Eigen::Matrix4d create_homotrans_matrix_z(
  const double angle,
  const Eigen::Vector3d & translation)
{
  return create_homotrans_matrix(Eigen::Vector3d::UnitZ(), angle, translation);
}

inline Eigen::Vector4d ik(const Eigen::Vector4d & input, const Eigen::Vector3d & tool_length)
{
  const double p_x = input(0);
  const double p_y = input(1);
  const double p_z = input(2);
  const double r_x = input(3);

  const Eigen::Matrix3d r_z =
    Eigen::AngleAxisd(r_x, Eigen::Vector3d::UnitZ()).toRotationMatrix();
  const Eigen::Vector3d tool_offset = r_z * tool_length;

  Eigen::Vector3d corrected_p(p_x, p_y, p_z);
  corrected_p -= tool_offset;

  const double pp_x = corrected_p.head<2>().norm() - LINK4(0) - LINK1(0);
  const double pp_z = corrected_p(2) - LINK4(2) - LINK1(2);

  const double length2 = LINK2.norm();
  const double length3 = LINK3.norm();

  const double j_1 = std::atan2(corrected_p(1), corrected_p(0));

  const double val1 =
    (std::pow(pp_x, 2) + std::pow(pp_z, 2) - std::pow(length2, 2) -
    std::pow(length3, 2)) / (2 * length2 * length3);
  if (val1 < -1.0 || val1 > 1.0) {
    throw std::runtime_error("Inverse kinematics error: acos domain out of range.");
  }

  double j_3_1 = std::asin(val1);
  double j_2 = std::atan2(pp_z, pp_x) -
    std::atan2(length2 + length3 * std::sin(j_3_1), length3 * std::cos(j_3_1));

  j_2 = -j_2;
  j_3_1 = -j_3_1;

  Eigen::Vector4d angles;
  angles << j_1, j_2, j_2 + j_3_1, normalize_angle(r_x - j_1);

  return angles;
}

inline Eigen::Vector4d fk(const Eigen::Vector4d & theta, const Eigen::Vector3d & tool_length)
{
  const double j1 = theta(0);
  const double j2 = theta(1);
  const double j3 = theta(2);
  const double j4 = theta(3);

  const Eigen::Matrix4d t0_1 = create_homotrans_matrix_z(j1, LINK1);
  const Eigen::Matrix4d t1_2 = create_homotrans_matrix_y(j2, LINK2);
  const Eigen::Matrix4d t2_3 = create_homotrans_matrix_y(j3 - j2, LINK3);
  const Eigen::Matrix4d t3_4 = create_homotrans_matrix_y(-j3, LINK4);
  const Eigen::Matrix4d t4_tool = create_homotrans_matrix_z(j4, tool_length);

  const Eigen::Matrix4d t0_tool = t0_1 * t1_2 * t2_3 * t3_4 * t4_tool;
  const Eigen::Vector3d tool_tip_pos = t0_tool.block<3, 1>(0, 3);

  Eigen::Vector4d result;
  result.head<3>() = tool_tip_pos;
  result(3) = j1 + j4;

  return result;
}

}  // namespace detail

inline Eigen::Vector4d ik_for_flange_from_origin(const Eigen::Vector4d & input_from_origin)
{
  return detail::ik(input_from_origin, Eigen::Vector3d::Zero());
}

inline Eigen::Vector4d fk_for_flange_from_origin(const Eigen::Vector4d & input)
{
  return detail::fk(input, Eigen::Vector3d::Zero());
}

}  // namespace kinematics
}  // namespace mg400_common

#endif  // MG400_COMMON__KINEMATICS_HPP_
