// Copyright 2023 HarvestX Inc.
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

#include "mg400_common/mg400_ik_util.hpp"

#include <cmath>
#include <stdexcept>

#include "mg400_common/kinematics.hpp"

namespace
{

constexpr size_t KINEMATICS_POSE_SIZE = 4;

void validate_vector_size(const std::vector<double> & values, size_t expected_minimum_size)
{
  if (values.size() < expected_minimum_size) {
    throw std::invalid_argument("MG400IKUtil input must contain at least 4 elements.");
  }
}

Eigen::Vector4d to_eigen4(const std::vector<double> & values)
{
  validate_vector_size(values, KINEMATICS_POSE_SIZE);

  Eigen::Vector4d result;
  result << values[0], values[1], values[2], values[3];
  return result;
}

std::vector<double> to_legacy_joint_vector(
  const Eigen::Vector4d & joints,
  double round_decimals)
{
  std::vector<double> rounded_angles = {
    joints(0), joints(1), joints(2), joints(3), 0.0, 0.0
  };

  const double round_scale = std::pow(10.0, round_decimals);
  for (auto & angle : rounded_angles) {
    angle = std::round(angle * round_scale) / round_scale;
  }

  return rounded_angles;
}

}  // namespace

namespace mg400_common
{
bool MG400IKUtil::InMG400Range(const std::vector<double> & angles)
{
  const Eigen::Vector4d joints = to_eigen4(angles);
  return kinematics::check_constraints(joints).allValid();
}

std::vector<double> MG400IKUtil::InverseKinematics(
  const std::vector<double> & tool_vec)
{
  const Eigen::Vector4d pose = to_eigen4(tool_vec);
  Eigen::Vector4d angles;
  try {
    angles = kinematics::ik_for_flange_from_origin(pose);
  } catch (const std::runtime_error &) {
    throw std::runtime_error("Inverse kinematics error");
  }

  if (!kinematics::check_constraints(angles).allValid()) {
    throw std::runtime_error("Outside of workspace.");
  }

  return to_legacy_joint_vector(angles, ROUND_DECIMALS);
}

std::vector<double> MG400IKUtil::ToolCoordToBaseCoord(
  const std::vector<double> & vec,
  const std::vector<double> & tool_coord)
{
  validate_vector_size(vec, KINEMATICS_POSE_SIZE);
  validate_vector_size(tool_coord, KINEMATICS_POSE_SIZE);

  std::vector<double> pos(3);
  for (int i = 0; i < 3; ++i) {
    pos[i] = vec[i] - tool_coord[i];
  }
  double ang = vec[3] + tool_coord[3];

  return std::vector<double>{pos[0], pos[1], pos[2], ang, 0.0, 0.0};
}

}  // namespace mg400_common
