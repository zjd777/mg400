// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/EnableRobot.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/enable_robot__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_EnableRobot_Request_center_z
{
public:
  explicit Init_EnableRobot_Request_center_z(::mg400_msgs::srv::EnableRobot_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::EnableRobot_Request center_z(::mg400_msgs::srv::EnableRobot_Request::_center_z_type arg)
  {
    msg_.center_z = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Request msg_;
};

class Init_EnableRobot_Request_center_y
{
public:
  explicit Init_EnableRobot_Request_center_y(::mg400_msgs::srv::EnableRobot_Request & msg)
  : msg_(msg)
  {}
  Init_EnableRobot_Request_center_z center_y(::mg400_msgs::srv::EnableRobot_Request::_center_y_type arg)
  {
    msg_.center_y = std::move(arg);
    return Init_EnableRobot_Request_center_z(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Request msg_;
};

class Init_EnableRobot_Request_center_x
{
public:
  explicit Init_EnableRobot_Request_center_x(::mg400_msgs::srv::EnableRobot_Request & msg)
  : msg_(msg)
  {}
  Init_EnableRobot_Request_center_y center_x(::mg400_msgs::srv::EnableRobot_Request::_center_x_type arg)
  {
    msg_.center_x = std::move(arg);
    return Init_EnableRobot_Request_center_y(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Request msg_;
};

class Init_EnableRobot_Request_load
{
public:
  explicit Init_EnableRobot_Request_load(::mg400_msgs::srv::EnableRobot_Request & msg)
  : msg_(msg)
  {}
  Init_EnableRobot_Request_center_x load(::mg400_msgs::srv::EnableRobot_Request::_load_type arg)
  {
    msg_.load = std::move(arg);
    return Init_EnableRobot_Request_center_x(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Request msg_;
};

class Init_EnableRobot_Request_num_of_params
{
public:
  Init_EnableRobot_Request_num_of_params()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_EnableRobot_Request_load num_of_params(::mg400_msgs::srv::EnableRobot_Request::_num_of_params_type arg)
  {
    msg_.num_of_params = std::move(arg);
    return Init_EnableRobot_Request_load(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::EnableRobot_Request>()
{
  return mg400_msgs::srv::builder::Init_EnableRobot_Request_num_of_params();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_EnableRobot_Response_error_id
{
public:
  explicit Init_EnableRobot_Response_error_id(::mg400_msgs::srv::EnableRobot_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::EnableRobot_Response error_id(::mg400_msgs::srv::EnableRobot_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Response msg_;
};

class Init_EnableRobot_Response_result
{
public:
  Init_EnableRobot_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_EnableRobot_Response_error_id result(::mg400_msgs::srv::EnableRobot_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_EnableRobot_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::EnableRobot_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::EnableRobot_Response>()
{
  return mg400_msgs::srv::builder::Init_EnableRobot_Response_result();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__BUILDER_HPP_
