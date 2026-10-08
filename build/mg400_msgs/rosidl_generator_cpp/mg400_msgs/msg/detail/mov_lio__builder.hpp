// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/MovLIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_LIO__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_LIO__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/mov_lio__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_MovLIO_acc_l
{
public:
  explicit Init_MovLIO_acc_l(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::MovLIO acc_l(::mg400_msgs::msg::MovLIO::_acc_l_type arg)
  {
    msg_.acc_l = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_set_acc_l
{
public:
  explicit Init_MovLIO_set_acc_l(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_acc_l set_acc_l(::mg400_msgs::msg::MovLIO::_set_acc_l_type arg)
  {
    msg_.set_acc_l = std::move(arg);
    return Init_MovLIO_acc_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_speed_l
{
public:
  explicit Init_MovLIO_speed_l(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_set_acc_l speed_l(::mg400_msgs::msg::MovLIO::_speed_l_type arg)
  {
    msg_.speed_l = std::move(arg);
    return Init_MovLIO_set_acc_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_set_speed_l
{
public:
  explicit Init_MovLIO_set_speed_l(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_speed_l set_speed_l(::mg400_msgs::msg::MovLIO::_set_speed_l_type arg)
  {
    msg_.set_speed_l = std::move(arg);
    return Init_MovLIO_speed_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_status
{
public:
  explicit Init_MovLIO_status(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_set_speed_l status(::mg400_msgs::msg::MovLIO::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_MovLIO_set_speed_l(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_index
{
public:
  explicit Init_MovLIO_index(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_status index(::mg400_msgs::msg::MovLIO::_index_type arg)
  {
    msg_.index = std::move(arg);
    return Init_MovLIO_status(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_distance
{
public:
  explicit Init_MovLIO_distance(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_index distance(::mg400_msgs::msg::MovLIO::_distance_type arg)
  {
    msg_.distance = std::move(arg);
    return Init_MovLIO_index(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_mode
{
public:
  explicit Init_MovLIO_mode(::mg400_msgs::msg::MovLIO & msg)
  : msg_(msg)
  {}
  Init_MovLIO_distance mode(::mg400_msgs::msg::MovLIO::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return Init_MovLIO_distance(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

class Init_MovLIO_pose
{
public:
  Init_MovLIO_pose()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MovLIO_mode pose(::mg400_msgs::msg::MovLIO::_pose_type arg)
  {
    msg_.pose = std::move(arg);
    return Init_MovLIO_mode(msg_);
  }

private:
  ::mg400_msgs::msg::MovLIO msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::MovLIO>()
{
  return mg400_msgs::msg::builder::Init_MovLIO_pose();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__MOV_LIO__BUILDER_HPP_
