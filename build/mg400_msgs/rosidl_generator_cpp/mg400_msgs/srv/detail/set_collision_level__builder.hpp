// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/SetCollisionLevel.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__SET_COLLISION_LEVEL__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__SET_COLLISION_LEVEL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/set_collision_level__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_SetCollisionLevel_Request_level
{
public:
  Init_SetCollisionLevel_Request_level()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::srv::SetCollisionLevel_Request level(::mg400_msgs::srv::SetCollisionLevel_Request::_level_type arg)
  {
    msg_.level = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::SetCollisionLevel_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::SetCollisionLevel_Request>()
{
  return mg400_msgs::srv::builder::Init_SetCollisionLevel_Request_level();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_SetCollisionLevel_Response_error_id
{
public:
  explicit Init_SetCollisionLevel_Response_error_id(::mg400_msgs::srv::SetCollisionLevel_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::SetCollisionLevel_Response error_id(::mg400_msgs::srv::SetCollisionLevel_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::SetCollisionLevel_Response msg_;
};

class Init_SetCollisionLevel_Response_result
{
public:
  Init_SetCollisionLevel_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetCollisionLevel_Response_error_id result(::mg400_msgs::srv::SetCollisionLevel_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_SetCollisionLevel_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::SetCollisionLevel_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::SetCollisionLevel_Response>()
{
  return mg400_msgs::srv::builder::Init_SetCollisionLevel_Response_result();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__SET_COLLISION_LEVEL__BUILDER_HPP_
