// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/joint_mov_j__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_JointMovJ_Request_j6
{
public:
  explicit Init_JointMovJ_Request_j6(::mg400_msgs::srv::JointMovJ_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::JointMovJ_Request j6(::mg400_msgs::srv::JointMovJ_Request::_j6_type arg)
  {
    msg_.j6 = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Request msg_;
};

class Init_JointMovJ_Request_j5
{
public:
  explicit Init_JointMovJ_Request_j5(::mg400_msgs::srv::JointMovJ_Request & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_Request_j6 j5(::mg400_msgs::srv::JointMovJ_Request::_j5_type arg)
  {
    msg_.j5 = std::move(arg);
    return Init_JointMovJ_Request_j6(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Request msg_;
};

class Init_JointMovJ_Request_j4
{
public:
  explicit Init_JointMovJ_Request_j4(::mg400_msgs::srv::JointMovJ_Request & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_Request_j5 j4(::mg400_msgs::srv::JointMovJ_Request::_j4_type arg)
  {
    msg_.j4 = std::move(arg);
    return Init_JointMovJ_Request_j5(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Request msg_;
};

class Init_JointMovJ_Request_j3
{
public:
  explicit Init_JointMovJ_Request_j3(::mg400_msgs::srv::JointMovJ_Request & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_Request_j4 j3(::mg400_msgs::srv::JointMovJ_Request::_j3_type arg)
  {
    msg_.j3 = std::move(arg);
    return Init_JointMovJ_Request_j4(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Request msg_;
};

class Init_JointMovJ_Request_j2
{
public:
  explicit Init_JointMovJ_Request_j2(::mg400_msgs::srv::JointMovJ_Request & msg)
  : msg_(msg)
  {}
  Init_JointMovJ_Request_j3 j2(::mg400_msgs::srv::JointMovJ_Request::_j2_type arg)
  {
    msg_.j2 = std::move(arg);
    return Init_JointMovJ_Request_j3(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Request msg_;
};

class Init_JointMovJ_Request_j1
{
public:
  Init_JointMovJ_Request_j1()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_JointMovJ_Request_j2 j1(::mg400_msgs::srv::JointMovJ_Request::_j1_type arg)
  {
    msg_.j1 = std::move(arg);
    return Init_JointMovJ_Request_j2(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::JointMovJ_Request>()
{
  return mg400_msgs::srv::builder::Init_JointMovJ_Request_j1();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_JointMovJ_Response_error_id
{
public:
  Init_JointMovJ_Response_error_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::mg400_msgs::srv::JointMovJ_Response error_id(::mg400_msgs::srv::JointMovJ_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::JointMovJ_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::JointMovJ_Response>()
{
  return mg400_msgs::srv::builder::Init_JointMovJ_Response_error_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__BUILDER_HPP_
