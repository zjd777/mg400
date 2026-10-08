// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:srv/PayLoad.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__PAY_LOAD__TRAITS_HPP_
#define MG400_MSGS__SRV__DETAIL__PAY_LOAD__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/srv/detail/pay_load__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const PayLoad_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: weight
  {
    out << "weight: ";
    rosidl_generator_traits::value_to_yaml(msg.weight, out);
    out << ", ";
  }

  // member: inertia
  {
    out << "inertia: ";
    rosidl_generator_traits::value_to_yaml(msg.inertia, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const PayLoad_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: weight
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "weight: ";
    rosidl_generator_traits::value_to_yaml(msg.weight, out);
    out << "\n";
  }

  // member: inertia
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "inertia: ";
    rosidl_generator_traits::value_to_yaml(msg.inertia, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const PayLoad_Request & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::PayLoad_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::PayLoad_Request & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::PayLoad_Request>()
{
  return "mg400_msgs::srv::PayLoad_Request";
}

template<>
inline const char * name<mg400_msgs::srv::PayLoad_Request>()
{
  return "mg400_msgs/srv/PayLoad_Request";
}

template<>
struct has_fixed_size<mg400_msgs::srv::PayLoad_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::PayLoad_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::PayLoad_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const PayLoad_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: result
  {
    out << "result: ";
    rosidl_generator_traits::value_to_yaml(msg.result, out);
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
  const PayLoad_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result: ";
    rosidl_generator_traits::value_to_yaml(msg.result, out);
    out << "\n";
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

inline std::string to_yaml(const PayLoad_Response & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::PayLoad_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::PayLoad_Response & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::PayLoad_Response>()
{
  return "mg400_msgs::srv::PayLoad_Response";
}

template<>
inline const char * name<mg400_msgs::srv::PayLoad_Response>()
{
  return "mg400_msgs/srv/PayLoad_Response";
}

template<>
struct has_fixed_size<mg400_msgs::srv::PayLoad_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::PayLoad_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::PayLoad_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::srv::PayLoad>()
{
  return "mg400_msgs::srv::PayLoad";
}

template<>
inline const char * name<mg400_msgs::srv::PayLoad>()
{
  return "mg400_msgs/srv/PayLoad";
}

template<>
struct has_fixed_size<mg400_msgs::srv::PayLoad>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::srv::PayLoad_Request>::value &&
    has_fixed_size<mg400_msgs::srv::PayLoad_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::srv::PayLoad>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::srv::PayLoad_Request>::value &&
    has_bounded_size<mg400_msgs::srv::PayLoad_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::srv::PayLoad>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::srv::PayLoad_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::srv::PayLoad_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__SRV__DETAIL__PAY_LOAD__TRAITS_HPP_
