// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/MoveJog.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__MOVE_JOG__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__MOVE_JOG__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/move_jog__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_MoveJog_Request_jog
{
public:
  Init_MoveJog_Request_jog()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::srv::MoveJog_Request jog(::mg400_msgs::srv::MoveJog_Request::_jog_type arg)
  {
    msg_.jog = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::MoveJog_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::MoveJog_Request>()
{
  return mg400_msgs::srv::builder::Init_MoveJog_Request_jog();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_MoveJog_Response_error_id
{
public:
  Init_MoveJog_Response_error_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::srv::MoveJog_Response error_id(::mg400_msgs::srv::MoveJog_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::MoveJog_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::MoveJog_Response>()
{
  return mg400_msgs::srv::builder::Init_MoveJog_Response_error_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__MOVE_JOG__BUILDER_HPP_
