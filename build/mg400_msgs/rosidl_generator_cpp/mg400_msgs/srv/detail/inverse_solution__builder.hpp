// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/InverseSolution.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__INVERSE_SOLUTION__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__INVERSE_SOLUTION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/inverse_solution__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_InverseSolution_Request_tool
{
public:
  explicit Init_InverseSolution_Request_tool(::mg400_msgs::srv::InverseSolution_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::InverseSolution_Request tool(::mg400_msgs::srv::InverseSolution_Request::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Request msg_;
};

class Init_InverseSolution_Request_user
{
public:
  explicit Init_InverseSolution_Request_user(::mg400_msgs::srv::InverseSolution_Request & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Request_tool user(::mg400_msgs::srv::InverseSolution_Request::_user_type arg)
  {
    msg_.user = std::move(arg);
    return Init_InverseSolution_Request_tool(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Request msg_;
};

class Init_InverseSolution_Request_r
{
public:
  explicit Init_InverseSolution_Request_r(::mg400_msgs::srv::InverseSolution_Request & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Request_user r(::mg400_msgs::srv::InverseSolution_Request::_r_type arg)
  {
    msg_.r = std::move(arg);
    return Init_InverseSolution_Request_user(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Request msg_;
};

class Init_InverseSolution_Request_z
{
public:
  explicit Init_InverseSolution_Request_z(::mg400_msgs::srv::InverseSolution_Request & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Request_r z(::mg400_msgs::srv::InverseSolution_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_InverseSolution_Request_r(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Request msg_;
};

class Init_InverseSolution_Request_y
{
public:
  explicit Init_InverseSolution_Request_y(::mg400_msgs::srv::InverseSolution_Request & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Request_z y(::mg400_msgs::srv::InverseSolution_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_InverseSolution_Request_z(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Request msg_;
};

class Init_InverseSolution_Request_x
{
public:
  Init_InverseSolution_Request_x()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_InverseSolution_Request_y x(::mg400_msgs::srv::InverseSolution_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_InverseSolution_Request_y(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::InverseSolution_Request>()
{
  return mg400_msgs::srv::builder::Init_InverseSolution_Request_x();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_InverseSolution_Response_j4
{
public:
  explicit Init_InverseSolution_Response_j4(::mg400_msgs::srv::InverseSolution_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::InverseSolution_Response j4(::mg400_msgs::srv::InverseSolution_Response::_j4_type arg)
  {
    msg_.j4 = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Response msg_;
};

class Init_InverseSolution_Response_j3
{
public:
  explicit Init_InverseSolution_Response_j3(::mg400_msgs::srv::InverseSolution_Response & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Response_j4 j3(::mg400_msgs::srv::InverseSolution_Response::_j3_type arg)
  {
    msg_.j3 = std::move(arg);
    return Init_InverseSolution_Response_j4(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Response msg_;
};

class Init_InverseSolution_Response_j2
{
public:
  explicit Init_InverseSolution_Response_j2(::mg400_msgs::srv::InverseSolution_Response & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Response_j3 j2(::mg400_msgs::srv::InverseSolution_Response::_j2_type arg)
  {
    msg_.j2 = std::move(arg);
    return Init_InverseSolution_Response_j3(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Response msg_;
};

class Init_InverseSolution_Response_j1
{
public:
  explicit Init_InverseSolution_Response_j1(::mg400_msgs::srv::InverseSolution_Response & msg)
  : msg_(msg)
  {}
  Init_InverseSolution_Response_j2 j1(::mg400_msgs::srv::InverseSolution_Response::_j1_type arg)
  {
    msg_.j1 = std::move(arg);
    return Init_InverseSolution_Response_j2(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Response msg_;
};

class Init_InverseSolution_Response_error_id
{
public:
  Init_InverseSolution_Response_error_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_InverseSolution_Response_j1 error_id(::mg400_msgs::srv::InverseSolution_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return Init_InverseSolution_Response_j1(msg_);
  }

private:
  ::mg400_msgs::srv::InverseSolution_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::InverseSolution_Response>()
{
  return mg400_msgs::srv::builder::Init_InverseSolution_Response_error_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__INVERSE_SOLUTION__BUILDER_HPP_
