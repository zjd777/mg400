// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/IDArray.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ID_ARRAY__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__ID_ARRAY__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/id_array__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_IDArray_ids
{
public:
  Init_IDArray_ids()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::IDArray ids(::mg400_msgs::msg::IDArray::_ids_type arg)
  {
    msg_.ids = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::IDArray msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::IDArray>()
{
  return mg400_msgs::msg::builder::Init_IDArray_ids();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ID_ARRAY__BUILDER_HPP_
