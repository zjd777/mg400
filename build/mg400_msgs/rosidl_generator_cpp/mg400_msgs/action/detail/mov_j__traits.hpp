// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:action/MovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__MOV_J__TRAITS_HPP_
#define MG400_MSGS__ACTION__DETAIL__MOV_J__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/action/detail/mov_j__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose_stamped__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: pose
  {
    out << "pose: ";
    to_flow_style_yaml(msg.pose, out);
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
  const MovJ_Goal & msg,
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

inline std::string to_yaml(const MovJ_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_Goal & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_Goal>()
{
  return "mg400_msgs::action::MovJ_Goal";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_Goal>()
{
  return "mg400_msgs/action/MovJ_Goal";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_Goal>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_Goal>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'error_id'
#include "mg400_msgs/msg/detail/error_id__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_Result & msg,
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
    to_flow_style_yaml(msg.error_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_Result & msg,
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
    out << "error_id:\n";
    to_block_style_yaml(msg.error_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_Result & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_Result>()
{
  return "mg400_msgs::action::MovJ_Result";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_Result>()
{
  return "mg400_msgs/action/MovJ_Result";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_Result>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::msg::ErrorID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_Result>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::msg::ErrorID>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'current_pose'
// already included above
// #include "geometry_msgs/msg/detail/pose_stamped__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: current_pose
  {
    out << "current_pose: ";
    to_flow_style_yaml(msg.current_pose, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: current_pose
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "current_pose:\n";
    to_block_style_yaml(msg.current_pose, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_Feedback & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_Feedback>()
{
  return "mg400_msgs::action::MovJ_Feedback";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_Feedback>()
{
  return "mg400_msgs/action/MovJ_Feedback";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_Feedback>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_Feedback>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "mg400_msgs/action/detail/mov_j__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_SendGoal_Request & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_SendGoal_Request>()
{
  return "mg400_msgs::action::MovJ_SendGoal_Request";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_SendGoal_Request>()
{
  return "mg400_msgs/action/MovJ_SendGoal_Request";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::action::MovJ_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::action::MovJ_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_SendGoal_Response & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_SendGoal_Response>()
{
  return "mg400_msgs::action::MovJ_SendGoal_Response";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_SendGoal_Response>()
{
  return "mg400_msgs/action/MovJ_SendGoal_Response";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::action::MovJ_SendGoal>()
{
  return "mg400_msgs::action::MovJ_SendGoal";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_SendGoal>()
{
  return "mg400_msgs/action/MovJ_SendGoal";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::action::MovJ_SendGoal_Request>::value &&
    has_fixed_size<mg400_msgs::action::MovJ_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::action::MovJ_SendGoal_Request>::value &&
    has_bounded_size<mg400_msgs::action::MovJ_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::action::MovJ_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::action::MovJ_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::action::MovJ_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_GetResult_Request & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_GetResult_Request>()
{
  return "mg400_msgs::action::MovJ_GetResult_Request";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_GetResult_Request>()
{
  return "mg400_msgs/action/MovJ_GetResult_Request";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "mg400_msgs/action/detail/mov_j__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_GetResult_Response & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_GetResult_Response>()
{
  return "mg400_msgs::action::MovJ_GetResult_Response";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_GetResult_Response>()
{
  return "mg400_msgs/action/MovJ_GetResult_Response";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::action::MovJ_Result>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::action::MovJ_Result>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::action::MovJ_GetResult>()
{
  return "mg400_msgs::action::MovJ_GetResult";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_GetResult>()
{
  return "mg400_msgs/action/MovJ_GetResult";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::action::MovJ_GetResult_Request>::value &&
    has_fixed_size<mg400_msgs::action::MovJ_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::action::MovJ_GetResult_Request>::value &&
    has_bounded_size<mg400_msgs::action::MovJ_GetResult_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::action::MovJ_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::action::MovJ_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::action::MovJ_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "mg400_msgs/action/detail/mov_j__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const MovJ_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MovJ_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MovJ_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace mg400_msgs

namespace rosidl_generator_traits
{

[[deprecated("use mg400_msgs::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const mg400_msgs::action::MovJ_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::MovJ_FeedbackMessage & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::MovJ_FeedbackMessage>()
{
  return "mg400_msgs::action::MovJ_FeedbackMessage";
}

template<>
inline const char * name<mg400_msgs::action::MovJ_FeedbackMessage>()
{
  return "mg400_msgs/action/MovJ_FeedbackMessage";
}

template<>
struct has_fixed_size<mg400_msgs::action::MovJ_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::action::MovJ_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::MovJ_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::action::MovJ_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<mg400_msgs::action::MovJ_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<mg400_msgs::action::MovJ>
  : std::true_type
{
};

template<>
struct is_action_goal<mg400_msgs::action::MovJ_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<mg400_msgs::action::MovJ_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<mg400_msgs::action::MovJ_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // MG400_MSGS__ACTION__DETAIL__MOV_J__TRAITS_HPP_
