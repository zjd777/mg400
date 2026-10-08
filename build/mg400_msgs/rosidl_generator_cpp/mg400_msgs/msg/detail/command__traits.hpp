// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COMMAND__TRAITS_HPP_
#define MG400_MSGS__MSG__DETAIL__COMMAND__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/msg/detail/command__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'mov_j_params'
#include "mg400_msgs/msg/detail/mov_j__traits.hpp"
// Member 'mov_l_params'
#include "mg400_msgs/msg/detail/mov_l__traits.hpp"
// Member 'joint_mov_j_params'
#include "mg400_msgs/msg/detail/joint_mov_j__traits.hpp"
// Member 'mov_jio_params'
#include "mg400_msgs/msg/detail/mov_jio__traits.hpp"
// Member 'mov_lio_params'
#include "mg400_msgs/msg/detail/mov_lio__traits.hpp"

namespace mg400_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const Command & msg,
  std::ostream & out)
{
  out << "{";
  // member: command_type
  {
    out << "command_type: ";
    rosidl_generator_traits::value_to_yaml(msg.command_type, out);
    out << ", ";
  }

  // member: mov_j_params
  {
    out << "mov_j_params: ";
    to_flow_style_yaml(msg.mov_j_params, out);
    out << ", ";
  }

  // member: mov_l_params
  {
    out << "mov_l_params: ";
    to_flow_style_yaml(msg.mov_l_params, out);
    out << ", ";
  }

  // member: joint_mov_j_params
  {
    out << "joint_mov_j_params: ";
    to_flow_style_yaml(msg.joint_mov_j_params, out);
    out << ", ";
  }

  // member: mov_jio_params
  {
    out << "mov_jio_params: ";
    to_flow_style_yaml(msg.mov_jio_params, out);
    out << ", ";
  }

  // member: mov_lio_params
  {
    out << "mov_lio_params: ";
    to_flow_style_yaml(msg.mov_lio_params, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Command & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: command_type
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "command_type: ";
    rosidl_generator_traits::value_to_yaml(msg.command_type, out);
    out << "\n";
  }

  // member: mov_j_params
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "mov_j_params:\n";
    to_block_style_yaml(msg.mov_j_params, out, indentation + 2);
  }

  // member: mov_l_params
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "mov_l_params:\n";
    to_block_style_yaml(msg.mov_l_params, out, indentation + 2);
  }

  // member: joint_mov_j_params
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "joint_mov_j_params:\n";
    to_block_style_yaml(msg.joint_mov_j_params, out, indentation + 2);
  }

  // member: mov_jio_params
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "mov_jio_params:\n";
    to_block_style_yaml(msg.mov_jio_params, out, indentation + 2);
  }

  // member: mov_lio_params
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "mov_lio_params:\n";
    to_block_style_yaml(msg.mov_lio_params, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Command & msg, bool use_flow_style = false)
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
  const mg400_msgs::msg::Command & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::msg::Command & msg)
{
  return mg400_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::msg::Command>()
{
  return "mg400_msgs::msg::Command";
}

template<>
inline const char * name<mg400_msgs::msg::Command>()
{
  return "mg400_msgs/msg/Command";
}

template<>
struct has_fixed_size<mg400_msgs::msg::Command>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::msg::JointMovJ>::value && has_fixed_size<mg400_msgs::msg::MovJ>::value && has_fixed_size<mg400_msgs::msg::MovJIO>::value && has_fixed_size<mg400_msgs::msg::MovL>::value && has_fixed_size<mg400_msgs::msg::MovLIO>::value> {};

template<>
struct has_bounded_size<mg400_msgs::msg::Command>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::msg::JointMovJ>::value && has_bounded_size<mg400_msgs::msg::MovJ>::value && has_bounded_size<mg400_msgs::msg::MovJIO>::value && has_bounded_size<mg400_msgs::msg::MovL>::value && has_bounded_size<mg400_msgs::msg::MovLIO>::value> {};

template<>
struct is_message<mg400_msgs::msg::Command>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__MSG__DETAIL__COMMAND__TRAITS_HPP_
