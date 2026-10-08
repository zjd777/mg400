// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/CollisionLevel.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/collision_level__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_CollisionLevel_level
{
public:
  Init_CollisionLevel_level()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::msg::CollisionLevel level(::mg400_msgs::msg::CollisionLevel::_level_type arg)
  {
    msg_.level = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::CollisionLevel msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::CollisionLevel>()
{
  return mg400_msgs::msg::builder::Init_CollisionLevel_level();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__BUILDER_HPP_
