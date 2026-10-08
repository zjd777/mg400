// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:msg/MovL.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_L__TRAITS_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_L__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/msg/detail/mov_l__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose_stamped__traits.hpp"

namespace mg400_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const MovL & msg,
  std::ostream & out)
{
  out << "{";
  // member: pose
  {
    out << "pose: ";
    to_flow_style_yaml(msg.pose, out);
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
    out << ", ";
  }

  // member: set_cp
  {
    out << "set_cp: ";
    rosidl_generator_traits::value_to_yaml(msg.set_cp, out);
    out << ", ";
  }

  // member: cp
  {
    out << "cp: ";
    rosidl_generator_traits::value_to_yaml(msg.cp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovL & msg,
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

  // member: set_cp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "set_cp: ";
    rosidl_generator_traits::value_to_yaml(msg.set_cp, out);
    out << "\n";
  }

  // member: cp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "cp: ";
    rosidl_generator_traits::value_to_yaml(msg.cp, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovL & msg, bool use_flow_style = false)
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
  const mg400_msgs::msg::MovL & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::msg::MovL & msg)
{
  return mg400_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::msg::MovL>()
{
  return "mg400_msgs::msg::MovL";
}

template<>
inline const char * name<mg400_msgs::msg::MovL>()
{
  return "mg400_msgs/msg/MovL";
}

template<>
struct has_fixed_size<mg400_msgs::msg::MovL>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct has_bounded_size<mg400_msgs::msg::MovL>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct is_message<mg400_msgs::msg::MovL>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__MSG__DETAIL__MOV_L__TRAITS_HPP_
