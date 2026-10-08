// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:msg/MovLIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_LIO__TRAITS_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_LIO__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/msg/detail/mov_lio__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose_stamped__traits.hpp"
// Member 'mode'
#include "mg400_msgs/msg/detail/distance_mode__traits.hpp"
// Member 'index'
#include "mg400_msgs/msg/detail/do_index__traits.hpp"
// Member 'status'
#include "mg400_msgs/msg/detail/do_status__traits.hpp"

namespace mg400_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const MovLIO & msg,
  std::ostream & out)
{
  out << "{";
  // member: pose
  {
    out << "pose: ";
    to_flow_style_yaml(msg.pose, out);
    out << ", ";
  }

  // member: mode
  {
    out << "mode: ";
    to_flow_style_yaml(msg.mode, out);
    out << ", ";
  }

  // member: distance
  {
    out << "distance: ";
    rosidl_generator_traits::value_to_yaml(msg.distance, out);
    out << ", ";
  }

  // member: index
  {
    out << "index: ";
    to_flow_style_yaml(msg.index, out);
    out << ", ";
  }

  // member: status
  {
    out << "status: ";
    to_flow_style_yaml(msg.status, out);
    out << ", ";
  }

  // member: set_speed_l
  {
    out << "set_speed_l: ";
    rosidl_generator_traits::value_to_yaml(msg.set_speed_l, out);
    out << ", ";
  }

  // member: speed_l
  {
    out << "speed_l: ";
    rosidl_generator_traits::value_to_yaml(msg.speed_l, out);
    out << ", ";
  }

  // member: set_acc_l
  {
    out << "set_acc_l: ";
    rosidl_generator_traits::value_to_yaml(msg.set_acc_l, out);
    out << ", ";
  }

  // member: acc_l
  {
    out << "acc_l: ";
    rosidl_generator_traits::value_to_yaml(msg.acc_l, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovLIO & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: pose
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose:\n";
    to_block_style_yaml(msg.pose, out, indentation + 2);
  }

  // member: mode
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "mode:\n";
    to_block_style_yaml(msg.mode, out, indentation + 2);
  }

  // member: distance
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "distance: ";
    rosidl_generator_traits::value_to_yaml(msg.distance, out);
    out << "\n";
  }

  // member: index
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "index:\n";
    to_block_style_yaml(msg.index, out, indentation + 2);
  }

  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status:\n";
    to_block_style_yaml(msg.status, out, indentation + 2);
  }

  // member: set_speed_l
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "set_speed_l: ";
    rosidl_generator_traits::value_to_yaml(msg.set_speed_l, out);
    out << "\n";
  }

  // member: speed_l
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "speed_l: ";
    rosidl_generator_traits::value_to_yaml(msg.speed_l, out);
    out << "\n";
  }

  // member: set_acc_l
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "set_acc_l: ";
    rosidl_generator_traits::value_to_yaml(msg.set_acc_l, out);
    out << "\n";
  }

  // member: acc_l
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "acc_l: ";
    rosidl_generator_traits::value_to_yaml(msg.acc_l, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovLIO & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::msg::MovLIO & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::msg::MovLIO & msg)
{
  return mg400_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::msg::MovLIO>()
{
  return "mg400_msgs::msg::MovLIO";
}

template<>
inline const char * name<mg400_msgs::msg::MovLIO>()
{
  return "mg400_msgs/msg/MovLIO";
}

template<>
struct has_fixed_size<mg400_msgs::msg::MovLIO>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::PoseStamped>::value && has_fixed_size<mg400_msgs::msg::DOIndex>::value && has_fixed_size<mg400_msgs::msg::DOStatus>::value && has_fixed_size<mg400_msgs::msg::DistanceMode>::value> {};

template<>
struct has_bounded_size<mg400_msgs::msg::MovLIO>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::PoseStamped>::value && has_bounded_size<mg400_msgs::msg::DOIndex>::value && has_bounded_size<mg400_msgs::msg::DOStatus>::value && has_bounded_size<mg400_msgs::msg::DistanceMode>::value> {};

template<>
struct is_message<mg400_msgs::msg::MovLIO>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__MSG__DETAIL__MOV_LIO__TRAITS_HPP_
