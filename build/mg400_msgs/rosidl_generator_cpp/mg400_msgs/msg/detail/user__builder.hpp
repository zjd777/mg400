// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/User.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__USER__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__USER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/user__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_User_user
{
public:
  Init_User_user()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::User user(::mg400_msgs::msg::User::_user_type arg)
  {
    msg_.user = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::User msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::User>()
{
  return mg400_msgs::msg::builder::Init_User_user();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__USER__BUILDER_HPP_
