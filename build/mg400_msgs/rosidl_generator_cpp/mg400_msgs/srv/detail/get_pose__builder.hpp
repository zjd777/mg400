// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/GetPose.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__GET_POSE__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__GET_POSE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/get_pose__struct.hpp"
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
auto build<::mg400_msgs::srv::GetPose_Request>()
{
  return ::mg400_msgs::srv::GetPose_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_GetPose_Response_pose6
{
public:
  explicit Init_GetPose_Response_pose6(::mg400_msgs::srv::GetPose_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::GetPose_Response pose6(::mg400_msgs::srv::GetPose_Response::_pose6_type arg)
  {
    msg_.pose6 = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

class Init_GetPose_Response_pose5
{
public:
  explicit Init_GetPose_Response_pose5(::mg400_msgs::srv::GetPose_Response & msg)
  : msg_(msg)
  {}
  Init_GetPose_Response_pose6 pose5(::mg400_msgs::srv::GetPose_Response::_pose5_type arg)
  {
    msg_.pose5 = std::move(arg);
    return Init_GetPose_Response_pose6(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

class Init_GetPose_Response_pose4
{
public:
  explicit Init_GetPose_Response_pose4(::mg400_msgs::srv::GetPose_Response & msg)
  : msg_(msg)
  {}
  Init_GetPose_Response_pose5 pose4(::mg400_msgs::srv::GetPose_Response::_pose4_type arg)
  {
    msg_.pose4 = std::move(arg);
    return Init_GetPose_Response_pose5(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

class Init_GetPose_Response_pose3
{
public:
  explicit Init_GetPose_Response_pose3(::mg400_msgs::srv::GetPose_Response & msg)
  : msg_(msg)
  {}
  Init_GetPose_Response_pose4 pose3(::mg400_msgs::srv::GetPose_Response::_pose3_type arg)
  {
    msg_.pose3 = std::move(arg);
    return Init_GetPose_Response_pose4(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

class Init_GetPose_Response_pose2
{
public:
  explicit Init_GetPose_Response_pose2(::mg400_msgs::srv::GetPose_Response & msg)
  : msg_(msg)
  {}
  Init_GetPose_Response_pose3 pose2(::mg400_msgs::srv::GetPose_Response::_pose2_type arg)
  {
    msg_.pose2 = std::move(arg);
    return Init_GetPose_Response_pose3(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

class Init_GetPose_Response_pose1
{
public:
  explicit Init_GetPose_Response_pose1(::mg400_msgs::srv::GetPose_Response & msg)
  : msg_(msg)
  {}
  Init_GetPose_Response_pose2 pose1(::mg400_msgs::srv::GetPose_Response::_pose1_type arg)
  {
    msg_.pose1 = std::move(arg);
    return Init_GetPose_Response_pose2(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

class Init_GetPose_Response_error_id
{
public:
  Init_GetPose_Response_error_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetPose_Response_pose1 error_id(::mg400_msgs::srv::GetPose_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return Init_GetPose_Response_pose1(msg_);
  }

private:
  ::mg400_msgs::srv::GetPose_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::GetPose_Response>()
{
  return mg400_msgs::srv::builder::Init_GetPose_Response_error_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__GET_POSE__BUILDER_HPP_
