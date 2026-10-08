// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:srv/RobotMode.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__ROBOT_MODE__TRAITS_HPP_
#define MG400_MSGS__SRV__DETAIL__ROBOT_MODE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/srv/detail/robot_mode__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const RobotMode_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RobotMode_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RobotMode_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::srv::RobotMode_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::RobotMode_Request & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::RobotMode_Request>()
{
  return "mg400_msgs::srv::RobotMode_Request";
}

template<>
inline const char * name<mg400_msgs::srv::RobotMode_Request>()
{
  return "mg400_msgs/srv/RobotMode_Request";
}

template<>
struct has_fixed_size<mg400_msgs::srv::RobotMode_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::RobotMode_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::RobotMode_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'robot_mode'
#include "mg400_msgs/msg/detail/robot_mode__traits.hpp"

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const RobotMode_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: robot_mode
  {
    out << "robot_mode: ";
    to_flow_style_yaml(msg.robot_mode, out);
    out << ", ";
  }

  // member: error_id
  {
    out << "error_id: ";
    rosidl_generator_traits::value_to_yaml(msg.error_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RobotMode_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: robot_mode
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "robot_mode:\n";
    to_block_style_yaml(msg.robot_mode, out, indentation + 2);
  }

  // member: error_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "error_id: ";
    rosidl_generator_traits::value_to_yaml(msg.error_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RobotMode_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::srv::RobotMode_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::RobotMode_Response & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::RobotMode_Response>()
{
  return "mg400_msgs::srv::RobotMode_Response";
}

template<>
inline const char * name<mg400_msgs::srv::RobotMode_Response>()
{
  return "mg400_msgs/srv/RobotMode_Response";
}

template<>
struct has_fixed_size<mg400_msgs::srv::RobotMode_Response>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::msg::RobotMode>::value> {};

template<>
struct has_bounded_size<mg400_msgs::srv::RobotMode_Response>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::msg::RobotMode>::value> {};

template<>
struct is_message<mg400_msgs::srv::RobotMode_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::srv::RobotMode>()
{
  return "mg400_msgs::srv::RobotMode";
}

template<>
inline const char * name<mg400_msgs::srv::RobotMode>()
{
  return "mg400_msgs/srv/RobotMode";
}

template<>
struct has_fixed_size<mg400_msgs::srv::RobotMode>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::srv::RobotMode_Request>::value &&
    has_fixed_size<mg400_msgs::srv::RobotMode_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::srv::RobotMode>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::srv::RobotMode_Request>::value &&
    has_bounded_size<mg400_msgs::srv::RobotMode_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::srv::RobotMode>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::srv::RobotMode_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::srv::RobotMode_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__SRV__DETAIL__ROBOT_MODE__TRAITS_HPP_
