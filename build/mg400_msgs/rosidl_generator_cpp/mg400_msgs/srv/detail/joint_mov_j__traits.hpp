// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:srv/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__TRAITS_HPP_
#define MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/srv/detail/joint_mov_j__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const JointMovJ_Request & msg,
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

  // member: j5
  {
    out << "j5: ";
    rosidl_generator_traits::value_to_yaml(msg.j5, out);
    out << ", ";
  }

  // member: j6
  {
    out << "j6: ";
    rosidl_generator_traits::value_to_yaml(msg.j6, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const JointMovJ_Request & msg,
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

  // member: j5
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "j5: ";
    rosidl_generator_traits::value_to_yaml(msg.j5, out);
    out << "\n";
  }

  // member: j6
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "j6: ";
    rosidl_generator_traits::value_to_yaml(msg.j6, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const JointMovJ_Request & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::JointMovJ_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::JointMovJ_Request & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::JointMovJ_Request>()
{
  return "mg400_msgs::srv::JointMovJ_Request";
}

template<>
inline const char * name<mg400_msgs::srv::JointMovJ_Request>()
{
  return "mg400_msgs/srv/JointMovJ_Request";
}

template<>
struct has_fixed_size<mg400_msgs::srv::JointMovJ_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::JointMovJ_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::JointMovJ_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace mg400_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const JointMovJ_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: error_id
  {
    out << "error_id: ";
    rosidl_generator_traits::value_to_yaml(msg.error_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const JointMovJ_Response & msg,
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
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const JointMovJ_Response & msg, bool use_flow_style = false)
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
  const mg400_msgs::srv::JointMovJ_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::srv::JointMovJ_Response & msg)
{
  return mg400_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::srv::JointMovJ_Response>()
{
  return "mg400_msgs::srv::JointMovJ_Response";
}

template<>
inline const char * name<mg400_msgs::srv::JointMovJ_Response>()
{
  return "mg400_msgs/srv/JointMovJ_Response";
}

template<>
struct has_fixed_size<mg400_msgs::srv::JointMovJ_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<mg400_msgs::srv::JointMovJ_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<mg400_msgs::srv::JointMovJ_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::srv::JointMovJ>()
{
  return "mg400_msgs::srv::JointMovJ";
}

template<>
inline const char * name<mg400_msgs::srv::JointMovJ>()
{
  return "mg400_msgs/srv/JointMovJ";
}

template<>
struct has_fixed_size<mg400_msgs::srv::JointMovJ>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::srv::JointMovJ_Request>::value &&
    has_fixed_size<mg400_msgs::srv::JointMovJ_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::srv::JointMovJ>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::srv::JointMovJ_Request>::value &&
    has_bounded_size<mg400_msgs::srv::JointMovJ_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::srv::JointMovJ>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::srv::JointMovJ_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::srv::JointMovJ_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__TRAITS_HPP_
