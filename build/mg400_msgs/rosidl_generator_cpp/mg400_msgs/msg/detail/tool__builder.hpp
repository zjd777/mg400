// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/Tool.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__TOOL__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__TOOL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/tool__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_Tool_tool
{
public:
  Init_Tool_tool()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::Tool tool(::mg400_msgs::msg::Tool::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::Tool msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::Tool>()
{
  return mg400_msgs::msg::builder::Init_Tool_tool();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__TOOL__BUILDER_HPP_
