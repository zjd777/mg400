// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/MovL.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_L__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_L__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/mov_l__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_MovL_cp
{
public:
  explicit Init_MovL_cp(::mg400_msgs::msg::MovL & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::MovL cp(::mg400_msgs::msg::MovL::_cp_type arg)
  {
    msg_.cp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

class Init_MovL_set_cp
{
public:
  explicit Init_MovL_set_cp(::mg400_msgs::msg::MovL & msg)
  : msg_(msg)
  {}
  Init_MovL_cp set_cp(::mg400_msgs::msg::MovL::_set_cp_type arg)
  {
    msg_.set_cp = std::move(arg);
    return Init_MovL_cp(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

class Init_MovL_acc_l
{
public:
  explicit Init_MovL_acc_l(::mg400_msgs::msg::MovL & msg)
  : msg_(msg)
  {}
  Init_MovL_set_cp acc_l(::mg400_msgs::msg::MovL::_acc_l_type arg)
  {
    msg_.acc_l = std::move(arg);
    return Init_MovL_set_cp(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

class Init_MovL_set_acc_l
{
public:
  explicit Init_MovL_set_acc_l(::mg400_msgs::msg::MovL & msg)
  : msg_(msg)
  {}
  Init_MovL_acc_l set_acc_l(::mg400_msgs::msg::MovL::_set_acc_l_type arg)
  {
    msg_.set_acc_l = std::move(arg);
    return Init_MovL_acc_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

class Init_MovL_speed_l
{
public:
  explicit Init_MovL_speed_l(::mg400_msgs::msg::MovL & msg)
  : msg_(msg)
  {}
  Init_MovL_set_acc_l speed_l(::mg400_msgs::msg::MovL::_speed_l_type arg)
  {
    msg_.speed_l = std::move(arg);
    return Init_MovL_set_acc_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

class Init_MovL_set_speed_l
{
public:
  explicit Init_MovL_set_speed_l(::mg400_msgs::msg::MovL & msg)
  : msg_(msg)
  {}
  Init_MovL_speed_l set_speed_l(::mg400_msgs::msg::MovL::_set_speed_l_type arg)
  {
    msg_.set_speed_l = std::move(arg);
    return Init_MovL_speed_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

class Init_MovL_pose
{
public:
  Init_MovL_pose()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MovL_set_speed_l pose(::mg400_msgs::msg::MovL::_pose_type arg)
  {
    msg_.pose = std::move(arg);
    return Init_MovL_set_speed_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovL msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::MovL>()
{
  return mg400_msgs::msg::builder::Init_MovL_pose();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__MOV_L__BUILDER_HPP_
