// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/ResetRobot.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__RESET_ROBOT__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__RESET_ROBOT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/reset_robot__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::ResetRobot_Request>()
{
  return ::mg400_msgs::srv::ResetRobot_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_ResetRobot_Response_error_id
{
public:
  explicit Init_ResetRobot_Response_error_id(::mg400_msgs::srv::ResetRobot_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::ResetRobot_Response error_id(::mg400_msgs::srv::ResetRobot_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::ResetRobot_Response msg_;
};

class Init_ResetRobot_Response_result
{
public:
  Init_ResetRobot_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ResetRobot_Response_error_id result(::mg400_msgs::srv::ResetRobot_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_ResetRobot_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::ResetRobot_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::ResetRobot_Response>()
{
  return mg400_msgs::srv::builder::Init_ResetRobot_Response_result();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__RESET_ROBOT__BUILDER_HPP_
