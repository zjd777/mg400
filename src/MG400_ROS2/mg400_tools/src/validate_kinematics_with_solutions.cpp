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

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <limits>
#include <random>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include <rclcpp/rclcpp.hpp>

#include "mg400_common/kinematics.hpp"
#include "mg400_msgs/srv/inverse_solution.hpp"
#include "mg400_msgs/srv/positive_solution.hpp"

namespace
{

using namespace std::chrono_literals;  // NOLINT
using PositiveSolution = mg400_msgs::srv::PositiveSolution;
using InverseSolution = mg400_msgs::srv::InverseSolution;

struct Config
{
  size_t case_count = 50;
  unsigned int seed = 12345;
  double service_wait_timeout_sec = 5.0;
  double service_call_timeout_sec = 3.0;
  double pose_position_tolerance_m = 1e-3;
  double pose_angle_tolerance_rad = mg400_common::kinematics::deg2rad(0.5);
  double joint_tolerance_rad = mg400_common::kinematics::deg2rad(0.5);
};

struct CaseResult
{
  size_t index = 0;
  bool passed = false;
  double max_pose_position_error_m = 0.0;
  double pose_angle_error_rad = 0.0;
  double max_joint_error_rad = 0.0;
  int positive_error_id = -1;
  int inverse_error_id = -1;
  Eigen::Vector4d reference_joints = Eigen::Vector4d::Zero();
  Eigen::Vector4d local_pose = Eigen::Vector4d::Zero();
  Eigen::Vector4d local_ik_joints = Eigen::Vector4d::Zero();
  Eigen::Vector4d service_pose = Eigen::Vector4d::Zero();
  Eigen::Vector4d service_joints = Eigen::Vector4d::Zero();
};

std::string format_joint_vector_deg(const Eigen::Vector4d & values)
{
  std::ostringstream stream;
  stream << std::fixed << std::setprecision(3)
         << "["
         << mg400_common::kinematics::rad2deg(values(0)) << ", "
         << mg400_common::kinematics::rad2deg(values(1)) << ", "
         << mg400_common::kinematics::rad2deg(values(2)) << ", "
         << mg400_common::kinematics::rad2deg(values(3)) << "]";
  return stream.str();
}

std::string format_pose_vector_mm_deg(const Eigen::Vector4d & values)
{
  std::ostringstream stream;
  stream << std::fixed << std::setprecision(3)
         << "["
         << values(0) * 1000.0 << ", "
         << values(1) * 1000.0 << ", "
         << values(2) * 1000.0 << ", "
         << mg400_common::kinematics::rad2deg(values(3)) << "]";
  return stream.str();
}

std::string format_pose_diff_mm_deg(const Eigen::Vector4d & lhs, const Eigen::Vector4d & rhs)
{
  Eigen::Vector4d diff;
  diff.head<3>() = (lhs.head<3>() - rhs.head<3>()) * 1000.0;
  diff(3) = mg400_common::kinematics::rad2deg(
    mg400_common::kinematics::normalize_angle(lhs(3) - rhs(3)));

  std::ostringstream stream;
  stream << std::fixed << std::setprecision(3)
         << "["
         << diff(0) << ", "
         << diff(1) << ", "
         << diff(2) << ", "
         << diff(3) << "]";
  return stream.str();
}

std::string format_joint_diff_deg(const Eigen::Vector4d & lhs, const Eigen::Vector4d & rhs)
{
  Eigen::Vector4d diff;
  diff << mg400_common::kinematics::rad2deg(
    mg400_common::kinematics::normalize_angle(lhs(0) - rhs(0))),
    mg400_common::kinematics::rad2deg(
    mg400_common::kinematics::normalize_angle(lhs(1) - rhs(1))),
    mg400_common::kinematics::rad2deg(
    mg400_common::kinematics::normalize_angle(lhs(2) - rhs(2))),
    mg400_common::kinematics::rad2deg(
    mg400_common::kinematics::normalize_angle(lhs(3) - rhs(3)));

  std::ostringstream stream;
  stream << std::fixed << std::setprecision(3)
         << "["
         << diff(0) << ", "
         << diff(1) << ", "
         << diff(2) << ", "
         << diff(3) << "]";
  return stream.str();
}

double wrapped_angle_distance(double lhs, double rhs)
{
  return std::abs(mg400_common::kinematics::normalize_angle(lhs - rhs));
}

Config parse_config(const std::vector<std::string> & args)
{
  Config config;

  auto require_value = [&args](size_t index, const std::string & option) -> const std::string & {
      if (index + 1 >= args.size()) {
        throw std::invalid_argument("Missing value for " + option);
      }
      return args[index + 1];
    };

  for (size_t index = 0; index < args.size(); ++index) {
    const std::string & arg = args[index];
    if (arg == "--cases") {
      config.case_count = static_cast<size_t>(std::stoul(require_value(index, arg)));
      ++index;
    } else if (arg == "--seed") {
      config.seed = static_cast<unsigned int>(std::stoul(require_value(index, arg)));
      ++index;
    } else if (arg == "--service-wait-timeout-sec") {
      config.service_wait_timeout_sec = std::stod(require_value(index, arg));
      ++index;
    } else if (arg == "--service-call-timeout-sec") {
      config.service_call_timeout_sec = std::stod(require_value(index, arg));
      ++index;
    } else if (arg == "--pose-position-tolerance-m") {
      config.pose_position_tolerance_m = std::stod(require_value(index, arg));
      ++index;
    } else if (arg == "--pose-angle-tolerance-rad") {
      config.pose_angle_tolerance_rad = std::stod(require_value(index, arg));
      ++index;
    } else if (arg == "--joint-tolerance-rad") {
      config.joint_tolerance_rad = std::stod(require_value(index, arg));
      ++index;
    } else if (arg == "--help" || arg == "-h") {
      std::cout
        << "Usage: validate_kinematics_with_solutions [options]\n"
        << "  --cases <N>\n"
        << "  --seed <N>\n"
        << "  --service-wait-timeout-sec <sec>\n"
        << "  --service-call-timeout-sec <sec>\n"
        << "  --pose-position-tolerance-m <m>\n"
        << "  --pose-angle-tolerance-rad <rad>\n"
        << "  --joint-tolerance-rad <rad>\n";
      std::exit(0);
    } else {
      throw std::invalid_argument("Unknown argument: " + arg);
    }
  }

  if (config.case_count == 0) {
    throw std::invalid_argument("--cases must be greater than zero.");
  }
  if (config.service_wait_timeout_sec <= 0.0 || config.service_call_timeout_sec <= 0.0) {
    throw std::invalid_argument("Service timeouts must be greater than zero.");
  }
  if (config.pose_position_tolerance_m < 0.0 || config.pose_angle_tolerance_rad < 0.0 ||
    config.joint_tolerance_rad < 0.0)
  {
    throw std::invalid_argument("Tolerances must be non-negative.");
  }

  return config;
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

  throw std::runtime_error("Failed to generate a valid joint sample.");
}

Eigen::Vector4d to_pose(const PositiveSolution::Response & response)
{
  Eigen::Vector4d pose;
  pose << response.x, response.y, response.z, response.r;
  return pose;
}

Eigen::Vector4d to_joints(const InverseSolution::Response & response)
{
  Eigen::Vector4d joints;
  joints << response.j1, response.j2, response.j3, response.j4;
  return joints;
}

template<typename ServiceT>
typename ServiceT::Response::SharedPtr wait_for_result(
  rclcpp::Node & node,
  const typename rclcpp::Client<ServiceT>::SharedPtr & client,
  const typename ServiceT::Request::SharedPtr & request,
  std::chrono::nanoseconds timeout)
{
  auto future = client->async_send_request(request);
  const auto result = rclcpp::spin_until_future_complete(
    node.get_node_base_interface(), future, timeout);
  if (result != rclcpp::FutureReturnCode::SUCCESS) {
    throw std::runtime_error("Timed out waiting for service response.");
  }
  return future.get();
}

CaseResult run_case(
  rclcpp::Node & node,
  const rclcpp::Client<PositiveSolution>::SharedPtr & positive_client,
  const rclcpp::Client<InverseSolution>::SharedPtr & inverse_client,
  const Config & config,
  size_t case_index,
  const Eigen::Vector4d & reference_joints)
{
  CaseResult result;
  result.index = case_index;
  result.reference_joints = reference_joints;

  const Eigen::Vector4d local_pose =
    mg400_common::kinematics::fk_for_flange_from_origin(reference_joints);
  result.local_pose = local_pose;
  result.local_ik_joints = mg400_common::kinematics::ik_for_flange_from_origin(local_pose);

  auto positive_request = std::make_shared<PositiveSolution::Request>();
  positive_request->j1 = reference_joints(0);
  positive_request->j2 = reference_joints(1);
  positive_request->j3 = reference_joints(2);
  positive_request->j4 = reference_joints(3);
  positive_request->user = 0;
  positive_request->tool = 0;

  const auto positive_response = wait_for_result<PositiveSolution>(
    node, positive_client, positive_request,
    std::chrono::duration_cast<std::chrono::nanoseconds>(
      std::chrono::duration<double>(config.service_call_timeout_sec)));
  result.positive_error_id = positive_response->error_id;
  if (positive_response->error_id != 0) {
    return result;
  }

  result.service_pose = to_pose(*positive_response);
  result.max_pose_position_error_m =
    (local_pose.head<3>() - result.service_pose.head<3>()).cwiseAbs().maxCoeff();
  result.pose_angle_error_rad = wrapped_angle_distance(local_pose(3), result.service_pose(3));

  auto inverse_request = std::make_shared<InverseSolution::Request>();
  inverse_request->x = local_pose(0);
  inverse_request->y = local_pose(1);
  inverse_request->z = local_pose(2);
  inverse_request->r = local_pose(3);
  inverse_request->user = 0;
  inverse_request->tool = 0;

  const auto inverse_response = wait_for_result<InverseSolution>(
    node, inverse_client, inverse_request,
    std::chrono::duration_cast<std::chrono::nanoseconds>(
      std::chrono::duration<double>(config.service_call_timeout_sec)));
  result.inverse_error_id = inverse_response->error_id;
  if (inverse_response->error_id != 0) {
    return result;
  }

  result.service_joints = to_joints(*inverse_response);
  result.max_joint_error_rad = std::max(
    {
      wrapped_angle_distance(reference_joints(0), result.service_joints(0)),
      wrapped_angle_distance(reference_joints(1), result.service_joints(1)),
      wrapped_angle_distance(reference_joints(2), result.service_joints(2)),
      wrapped_angle_distance(reference_joints(3), result.service_joints(3))
    });

  result.passed =
    result.max_pose_position_error_m <= config.pose_position_tolerance_m &&
    result.pose_angle_error_rad <= config.pose_angle_tolerance_rad &&
    result.max_joint_error_rad <= config.joint_tolerance_rad;

  return result;
}

void print_case_result(const CaseResult & result)
{
  std::ostringstream label_stream;
  label_stream << "case_" << std::setw(2) << std::setfill('0') << result.index;
  const std::string case_label = label_stream.str();

  std::cout
    << (result.passed ? "PASS" : "FAIL")
    << " " << std::left << std::setw(18) << case_label << std::right
    << " diff=" << std::fixed << std::setprecision(6)
    << mg400_common::kinematics::rad2deg(result.max_joint_error_rad) << "deg"
    << " pos_err=" << result.positive_error_id
    << " inv_err=" << result.inverse_error_id
    << "\n";
  std::cout
    << "  LocalFK          : " << format_joint_vector_deg(result.reference_joints)
    << " -> " << format_pose_vector_mm_deg(result.local_pose)
    << "\n";
  std::cout
    << "  PositiveSolution: " << format_joint_vector_deg(result.reference_joints)
    << " -> " << format_pose_vector_mm_deg(result.service_pose)
    << "\n";
  std::cout
    << "  FK diff          : "
    << format_pose_diff_mm_deg(result.service_pose, result.local_pose)
    << "\n";
  std::cout
    << "  LocalIK          : " << format_joint_vector_deg(result.local_ik_joints)
    << " <- " << format_pose_vector_mm_deg(result.local_pose)
    << "\n";
  std::cout
    << "  InverseSolution : " << format_joint_vector_deg(result.service_joints)
    << " <- " << format_pose_vector_mm_deg(result.local_pose)
    << "\n";
  std::cout
    << "  IK diff          : "
    << format_joint_diff_deg(result.service_joints, result.local_ik_joints)
    << "\n";
}

}  // namespace

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);

  try {
    const std::vector<std::string> args =
      rclcpp::remove_ros_arguments(argc, argv);
    const std::vector<std::string> program_args(args.begin() + 1, args.end());
    const Config config = parse_config(program_args);

    auto node = std::make_shared<rclcpp::Node>("validate_kinematics_with_solutions");
    const auto positive_client = node->create_client<PositiveSolution>("positive_solution");
    const auto inverse_client = node->create_client<InverseSolution>("inverse_solution");

    const auto service_wait_timeout = std::chrono::duration_cast<std::chrono::nanoseconds>(
      std::chrono::duration<double>(config.service_wait_timeout_sec));
    if (!positive_client->wait_for_service(service_wait_timeout)) {
      throw std::runtime_error("Timed out waiting for positive_solution service.");
    }
    if (!inverse_client->wait_for_service(service_wait_timeout)) {
      throw std::runtime_error("Timed out waiting for inverse_solution service.");
    }

    std::mt19937 generator(config.seed);
    size_t passed_count = 0;

    std::cout << std::fixed << std::setprecision(6);
    std::cout
      << "threshold: fk_pos=" << config.pose_position_tolerance_m * 1000.0 << "mm"
      << " fk_yaw=" << mg400_common::kinematics::rad2deg(config.pose_angle_tolerance_rad)
      << "deg ik=" << mg400_common::kinematics::rad2deg(config.joint_tolerance_rad)
      << "deg\n";
    std::cout << "Running " << config.case_count
              << " validation cases with seed=" << config.seed << "\n";

    for (size_t case_index = 1; case_index <= config.case_count; ++case_index) {
      const Eigen::Vector4d joints = generate_valid_joints(generator);
      const CaseResult result = run_case(
        *node, positive_client, inverse_client, config, case_index, joints);

      print_case_result(result);
      if (result.passed) {
        ++passed_count;
      }
    }

    std::cout
      << "\nSummary\n"
      << "  cases=" << config.case_count
      << " pass=" << passed_count
      << " fail=" << (config.case_count - passed_count) << "\n";

    rclcpp::shutdown();
    return passed_count == config.case_count ? 0 : 1;
  } catch (const std::exception & ex) {
    std::cerr << ex.what() << "\n";
    rclcpp::shutdown();
    return 1;
  }
}
