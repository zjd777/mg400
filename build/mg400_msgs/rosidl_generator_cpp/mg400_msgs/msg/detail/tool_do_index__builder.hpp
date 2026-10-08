// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/ToolDOIndex.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__TOOL_DO_INDEX__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__TOOL_DO_INDEX__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/tool_do_index__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_ToolDOIndex_index
{
public:
  Init_ToolDOIndex_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::ToolDOIndex index(::mg400_msgs::msg::ToolDOIndex::_index_type arg)
  {
    msg_.index = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::ToolDOIndex msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::ToolDOIndex>()
{
  return mg400_msgs::msg::builder::Init_ToolDOIndex_index();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__TOOL_DO_INDEX__BUILDER_HPP_
