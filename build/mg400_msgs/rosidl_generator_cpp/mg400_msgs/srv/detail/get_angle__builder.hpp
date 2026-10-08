// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/GetAngle.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__GET_ANGLE__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__GET_ANGLE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/get_angle__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::GetAngle_Request>()
{
  return ::mg400_msgs::srv::GetAngle_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_GetAngle_Response_joint6
{
public:
  explicit Init_GetAngle_Response_joint6(::mg400_msgs::srv::GetAngle_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::GetAngle_Response joint6(::mg400_msgs::srv::GetAngle_Response::_joint6_type arg)
  {
    msg_.joint6 = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

class Init_GetAngle_Response_joint5
{
public:
  explicit Init_GetAngle_Response_joint5(::mg400_msgs::srv::GetAngle_Response & msg)
  : msg_(msg)
  {}
  Init_GetAngle_Response_joint6 joint5(::mg400_msgs::srv::GetAngle_Response::_joint5_type arg)
  {
    msg_.joint5 = std::move(arg);
    return Init_GetAngle_Response_joint6(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

class Init_GetAngle_Response_joint4
{
public:
  explicit Init_GetAngle_Response_joint4(::mg400_msgs::srv::GetAngle_Response & msg)
  : msg_(msg)
  {}
  Init_GetAngle_Response_joint5 joint4(::mg400_msgs::srv::GetAngle_Response::_joint4_type arg)
  {
    msg_.joint4 = std::move(arg);
    return Init_GetAngle_Response_joint5(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

class Init_GetAngle_Response_joint3
{
public:
  explicit Init_GetAngle_Response_joint3(::mg400_msgs::srv::GetAngle_Response & msg)
  : msg_(msg)
  {}
  Init_GetAngle_Response_joint4 joint3(::mg400_msgs::srv::GetAngle_Response::_joint3_type arg)
  {
    msg_.joint3 = std::move(arg);
    return Init_GetAngle_Response_joint4(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

class Init_GetAngle_Response_joint2
{
public:
  explicit Init_GetAngle_Response_joint2(::mg400_msgs::srv::GetAngle_Response & msg)
  : msg_(msg)
  {}
  Init_GetAngle_Response_joint3 joint2(::mg400_msgs::srv::GetAngle_Response::_joint2_type arg)
  {
    msg_.joint2 = std::move(arg);
    return Init_GetAngle_Response_joint3(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

class Init_GetAngle_Response_joint1
{
public:
  explicit Init_GetAngle_Response_joint1(::mg400_msgs::srv::GetAngle_Response & msg)
  : msg_(msg)
  {}
  Init_GetAngle_Response_joint2 joint1(::mg400_msgs::srv::GetAngle_Response::_joint1_type arg)
  {
    msg_.joint1 = std::move(arg);
    return Init_GetAngle_Response_joint2(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

class Init_GetAngle_Response_error_id
{
public:
  Init_GetAngle_Response_error_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetAngle_Response_joint1 error_id(::mg400_msgs::srv::GetAngle_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return Init_GetAngle_Response_joint1(msg_);
  }

private:
  ::mg400_msgs::srv::GetAngle_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::GetAngle_Response>()
{
  return mg400_msgs::srv::builder::Init_GetAngle_Response_error_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__GET_ANGLE__BUILDER_HPP_
