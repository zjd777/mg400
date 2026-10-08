// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/User.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__USER__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__USER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/user__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_User_Request_user
{
public:
  Init_User_Request_user()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::srv::User_Request user(::mg400_msgs::srv::User_Request::_user_type arg)
  {
    msg_.user = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::User_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::User_Request>()
{
  return mg400_msgs::srv::builder::Init_User_Request_user();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_User_Response_error_id
{
public:
  explicit Init_User_Response_error_id(::mg400_msgs::srv::User_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::User_Response error_id(::mg400_msgs::srv::User_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::User_Response msg_;
};

class Init_User_Response_result
{
public:
  Init_User_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_User_Response_error_id result(::mg400_msgs::srv::User_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_User_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::User_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::User_Response>()
{
  return mg400_msgs::srv::builder::Init_User_Response_result();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__USER__BUILDER_HPP_
