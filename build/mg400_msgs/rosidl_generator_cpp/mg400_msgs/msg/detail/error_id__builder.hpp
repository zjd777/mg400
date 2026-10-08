// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ERROR_ID__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__ERROR_ID__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/error_id__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_ErrorID_servo
{
public:
  explicit Init_ErrorID_servo(::mg400_msgs::msg::ErrorID & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::ErrorID servo(::mg400_msgs::msg::ErrorID::_servo_type arg)
  {
    msg_.servo = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::ErrorID msg_;
};

class Init_ErrorID_controller
{
public:
  Init_ErrorID_controller()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ErrorID_servo controller(::mg400_msgs::msg::ErrorID::_controller_type arg)
  {
    msg_.controller = std::move(arg);
    return Init_ErrorID_servo(msg_);
  }

private:
  ::mg400_msgs::msg::ErrorID msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::ErrorID>()
{
  return mg400_msgs::msg::builder::Init_ErrorID_controller();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ERROR_ID__BUILDER_HPP_
