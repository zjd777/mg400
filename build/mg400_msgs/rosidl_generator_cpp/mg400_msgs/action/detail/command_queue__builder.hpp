// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:action/CommandQueue.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__BUILDER_HPP_
#define MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/action/detail/command_queue__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_Goal_commands
{
public:
  Init_CommandQueue_Goal_commands()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::action::CommandQueue_Goal commands(::mg400_msgs::action::CommandQueue_Goal::_commands_type arg)
  {
    msg_.commands = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_Goal>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_Goal_commands();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_Result_error_id
{
public:
  explicit Init_CommandQueue_Result_error_id(::mg400_msgs::action::CommandQueue_Result & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::action::CommandQueue_Result error_id(::mg400_msgs::action::CommandQueue_Result::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_Result msg_;
};

class Init_CommandQueue_Result_result
{
public:
  Init_CommandQueue_Result_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CommandQueue_Result_error_id result(::mg400_msgs::action::CommandQueue_Result::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_CommandQueue_Result_error_id(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_Result>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_Result_result();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_Feedback_current_angles
{
public:
  explicit Init_CommandQueue_Feedback_current_angles(::mg400_msgs::action::CommandQueue_Feedback & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::action::CommandQueue_Feedback current_angles(::mg400_msgs::action::CommandQueue_Feedback::_current_angles_type arg)
  {
    msg_.current_angles = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_Feedback msg_;
};

class Init_CommandQueue_Feedback_current_pose
{
public:
  Init_CommandQueue_Feedback_current_pose()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CommandQueue_Feedback_current_angles current_pose(::mg400_msgs::action::CommandQueue_Feedback::_current_pose_type arg)
  {
    msg_.current_pose = std::move(arg);
    return Init_CommandQueue_Feedback_current_angles(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_Feedback>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_Feedback_current_pose();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_SendGoal_Request_goal
{
public:
  explicit Init_CommandQueue_SendGoal_Request_goal(::mg400_msgs::action::CommandQueue_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::action::CommandQueue_SendGoal_Request goal(::mg400_msgs::action::CommandQueue_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_SendGoal_Request msg_;
};

class Init_CommandQueue_SendGoal_Request_goal_id
{
public:
  Init_CommandQueue_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CommandQueue_SendGoal_Request_goal goal_id(::mg400_msgs::action::CommandQueue_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_CommandQueue_SendGoal_Request_goal(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_SendGoal_Request>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_SendGoal_Request_goal_id();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_SendGoal_Response_stamp
{
public:
  explicit Init_CommandQueue_SendGoal_Response_stamp(::mg400_msgs::action::CommandQueue_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::action::CommandQueue_SendGoal_Response stamp(::mg400_msgs::action::CommandQueue_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_SendGoal_Response msg_;
};

class Init_CommandQueue_SendGoal_Response_accepted
{
public:
  Init_CommandQueue_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CommandQueue_SendGoal_Response_stamp accepted(::mg400_msgs::action::CommandQueue_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_CommandQueue_SendGoal_Response_stamp(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_SendGoal_Response>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_SendGoal_Response_accepted();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_GetResult_Request_goal_id
{
public:
  Init_CommandQueue_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::action::CommandQueue_GetResult_Request goal_id(::mg400_msgs::action::CommandQueue_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_GetResult_Request>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_GetResult_Request_goal_id();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_GetResult_Response_result
{
public:
  explicit Init_CommandQueue_GetResult_Response_result(::mg400_msgs::action::CommandQueue_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::action::CommandQueue_GetResult_Response result(::mg400_msgs::action::CommandQueue_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_GetResult_Response msg_;
};

class Init_CommandQueue_GetResult_Response_status
{
public:
  Init_CommandQueue_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CommandQueue_GetResult_Response_result status(::mg400_msgs::action::CommandQueue_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_CommandQueue_GetResult_Response_result(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_GetResult_Response>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_GetResult_Response_status();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace action
{

namespace builder
{

class Init_CommandQueue_FeedbackMessage_feedback
{
public:
  explicit Init_CommandQueue_FeedbackMessage_feedback(::mg400_msgs::action::CommandQueue_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::action::CommandQueue_FeedbackMessage feedback(::mg400_msgs::action::CommandQueue_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_FeedbackMessage msg_;
};

class Init_CommandQueue_FeedbackMessage_goal_id
{
public:
  Init_CommandQueue_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CommandQueue_FeedbackMessage_feedback goal_id(::mg400_msgs::action::CommandQueue_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_CommandQueue_FeedbackMessage_feedback(msg_);
  }

private:
  ::mg400_msgs::action::CommandQueue_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::action::CommandQueue_FeedbackMessage>()
{
  return mg400_msgs::action::builder::Init_CommandQueue_FeedbackMessage_goal_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__BUILDER_HPP_
