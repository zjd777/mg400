// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/PositiveSolution.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/positive_solution__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_PositiveSolution_Request_tool
{
public:
  explicit Init_PositiveSolution_Request_tool(::mg400_msgs::srv::PositiveSolution_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::PositiveSolution_Request tool(::mg400_msgs::srv::PositiveSolution_Request::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Request msg_;
};

class Init_PositiveSolution_Request_user
{
public:
  explicit Init_PositiveSolution_Request_user(::mg400_msgs::srv::PositiveSolution_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Request_tool user(::mg400_msgs::srv::PositiveSolution_Request::_user_type arg)
  {
    msg_.user = std::move(arg);
    return Init_PositiveSolution_Request_tool(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Request msg_;
};

class Init_PositiveSolution_Request_j4
{
public:
  explicit Init_PositiveSolution_Request_j4(::mg400_msgs::srv::PositiveSolution_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Request_user j4(::mg400_msgs::srv::PositiveSolution_Request::_j4_type arg)
  {
    msg_.j4 = std::move(arg);
    return Init_PositiveSolution_Request_user(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Request msg_;
};

class Init_PositiveSolution_Request_j3
{
public:
  explicit Init_PositiveSolution_Request_j3(::mg400_msgs::srv::PositiveSolution_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Request_j4 j3(::mg400_msgs::srv::PositiveSolution_Request::_j3_type arg)
  {
    msg_.j3 = std::move(arg);
    return Init_PositiveSolution_Request_j4(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Request msg_;
};

class Init_PositiveSolution_Request_j2
{
public:
  explicit Init_PositiveSolution_Request_j2(::mg400_msgs::srv::PositiveSolution_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Request_j3 j2(::mg400_msgs::srv::PositiveSolution_Request::_j2_type arg)
  {
    msg_.j2 = std::move(arg);
    return Init_PositiveSolution_Request_j3(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Request msg_;
};

class Init_PositiveSolution_Request_j1
{
public:
  Init_PositiveSolution_Request_j1()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PositiveSolution_Request_j2 j1(::mg400_msgs::srv::PositiveSolution_Request::_j1_type arg)
  {
    msg_.j1 = std::move(arg);
    return Init_PositiveSolution_Request_j2(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::PositiveSolution_Request>()
{
  return mg400_msgs::srv::builder::Init_PositiveSolution_Request_j1();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_PositiveSolution_Response_r
{
public:
  explicit Init_PositiveSolution_Response_r(::mg400_msgs::srv::PositiveSolution_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::PositiveSolution_Response r(::mg400_msgs::srv::PositiveSolution_Response::_r_type arg)
  {
    msg_.r = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Response msg_;
};

class Init_PositiveSolution_Response_z
{
public:
  explicit Init_PositiveSolution_Response_z(::mg400_msgs::srv::PositiveSolution_Response & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Response_r z(::mg400_msgs::srv::PositiveSolution_Response::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_PositiveSolution_Response_r(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Response msg_;
};

class Init_PositiveSolution_Response_y
{
public:
  explicit Init_PositiveSolution_Response_y(::mg400_msgs::srv::PositiveSolution_Response & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Response_z y(::mg400_msgs::srv::PositiveSolution_Response::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_PositiveSolution_Response_z(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Response msg_;
};

class Init_PositiveSolution_Response_x
{
public:
  explicit Init_PositiveSolution_Response_x(::mg400_msgs::srv::PositiveSolution_Response & msg)
  : msg_(msg)
  {}
  Init_PositiveSolution_Response_y x(::mg400_msgs::srv::PositiveSolution_Response::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_PositiveSolution_Response_y(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Response msg_;
};

class Init_PositiveSolution_Response_error_id
{
public:
  Init_PositiveSolution_Response_error_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PositiveSolution_Response_x error_id(::mg400_msgs::srv::PositiveSolution_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return Init_PositiveSolution_Response_x(msg_);
  }

private:
  ::mg400_msgs::srv::PositiveSolution_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::PositiveSolution_Response>()
{
  return mg400_msgs::srv::builder::Init_PositiveSolution_Response_error_id();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__BUILDER_HPP_
