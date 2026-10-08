// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__TRAITS_HPP_
#define MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/msg/detail/joint_mov_j__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const JointMovJ & msg,
  std::ostream & out)
{
  out << "{";
  // member: joint_angles
  {
    if (msg.joint_angles.size() == 0) {
      out << "joint_angles: []";
    } else {
      out << "joint_angles: [";
      size_t pending_items = msg.joint_angles.size();
      for (auto item : msg.joint_angles) {
        rosidl_generator_traits::value_to_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: set_speed_j
  {
    out << "set_speed_j: ";
    rosidl_generator_traits::value_to_yaml(msg.set_speed_j, out);
    out << ", ";
  }

  // member: speed_j
  {
    out << "speed_j: ";
    rosidl_generator_traits::value_to_yaml(msg.speed_j, out);
    out << ", ";
  }

  // member: set_acc_j
  {
    out << "set_acc_j: ";
    rosidl_generator_traits::value_to_yaml(msg.set_acc_j, out);
    out << ", ";
  }

  // member: acc_j
  {
    out << "acc_j: ";
    rosidl_generator_traits::value_to_yaml(msg.acc_j, out);
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
  const JointMovJ & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: joint_angles
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.joint_angles.size() == 0) {
      out << "joint_angles: []\n";
    } else {
      out << "joint_angles:\n";
      for (auto item : msg.joint_angles) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }

  // member: set_speed_j
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "set_speed_j: ";
    rosidl_generator_traits::value_to_yaml(msg.set_speed_j, out);
    out << "\n";
  }

  // member: speed_j
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "speed_j: ";
    rosidl_generator_traits::value_to_yaml(msg.speed_j, out);
    out << "\n";
  }

  // member: set_acc_j
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "set_acc_j: ";
    rosidl_generator_traits::value_to_yaml(msg.set_acc_j, out);
    out << "\n";
  }

  // member: acc_j
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "acc_j: ";
    rosidl_generator_traits::value_to_yaml(msg.acc_j, out);
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

inline std::string to_yaml(const JointMovJ & msg, bool use_flow_style = false)
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
  const mg400_msgs::msg::JointMovJ & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::msg::JointMovJ & msg)
{
  return mg400_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::msg::JointMovJ>()
{
  return "mg400_msgs::msg::JointMovJ";
}

template<>
inline const char * name<mg400_msgs::msg::JointMovJ>()
{
  return "mg400_msgs/msg/JointMovJ";
}

template<>
struct has_fixed_size<mg400_msgs::msg::JointMovJ>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::msg::JointMovJ>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::msg::JointMovJ>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__TRAITS_HPP_
