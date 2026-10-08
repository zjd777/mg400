// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ERROR_ID__TRAITS_HPP_
#define MG400_MSGS__MSG__DETAIL__ERROR_ID__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/msg/detail/error_id__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'controller'
// Member 'servo'
#include "mg400_msgs/msg/detail/id_array__traits.hpp"

namespace mg400_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const ErrorID & msg,
  std::ostream & out)
{
  out << "{";
  // member: controller
  {
    out << "controller: ";
    to_flow_style_yaml(msg.controller, out);
    out << ", ";
  }

  // member: servo
  {
    if (msg.servo.size() == 0) {
      out << "servo: []";
    } else {
      out << "servo: [";
      size_t pending_items = msg.servo.size();
      for (auto item : msg.servo) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ErrorID & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: controller
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "controller:\n";
    to_block_style_yaml(msg.controller, out, indentation + 2);
  }

  // member: servo
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.servo.size() == 0) {
      out << "servo: []\n";
    } else {
      out << "servo:\n";
      for (auto item : msg.servo) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ErrorID & msg, bool use_flow_style = false)
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
  const mg400_msgs::msg::ErrorID & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::msg::ErrorID & msg)
{
  return mg400_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::msg::ErrorID>()
{
  return "mg400_msgs::msg::ErrorID";
}

template<>
inline const char * name<mg400_msgs::msg::ErrorID>()
{
  return "mg400_msgs/msg/ErrorID";
}

template<>
struct has_fixed_size<mg400_msgs::msg::ErrorID>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::msg::IDArray>::value> {};

template<>
struct has_bounded_size<mg400_msgs::msg::ErrorID>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::msg::IDArray>::value> {};

template<>
struct is_message<mg400_msgs::msg::ErrorID>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__MSG__DETAIL__ERROR_ID__TRAITS_HPP_
