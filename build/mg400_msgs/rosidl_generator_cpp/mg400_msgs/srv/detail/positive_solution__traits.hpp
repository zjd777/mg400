// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:srv/PositiveSolution.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__TRAITS_HPP_
#define MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/srv/detail/positive_solution__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const PositiveSolution_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: j1
  {
    out << "j1: ";
    rosidl_generator_traits::value_to_yaml(msg.j1, out);
    out << ", ";
  }

  // member: j2
  {
    out << "j2: ";
    rosidl_generator_traits::value_to_yaml(msg.j2, out);
    out << ", ";
  }

  // member: j3
  {
    out << "j3: ";
    rosidl_generator_traits::value_to_yaml(msg.j3, out);
    out << ", ";
  }

  // member: j4
  {
    out << "j4: ";
    rosidl_generator_traits::value_to_yaml(msg.j4, out);
    out << ", ";
  }

  // member: user
  {
    out << "user: ";
    rosidl_generator_traits::value_to_yaml(msg.user, out);
    out << ", ";
  }

  // member: tool
  {
    out << "tool: ";
    rosidl_generator_traits::value_to_yaml(msg.tool, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PositiveSolution_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: j1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "j1: ";
    rosidl_generator_traits::value_to_yaml(msg.j1, out);
    out << "\n";
  }

  // member: j2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "j2: ";
    rosidl_generator_traits::value_to_yaml(msg.j2, out);
    out << "\n";
  }

  // member: j3
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "j3: ";
    rosidl_generator_traits::value_to_yaml(msg.j3, out);
    out << "\n";
  }

  // member: j4
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "j4: ";
    rosidl_generator_traits::value_to_yaml(msg.j4, out);
    out << "\n";
  }

  // member: user
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "user: ";
    rosidl_generator_traits::value_to_yaml(msg.user, out);
    out << "\n";
  }

  // member: tool
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "tool: ";
    rosidl_generator_traits::value_to_yaml(msg.tool, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PositiveSolution_Request & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::PositiveSolution_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::PositiveSolution_Request & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::PositiveSolution_Request>()
{
  return "mg400_msgs::srv::PositiveSolution_Request";
}

template<>
inline const char * name<mg400_msgs::srv::PositiveSolution_Request>()
{
  return "mg400_msgs/srv/PositiveSolution_Request";
}

template<>
struct has_fixed_size<mg400_msgs::srv::PositiveSolution_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::PositiveSolution_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::PositiveSolution_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const PositiveSolution_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: error_id
  {
    out << "error_id: ";
    rosidl_generator_traits::value_to_yaml(msg.error_id, out);
    out << ", ";
  }

  // member: x
  {
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << ", ";
  }

  // member: y
  {
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << ", ";
  }

  // member: z
  {
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << ", ";
  }

  // member: r
  {
    out << "r: ";
    rosidl_generator_traits::value_to_yaml(msg.r, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PositiveSolution_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: error_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "error_id: ";
    rosidl_generator_traits::value_to_yaml(msg.error_id, out);
    out << "\n";
  }

  // member: x
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "x: ";
    rosidl_generator_traits::value_to_yaml(msg.x, out);
    out << "\n";
  }

  // member: y
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "y: ";
    rosidl_generator_traits::value_to_yaml(msg.y, out);
    out << "\n";
  }

  // member: z
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "z: ";
    rosidl_generator_traits::value_to_yaml(msg.z, out);
    out << "\n";
  }

  // member: r
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "r: ";
    rosidl_generator_traits::value_to_yaml(msg.r, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PositiveSolution_Response & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::PositiveSolution_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::PositiveSolution_Response & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::PositiveSolution_Response>()
{
  return "mg400_msgs::srv::PositiveSolution_Response";
}

template<>
inline const char * name<mg400_msgs::srv::PositiveSolution_Response>()
{
  return "mg400_msgs/srv/PositiveSolution_Response";
}

template<>
struct has_fixed_size<mg400_msgs::srv::PositiveSolution_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::PositiveSolution_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::PositiveSolution_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::srv::PositiveSolution>()
{
  return "mg400_msgs::srv::PositiveSolution";
}

template<>
inline const char * name<mg400_msgs::srv::PositiveSolution>()
{
  return "mg400_msgs/srv/PositiveSolution";
}

template<>
struct has_fixed_size<mg400_msgs::srv::PositiveSolution>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::srv::PositiveSolution_Request>::value &&
    has_fixed_size<mg400_msgs::srv::PositiveSolution_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::srv::PositiveSolution>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::srv::PositiveSolution_Request>::value &&
    has_bounded_size<mg400_msgs::srv::PositiveSolution_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::srv::PositiveSolution>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::srv::PositiveSolution_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::srv::PositiveSolution_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__TRAITS_HPP_
