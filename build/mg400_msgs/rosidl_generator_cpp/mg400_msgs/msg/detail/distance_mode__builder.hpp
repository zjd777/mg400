// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/DistanceMode.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DISTANCE_MODE__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__DISTANCE_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/distance_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_DistanceMode_mode
{
public:
  Init_DistanceMode_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::DistanceMode mode(::mg400_msgs::msg::DistanceMode::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::DistanceMode msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::DistanceMode>()
{
  return mg400_msgs::msg::builder::Init_DistanceMode_mode();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__DISTANCE_MODE__BUILDER_HPP_
