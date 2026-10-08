// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:srv/GetPose.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__GET_POSE__TRAITS_HPP_
#define MG400_MSGS__SRV__DETAIL__GET_POSE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/srv/detail/get_pose__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetPose_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetPose_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetPose_Request & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::GetPose_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::GetPose_Request & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::GetPose_Request>()
{
  return "mg400_msgs::srv::GetPose_Request";
}

template<>
inline const char * name<mg400_msgs::srv::GetPose_Request>()
{
  return "mg400_msgs/srv/GetPose_Request";
}

template<>
struct has_fixed_size<mg400_msgs::srv::GetPose_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::GetPose_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::GetPose_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetPose_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: error_id
  {
    out << "error_id: ";
    rosidl_generator_traits::value_to_yaml(msg.error_id, out);
    out << ", ";
  }

  // member: pose1
  {
    out << "pose1: ";
    rosidl_generator_traits::value_to_yaml(msg.pose1, out);
    out << ", ";
  }

  // member: pose2
  {
    out << "pose2: ";
    rosidl_generator_traits::value_to_yaml(msg.pose2, out);
    out << ", ";
  }

  // member: pose3
  {
    out << "pose3: ";
    rosidl_generator_traits::value_to_yaml(msg.pose3, out);
    out << ", ";
  }

  // member: pose4
  {
    out << "pose4: ";
    rosidl_generator_traits::value_to_yaml(msg.pose4, out);
    out << ", ";
  }

  // member: pose5
  {
    out << "pose5: ";
    rosidl_generator_traits::value_to_yaml(msg.pose5, out);
    out << ", ";
  }

  // member: pose6
  {
    out << "pose6: ";
    rosidl_generator_traits::value_to_yaml(msg.pose6, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetPose_Response & msg,
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

  // member: pose1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose1: ";
    rosidl_generator_traits::value_to_yaml(msg.pose1, out);
    out << "\n";
  }

  // member: pose2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose2: ";
    rosidl_generator_traits::value_to_yaml(msg.pose2, out);
    out << "\n";
  }

  // member: pose3
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose3: ";
    rosidl_generator_traits::value_to_yaml(msg.pose3, out);
    out << "\n";
  }

  // member: pose4
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose4: ";
    rosidl_generator_traits::value_to_yaml(msg.pose4, out);
    out << "\n";
  }

  // member: pose5
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose5: ";
    rosidl_generator_traits::value_to_yaml(msg.pose5, out);
    out << "\n";
  }

  // member: pose6
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose6: ";
    rosidl_generator_traits::value_to_yaml(msg.pose6, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetPose_Response & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::GetPose_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::GetPose_Response & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::GetPose_Response>()
{
  return "mg400_msgs::srv::GetPose_Response";
}

template<>
inline const char * name<mg400_msgs::srv::GetPose_Response>()
{
  return "mg400_msgs/srv/GetPose_Response";
}

template<>
struct has_fixed_size<mg400_msgs::srv::GetPose_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::GetPose_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::GetPose_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::srv::GetPose>()
{
  return "mg400_msgs::srv::GetPose";
}

template<>
inline const char * name<mg400_msgs::srv::GetPose>()
{
  return "mg400_msgs/srv/GetPose";
}

template<>
struct has_fixed_size<mg400_msgs::srv::GetPose>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::srv::GetPose_Request>::value &&
    has_fixed_size<mg400_msgs::srv::GetPose_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::srv::GetPose>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::srv::GetPose_Request>::value &&
    has_bounded_size<mg400_msgs::srv::GetPose_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::srv::GetPose>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::srv::GetPose_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::srv::GetPose_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__SRV__DETAIL__GET_POSE__TRAITS_HPP_
