// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/RobotMode.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ROBOT_MODE__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__ROBOT_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/robot_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_RobotMode_robot_mode
{
public:
  Init_RobotMode_robot_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::RobotMode robot_mode(::mg400_msgs::msg::RobotMode::_robot_mode_type arg)
  {
    msg_.robot_mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::RobotMode msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::RobotMode>()
{
  return mg400_msgs::msg::builder::Init_RobotMode_robot_mode();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ROBOT_MODE__BUILDER_HPP_
