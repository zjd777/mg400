// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/DOStatus.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DO_STATUS__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__DO_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/do_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_DOStatus_status
{
public:
  Init_DOStatus_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::DOStatus status(::mg400_msgs::msg::DOStatus::_status_type arg)
  {
    msg_.status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::DOStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::DOStatus>()
{
  return mg400_msgs::msg::builder::Init_DOStatus_status();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__DO_STATUS__BUILDER_HPP_
