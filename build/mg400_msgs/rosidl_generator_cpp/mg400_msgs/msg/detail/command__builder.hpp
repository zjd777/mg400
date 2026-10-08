// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COMMAND__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__COMMAND__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/command__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_Command_mov_lio_params
{
public:
  explicit Init_Command_mov_lio_params(::mg400_msgs::msg::Command & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::Command mov_lio_params(::mg400_msgs::msg::Command::_mov_lio_params_type arg)
  {
    msg_.mov_lio_params = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::Command msg_;
};

class Init_Command_mov_jio_params
{
public:
  explicit Init_Command_mov_jio_params(::mg400_msgs::msg::Command & msg)
  : msg_(msg)
  {}
  Init_Command_mov_lio_params mov_jio_params(::mg400_msgs::msg::Command::_mov_jio_params_type arg)
  {
    msg_.mov_jio_params = std::move(arg);
    return Init_Command_mov_lio_params(msg_);
  }

private:
  ::mg400_msgs::msg::Command msg_;
};

class Init_Command_joint_mov_j_params
{
public:
  explicit Init_Command_joint_mov_j_params(::mg400_msgs::msg::Command & msg)
  : msg_(msg)
  {}
  Init_Command_mov_jio_params joint_mov_j_params(::mg400_msgs::msg::Command::_joint_mov_j_params_type arg)
  {
    msg_.joint_mov_j_params = std::move(arg);
    return Init_Command_mov_jio_params(msg_);
  }

private:
  ::mg400_msgs::msg::Command msg_;
};

class Init_Command_mov_l_params
{
public:
  explicit Init_Command_mov_l_params(::mg400_msgs::msg::Command & msg)
  : msg_(msg)
  {}
  Init_Command_joint_mov_j_params mov_l_params(::mg400_msgs::msg::Command::_mov_l_params_type arg)
  {
    msg_.mov_l_params = std::move(arg);
    return Init_Command_joint_mov_j_params(msg_);
  }

private:
  ::mg400_msgs::msg::Command msg_;
};

class Init_Command_mov_j_params
{
public:
  explicit Init_Command_mov_j_params(::mg400_msgs::msg::Command & msg)
  : msg_(msg)
  {}
  Init_Command_mov_l_params mov_j_params(::mg400_msgs::msg::Command::_mov_j_params_type arg)
  {
    msg_.mov_j_params = std::move(arg);
    return Init_Command_mov_l_params(msg_);
  }

private:
  ::mg400_msgs::msg::Command msg_;
};

class Init_Command_command_type
{
public:
  Init_Command_command_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Command_mov_j_params command_type(::mg400_msgs::msg::Command::_command_type_type arg)
  {
    msg_.command_type = std::move(arg);
    return Init_Command_mov_j_params(msg_);
  }

private:
  ::mg400_msgs::msg::Command msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::Command>()
{
  return mg400_msgs::msg::builder::Init_Command_command_type();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__COMMAND__BUILDER_HPP_
