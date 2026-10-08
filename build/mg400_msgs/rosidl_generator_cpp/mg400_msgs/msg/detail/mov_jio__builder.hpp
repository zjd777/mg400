// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:msg/MovJIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_JIO__BUILDER_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_JIO__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/msg/detail/mov_jio__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace msg
{

namespace builder
{

class Init_MovJIO_acc_j
{
public:
  explicit Init_MovJIO_acc_j(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::msg::MovJIO acc_j(::mg400_msgs::msg::MovJIO::_acc_j_type arg)
  {
    msg_.acc_j = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_set_acc_j
{
public:
  explicit Init_MovJIO_set_acc_j(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_acc_j set_acc_j(::mg400_msgs::msg::MovJIO::_set_acc_j_type arg)
  {
    msg_.set_acc_j = std::move(arg);
    return Init_MovJIO_acc_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_speed_j
{
public:
  explicit Init_MovJIO_speed_j(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_set_acc_j speed_j(::mg400_msgs::msg::MovJIO::_speed_j_type arg)
  {
    msg_.speed_j = std::move(arg);
    return Init_MovJIO_set_acc_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_set_speed_j
{
public:
  explicit Init_MovJIO_set_speed_j(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_speed_j set_speed_j(::mg400_msgs::msg::MovJIO::_set_speed_j_type arg)
  {
    msg_.set_speed_j = std::move(arg);
    return Init_MovJIO_speed_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_status
{
public:
  explicit Init_MovJIO_status(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_set_speed_j status(::mg400_msgs::msg::MovJIO::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_MovJIO_set_speed_j(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_index
{
public:
  explicit Init_MovJIO_index(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_status index(::mg400_msgs::msg::MovJIO::_index_type arg)
  {
    msg_.index = std::move(arg);
    return Init_MovJIO_status(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_distance
{
public:
  explicit Init_MovJIO_distance(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_index distance(::mg400_msgs::msg::MovJIO::_distance_type arg)
  {
    msg_.distance = std::move(arg);
    return Init_MovJIO_index(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_mode
{
public:
  explicit Init_MovJIO_mode(::mg400_msgs::msg::MovJIO & msg)
  : msg_(msg)
  {}
  Init_MovJIO_distance mode(::mg400_msgs::msg::MovJIO::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return Init_MovJIO_distance(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

class Init_MovJIO_pose
{
public:
  Init_MovJIO_pose()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MovJIO_mode pose(::mg400_msgs::msg::MovJIO::_pose_type arg)
  {
    msg_.pose = std::move(arg);
    return Init_MovJIO_mode(msg_);
  }

private:
  ::mg400_msgs::msg::MovJIO msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::msg::MovJIO>()
{
  return mg400_msgs::msg::builder::Init_MovJIO_pose();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__MOV_JIO__BUILDER_HPP_
