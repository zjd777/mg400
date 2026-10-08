// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/PayLoad.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__PAY_LOAD__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__PAY_LOAD__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/pay_load__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_PayLoad_Request_inertia
{
public:
  explicit Init_PayLoad_Request_inertia(::mg400_msgs::srv::PayLoad_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::PayLoad_Request inertia(::mg400_msgs::srv::PayLoad_Request::_inertia_type arg)
  {
    msg_.inertia = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::PayLoad_Request msg_;
};

class Init_PayLoad_Request_weight
{
public:
  Init_PayLoad_Request_weight()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PayLoad_Request_inertia weight(::mg400_msgs::srv::PayLoad_Request::_weight_type arg)
  {
    msg_.weight = std::move(arg);
    return Init_PayLoad_Request_inertia(msg_);
  }

private:
  ::mg400_msgs::srv::PayLoad_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::PayLoad_Request>()
{
  return mg400_msgs::srv::builder::Init_PayLoad_Request_weight();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_PayLoad_Response_error_id
{
public:
  explicit Init_PayLoad_Response_error_id(::mg400_msgs::srv::PayLoad_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::PayLoad_Response error_id(::mg400_msgs::srv::PayLoad_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::PayLoad_Response msg_;
};

class Init_PayLoad_Response_result
{
public:
  Init_PayLoad_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PayLoad_Response_error_id result(::mg400_msgs::srv::PayLoad_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_PayLoad_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::PayLoad_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::PayLoad_Response>()
{
  return mg400_msgs::srv::builder::Init_PayLoad_Response_result();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__PAY_LOAD__BUILDER_HPP_
