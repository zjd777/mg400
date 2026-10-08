// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from mg400_msgs:action/CommandQueue.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__TRAITS_HPP_
#define MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "mg400_msgs/action/detail/command_queue__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'commands'
#include "mg400_msgs/msg/detail/command__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const CommandQueue_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: commands
  {
    if (msg.commands.size() == 0) {
      out << "commands: []";
    } else {
      out << "commands: [";
      size_t pending_items = msg.commands.size();
      for (auto item : msg.commands) {
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
  const CommandQueue_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: commands
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.commands.size() == 0) {
      out << "commands: []\n";
    } else {
      out << "commands:\n";
      for (auto item : msg.commands) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CommandQueue_Goal & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_Goal & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_Goal>()
{
  return "mg400_msgs::action::CommandQueue_Goal";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_Goal>()
{
  return "mg400_msgs/action/CommandQueue_Goal";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_Goal>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_Goal>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_Goal>
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
  const CommandQueue_Result & msg,
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
  const CommandQueue_Result & msg,
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

inline std::string to_yaml(const CommandQueue_Result & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_Result & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_Result>()
{
  return "mg400_msgs::action::CommandQueue_Result";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_Result>()
{
  return "mg400_msgs/action/CommandQueue_Result";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_Result>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::msg::ErrorID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_Result>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::msg::ErrorID>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'current_pose'
#include "geometry_msgs/msg/detail/pose_stamped__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const CommandQueue_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: current_pose
  {
    out << "current_pose: ";
    to_flow_style_yaml(msg.current_pose, out);
    out << ", ";
  }

  // member: current_angles
  {
    if (msg.current_angles.size() == 0) {
      out << "current_angles: []";
    } else {
      out << "current_angles: [";
      size_t pending_items = msg.current_angles.size();
      for (auto item : msg.current_angles) {
        rosidl_generator_traits::value_to_yaml(item, out);
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
  const CommandQueue_Feedback & msg,
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

  // member: current_angles
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.current_angles.size() == 0) {
      out << "current_angles: []\n";
    } else {
      out << "current_angles:\n";
      for (auto item : msg.current_angles) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CommandQueue_Feedback & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_Feedback & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_Feedback>()
{
  return "mg400_msgs::action::CommandQueue_Feedback";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_Feedback>()
{
  return "mg400_msgs/action/CommandQueue_Feedback";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_Feedback>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_Feedback>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::PoseStamped>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "mg400_msgs/action/detail/command_queue__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const CommandQueue_SendGoal_Request & msg,
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
  const CommandQueue_SendGoal_Request & msg,
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

inline std::string to_yaml(const CommandQueue_SendGoal_Request & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_SendGoal_Request & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_SendGoal_Request>()
{
  return "mg400_msgs::action::CommandQueue_SendGoal_Request";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_SendGoal_Request>()
{
  return "mg400_msgs/action/CommandQueue_SendGoal_Request";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::action::CommandQueue_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::action::CommandQueue_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_SendGoal_Request>
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
  const CommandQueue_SendGoal_Response & msg,
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
  const CommandQueue_SendGoal_Response & msg,
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

inline std::string to_yaml(const CommandQueue_SendGoal_Response & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_SendGoal_Response & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_SendGoal_Response>()
{
  return "mg400_msgs::action::CommandQueue_SendGoal_Response";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_SendGoal_Response>()
{
  return "mg400_msgs/action/CommandQueue_SendGoal_Response";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_SendGoal>()
{
  return "mg400_msgs::action::CommandQueue_SendGoal";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_SendGoal>()
{
  return "mg400_msgs/action/CommandQueue_SendGoal";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::action::CommandQueue_SendGoal_Request>::value &&
    has_fixed_size<mg400_msgs::action::CommandQueue_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::action::CommandQueue_SendGoal_Request>::value &&
    has_bounded_size<mg400_msgs::action::CommandQueue_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::action::CommandQueue_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::action::CommandQueue_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::action::CommandQueue_SendGoal_Response>
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
  const CommandQueue_GetResult_Request & msg,
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
  const CommandQueue_GetResult_Request & msg,
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

inline std::string to_yaml(const CommandQueue_GetResult_Request & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_GetResult_Request & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_GetResult_Request>()
{
  return "mg400_msgs::action::CommandQueue_GetResult_Request";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_GetResult_Request>()
{
  return "mg400_msgs/action/CommandQueue_GetResult_Request";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "mg400_msgs/action/detail/command_queue__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const CommandQueue_GetResult_Response & msg,
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
  const CommandQueue_GetResult_Response & msg,
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

inline std::string to_yaml(const CommandQueue_GetResult_Response & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_GetResult_Response & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_GetResult_Response>()
{
  return "mg400_msgs::action::CommandQueue_GetResult_Response";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_GetResult_Response>()
{
  return "mg400_msgs/action/CommandQueue_GetResult_Response";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::action::CommandQueue_Result>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::action::CommandQueue_Result>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_GetResult>()
{
  return "mg400_msgs::action::CommandQueue_GetResult";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_GetResult>()
{
  return "mg400_msgs/action/CommandQueue_GetResult";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<mg400_msgs::action::CommandQueue_GetResult_Request>::value &&
    has_fixed_size<mg400_msgs::action::CommandQueue_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<mg400_msgs::action::CommandQueue_GetResult_Request>::value &&
    has_bounded_size<mg400_msgs::action::CommandQueue_GetResult_Response>::value
  >
{
};

template<>
struct is_service<mg400_msgs::action::CommandQueue_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<mg400_msgs::action::CommandQueue_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<mg400_msgs::action::CommandQueue_GetResult_Response>
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
// #include "mg400_msgs/action/detail/command_queue__traits.hpp"

namespace mg400_msgs
{

namespace action
{

inline void to_flow_style_yaml(
  const CommandQueue_FeedbackMessage & msg,
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
  const CommandQueue_FeedbackMessage & msg,
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

inline std::string to_yaml(const CommandQueue_FeedbackMessage & msg, bool use_flow_style = false)
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
  const mg400_msgs::action::CommandQueue_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  mg400_msgs::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use mg400_msgs::action::to_yaml() instead")]]
inline std::string to_yaml(const mg400_msgs::action::CommandQueue_FeedbackMessage & msg)
{
  return mg400_msgs::action::to_yaml(msg);
}

template<>
inline const char * data_type<mg400_msgs::action::CommandQueue_FeedbackMessage>()
{
  return "mg400_msgs::action::CommandQueue_FeedbackMessage";
}

template<>
inline const char * name<mg400_msgs::action::CommandQueue_FeedbackMessage>()
{
  return "mg400_msgs/action/CommandQueue_FeedbackMessage";
}

template<>
struct has_fixed_size<mg400_msgs::action::CommandQueue_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<mg400_msgs::action::CommandQueue_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<mg400_msgs::action::CommandQueue_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<mg400_msgs::action::CommandQueue_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<mg400_msgs::action::CommandQueue_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<mg400_msgs::action::CommandQueue>
  : std::true_type
{
};

template<>
struct is_action_goal<mg400_msgs::action::CommandQueue_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<mg400_msgs::action::CommandQueue_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<mg400_msgs::action::CommandQueue_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__TRAITS_HPP_
