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

#include <cmath>
#include <random>
#include <stdexcept>
#include <vector>

#include <gtest/gtest.h>

#include "mg400_common/kinematics.hpp"
#include "mg400_common/mg400_ik_util.hpp"

namespace
{

double wrapped_angle_distance(double lhs, double rhs)
{
  return std::abs(mg400_common::kinematics::normalize_angle(lhs - rhs));
}

Eigen::Vector4d generate_valid_joints(std::mt19937 & generator)
{
  std::uniform_real_distribution<double> dist_joint1(
    mg400_common::kinematics::J1_MIN + mg400_common::kinematics::MARGIN,
    mg400_common::kinematics::J1_MAX - mg400_common::kinematics::MARGIN);
  std::uniform_real_distribution<double> dist_joint2(
    mg400_common::kinematics::J2_MIN_NO_COLLISION + mg400_common::kinematics::MARGIN,
    mg400_common::kinematics::J2_MAX - mg400_common::kinematics::MARGIN);
  std::uniform_real_distribution<double> dist_joint3(
    mg400_common::kinematics::J3_MIN + mg400_common::kinematics::MARGIN,
    mg400_common::kinematics::J3_MAX - mg400_common::kinematics::MARGIN);
  std::uniform_real_distribution<double> dist_joint4(
    mg400_common::kinematics::J4_MIN + mg400_common::kinematics::MARGIN,
    mg400_common::kinematics::J4_MAX - mg400_common::kinematics::MARGIN);

  for (int attempt = 0; attempt < 1000; ++attempt) {
    Eigen::Vector4d joints;
    joints << dist_joint1(generator), dist_joint2(generator),
      dist_joint3(generator), dist_joint4(generator);
    if (mg400_common::kinematics::check_constraints(joints).allValid()) {
      return joints;
    }
  }

  return Eigen::Vector4d::Zero();
}

}  // namespace

TEST(MG400KinematicsTest, NormalizeAngleHandlesWrapBoundaries)
{
  EXPECT_NEAR(mg400_common::kinematics::normalize_angle(2.0 * M_PI + 0.25), 0.25, 1e-12);
  EXPECT_NEAR(mg400_common::kinematics::normalize_angle(-2.0 * M_PI - 0.25), -0.25, 1e-12);
  EXPECT_NEAR(
    mg400_common::kinematics::normalize_angle(M_PI + 0.1),
    -M_PI + 0.1, 1e-12);
}

TEST(MG400KinematicsTest, ConstraintCheckNormalizesWrappedJ4)
{
  Eigen::Vector4d joints;
  joints << 0.0, 0.0, 0.0, mg400_common::kinematics::deg2rad(280.0);

  const auto result = mg400_common::kinematics::check_constraints(joints);

  EXPECT_TRUE(result.allValid());
  EXPECT_TRUE(result.j4_valid);
}

TEST(MG400KinematicsTest, ConstraintCheckAcceptsOverriddenLimits)
{
  Eigen::Vector4d joints;
  joints << mg400_common::kinematics::deg2rad(170.0), 0.0, 0.0, 0.0;

  EXPECT_FALSE(mg400_common::kinematics::check_constraints(joints).j1_valid);

  mg400_common::kinematics::ConstraintOptions options;
  options.j1_min = mg400_common::kinematics::deg2rad(-180.0);
  options.j1_max = mg400_common::kinematics::deg2rad(180.0);
  options.margin = 0.0;

  const auto result = mg400_common::kinematics::check_constraints(joints, options);

  EXPECT_TRUE(result.j1_valid);
}

