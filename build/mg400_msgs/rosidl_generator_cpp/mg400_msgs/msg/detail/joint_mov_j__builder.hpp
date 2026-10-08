// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/joint_mov_j__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_JointMovJ_cp
{
public:
  explicit Init_JointMovJ_cp(::mg400_msgs::msg::JointMovJ & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::JointMovJ cp(::mg400_msgs::msg::JointMovJ::_cp_type arg)
  {
    msg_.cp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

class Init_JointMovJ_set_cp
{
public:
  explicit Init_JointMovJ_set_cp(::mg400_msgs::msg::JointMovJ & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_cp set_cp(::mg400_msgs::msg::JointMovJ::_set_cp_type arg)
  {
    msg_.set_cp = std::move(arg);
    return Init_JointMovJ_cp(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

class Init_JointMovJ_acc_j
{
public:
  explicit Init_JointMovJ_acc_j(::mg400_msgs::msg::JointMovJ & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_set_cp acc_j(::mg400_msgs::msg::JointMovJ::_acc_j_type arg)
  {
    msg_.acc_j = std::move(arg);
    return Init_JointMovJ_set_cp(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

class Init_JointMovJ_set_acc_j
{
public:
  explicit Init_JointMovJ_set_acc_j(::mg400_msgs::msg::JointMovJ & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_acc_j set_acc_j(::mg400_msgs::msg::JointMovJ::_set_acc_j_type arg)
  {
    msg_.set_acc_j = std::move(arg);
    return Init_JointMovJ_acc_j(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

class Init_JointMovJ_speed_j
{
public:
  explicit Init_JointMovJ_speed_j(::mg400_msgs::msg::JointMovJ & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_set_acc_j speed_j(::mg400_msgs::msg::JointMovJ::_speed_j_type arg)
  {
    msg_.speed_j = std::move(arg);
    return Init_JointMovJ_set_acc_j(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

class Init_JointMovJ_set_speed_j
{
public:
  explicit Init_JointMovJ_set_speed_j(::mg400_msgs::msg::JointMovJ & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_speed_j set_speed_j(::mg400_msgs::msg::JointMovJ::_set_speed_j_type arg)
  {
    msg_.set_speed_j = std::move(arg);
    return Init_JointMovJ_speed_j(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

class Init_JointMovJ_joint_angles
{
public:
  Init_JointMovJ_joint_angles()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_JointMovJ_set_speed_j joint_angles(::mg400_msgs::msg::JointMovJ::_joint_angles_type arg)
  {
    msg_.joint_angles = std::move(arg);
    return Init_JointMovJ_set_speed_j(msg_);
  }

private:
  ::mg400_msgs::msg::JointMovJ msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::JointMovJ>()
{
  return mg400_msgs::msg::builder::Init_JointMovJ_joint_angles();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__BUILDER_HPP_
