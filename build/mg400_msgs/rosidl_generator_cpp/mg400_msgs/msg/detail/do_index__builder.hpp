// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/DOIndex.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DO_INDEX__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__DO_INDEX__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/do_index__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_DOIndex_index
{
public:
  Init_DOIndex_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::DOIndex index(::mg400_msgs::msg::DOIndex::_index_type arg)
  {
    msg_.index = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::DOIndex msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::DOIndex>()
{
  return mg400_msgs::msg::builder::Init_DOIndex_index();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__DO_INDEX__BUILDER_HPP_
