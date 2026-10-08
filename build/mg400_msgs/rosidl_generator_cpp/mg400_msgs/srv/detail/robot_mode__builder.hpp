// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/RobotMode.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__ROBOT_MODE__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__ROBOT_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/robot_mode__struct.hpp"
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
auto build<::mg400_msgs::srv::RobotMode_Request>()
{
  return ::mg400_msgs::srv::RobotMode_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_RobotMode_Response_error_id
{
public:
  explicit Init_RobotMode_Response_error_id(::mg400_msgs::srv::RobotMode_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::RobotMode_Response error_id(::mg400_msgs::srv::RobotMode_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::RobotMode_Response msg_;
};

class Init_RobotMode_Response_robot_mode
{
public:
  Init_RobotMode_Response_robot_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RobotMode_Response_error_id robot_mode(::mg400_msgs::srv::RobotMode_Response::_robot_mode_type arg)
  {
    msg_.robot_mode = std::move(arg);
    return Init_RobotMode_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::RobotMode_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::RobotMode_Response>()
{
  return mg400_msgs::srv::builder::Init_RobotMode_Response_robot_mode();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__ROBOT_MODE__BUILDER_HPP_