TEST(MG400KinematicsTest, IKFKRoundTripRemainsStable)
{
  std::mt19937 generator(12345);

  for (int i = 0; i < 50; ++i) {
    const Eigen::Vector4d original_joints = generate_valid_joints(generator);
    const Eigen::Vector4d pose =
      mg400_common::kinematics::fk_for_flange_from_origin(original_joints);
    const Eigen::Vector4d recovered_joints =
      mg400_common::kinematics::ik_for_flange_from_origin(pose);
    const Eigen::Vector4d recovered_pose =
      mg400_common::kinematics::fk_for_flange_from_origin(recovered_joints);

    EXPECT_NEAR((pose.head<3>() - recovered_pose.head<3>()).norm(), 0.0, 1e-6);
    EXPECT_NEAR(wrapped_angle_distance(pose(3), recovered_pose(3)), 0.0, 1e-6);
    EXPECT_TRUE(mg400_common::kinematics::check_constraints(recovered_joints).allValid());
  }
}

TEST(MG400KinematicsTest, IKAcceptsWrappedYawPose)
{
  Eigen::Vector4d original_joints;
  original_joints << mg400_common::kinematics::deg2rad(112.353),
    mg400_common::kinematics::deg2rad(23.769),
    mg400_common::kinematics::deg2rad(17.340),
    mg400_common::kinematics::deg2rad(69.251);

  Eigen::Vector4d pose = mg400_common::kinematics::fk_for_flange_from_origin(original_joints);
  pose(3) = mg400_common::kinematics::normalize_angle(pose(3));

  const Eigen::Vector4d recovered_joints =
    mg400_common::kinematics::ik_for_flange_from_origin(pose);

  EXPECT_TRUE(mg400_common::kinematics::check_constraints(recovered_joints).allValid());
  EXPECT_NEAR(
    wrapped_angle_distance(
      mg400_common::kinematics::normalize_angle(original_joints(3)),
      recovered_joints(3)),
    0.0, 1e-4);
}

TEST(MG400KinematicsTest, MG400IKUtilDelegatesToCanonicalImplementation)
{
  mg400_common::MG400IKUtil ik_util;
  const std::vector<double> pose = {-0.132, 0.321, 0.055, -3.113594};

  const std::vector<double> actual = ik_util.InverseKinematics(pose);

  Eigen::Vector4d eigen_pose;
  eigen_pose << pose[0], pose[1], pose[2], pose[3];
  const Eigen::Vector4d expected =
    mg400_common::kinematics::ik_for_flange_from_origin(eigen_pose);

  ASSERT_EQ(actual.size(), 6u);
  EXPECT_NEAR(actual[0], expected(0), 1e-8);
  EXPECT_NEAR(actual[1], expected(1), 1e-8);
  EXPECT_NEAR(actual[2], expected(2), 1e-8);
  EXPECT_NEAR(actual[3], expected(3), 1e-8);
  EXPECT_DOUBLE_EQ(actual[4], 0.0);
  EXPECT_DOUBLE_EQ(actual[5], 0.0);
}

TEST(MG400KinematicsTest, MG400IKUtilRejectsShortInputVectors)
{
  mg400_common::MG400IKUtil ik_util;

  EXPECT_THROW(ik_util.InMG400Range({0.0, 0.0, 0.0}), std::invalid_argument);
  EXPECT_THROW(ik_util.InverseKinematics({0.0, 0.0, 0.0}), std::invalid_argument);
  EXPECT_THROW(
    ik_util.ToolCoordToBaseCoord({0.0, 0.0, 0.0}, {0.0, 0.0, 0.0, 0.0}),
    std::invalid_argument);
}

TEST(MG400KinematicsTest, ToolCoordToBaseCoordUsesFourthElementAsYawOffset)
{
  mg400_common::MG400IKUtil ik_util;

  const std::vector<double> result =
    ik_util.ToolCoordToBaseCoord({0.3, 0.2, 0.1, 0.4}, {0.1, 0.2, 0.3, 0.5, 9.9, 8.8});

  ASSERT_EQ(result.size(), 6u);
  EXPECT_DOUBLE_EQ(result[0], 0.2);
  EXPECT_DOUBLE_EQ(result[1], 0.0);
  EXPECT_DOUBLE_EQ(result[2], -0.2);
  EXPECT_DOUBLE_EQ(result[3], 0.9);
}
