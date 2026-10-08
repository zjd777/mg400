// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/MovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_J__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_J__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/mov_j__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_MovJ_cp
{
public:
  explicit Init_MovJ_cp(::mg400_msgs::msg::MovJ & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::MovJ cp(::mg400_msgs::msg::MovJ::_cp_type arg)
  {
    msg_.cp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

class Init_MovJ_set_cp
{
public:
  explicit Init_MovJ_set_cp(::mg400_msgs::msg::MovJ & msg)
  : msg_(msg)
  {}
  Init_MovJ_cp set_cp(::mg400_msgs::msg::MovJ::_set_cp_type arg)
  {
    msg_.set_cp = std::move(arg);
    return Init_MovJ_cp(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

class Init_MovJ_acc_j
{
public:
  explicit Init_MovJ_acc_j(::mg400_msgs::msg::MovJ & msg)
  : msg_(msg)
  {}
  Init_MovJ_set_cp acc_j(::mg400_msgs::msg::MovJ::_acc_j_type arg)
  {
    msg_.acc_j = std::move(arg);
    return Init_MovJ_set_cp(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

class Init_MovJ_set_acc_j
{
public:
  explicit Init_MovJ_set_acc_j(::mg400_msgs::msg::MovJ & msg)
  : msg_(msg)
  {}
  Init_MovJ_acc_j set_acc_j(::mg400_msgs::msg::MovJ::_set_acc_j_type arg)
  {
    msg_.set_acc_j = std::move(arg);
    return Init_MovJ_acc_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

class Init_MovJ_speed_j
{
public:
  explicit Init_MovJ_speed_j(::mg400_msgs::msg::MovJ & msg)
  : msg_(msg)
  {}
  Init_MovJ_set_acc_j speed_j(::mg400_msgs::msg::MovJ::_speed_j_type arg)
  {
    msg_.speed_j = std::move(arg);
    return Init_MovJ_set_acc_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

class Init_MovJ_set_speed_j
{
public:
  explicit Init_MovJ_set_speed_j(::mg400_msgs::msg::MovJ & msg)
  : msg_(msg)
  {}
  Init_MovJ_speed_j set_speed_j(::mg400_msgs::msg::MovJ::_set_speed_j_type arg)
  {
    msg_.set_speed_j = std::move(arg);
    return Init_MovJ_speed_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

class Init_MovJ_pose
{
public:
  Init_MovJ_pose()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MovJ_set_speed_j pose(::mg400_msgs::msg::MovJ::_pose_type arg)
  {
    msg_.pose = std::move(arg);
    return Init_MovJ_set_speed_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJ msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::MovJ>()
{
  return mg400_msgs::msg::builder::Init_MovJ_pose();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__MOV_J__BUILDER_HPP_
