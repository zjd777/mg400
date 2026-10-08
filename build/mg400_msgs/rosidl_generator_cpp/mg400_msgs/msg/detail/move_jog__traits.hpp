// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:msg/MoveJog.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOVE_JOG__TRAITS_HPP_
#define MG400_MSGS__MSG__DETAIL__MOVE_JOG__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/msg/detail/move_jog__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const MoveJog & msg,
  std::ostream & out)
{
  out << "{";
  // member: jog_mode
  {
    out << "jog_mode: ";
    rosidl_generator_traits::value_to_yaml(msg.jog_mode, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveJog & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: jog_mode
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "jog_mode: ";
    rosidl_generator_traits::value_to_yaml(msg.jog_mode, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveJog & msg, bool use_flow_style = false)
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
  const mg400_msgs::msg::MoveJog & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::msg::MoveJog & msg)
{
  return mg400_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::msg::MoveJog>()
{
  return "mg400_msgs::msg::MoveJog";
}

template<>
inline const char * name<mg400_msgs::msg::MoveJog>()
{
  return "mg400_msgs/msg/MoveJog";
}

template<>
struct has_fixed_size<mg400_msgs::msg::MoveJog>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<mg400_msgs::msg::MoveJog>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<mg400_msgs::msg::MoveJog>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__MSG__DETAIL__MOVE_JOG__TRAITS_HPP_
