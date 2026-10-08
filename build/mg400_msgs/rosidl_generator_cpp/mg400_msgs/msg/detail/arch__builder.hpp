// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/Arch.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ARCH__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__ARCH__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/arch__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_Arch_index
{
public:
  Init_Arch_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::Arch index(::mg400_msgs::msg::Arch::_index_type arg)
  {
    msg_.index = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::Arch msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::Arch>()
{
  return mg400_msgs::msg::builder::Init_Arch_index();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ARCH__BUILDER_HPP_
