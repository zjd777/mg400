// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from mg400_msgs:srv/ToolDOExecute.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__TOOL_DO_EXECUTE__BUILDER_HPP_
#define MG400_MSGS__SRV__DETAIL__TOOL_DO_EXECUTE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "mg400_msgs/srv/detail/tool_do_execute__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_ToolDOExecute_Request_status
{
public:
  explicit Init_ToolDOExecute_Request_status(::mg400_msgs::srv::ToolDOExecute_Request & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::ToolDOExecute_Request status(::mg400_msgs::srv::ToolDOExecute_Request::_status_type arg)
  {
    msg_.status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::ToolDOExecute_Request msg_;
};

class Init_ToolDOExecute_Request_index
{
public:
  Init_ToolDOExecute_Request_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ToolDOExecute_Request_status index(::mg400_msgs::srv::ToolDOExecute_Request::_index_type arg)
  {
    msg_.index = std::move(arg);
    return Init_ToolDOExecute_Request_status(msg_);
  }

private:
  ::mg400_msgs::srv::ToolDOExecute_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::ToolDOExecute_Request>()
{
  return mg400_msgs::srv::builder::Init_ToolDOExecute_Request_index();
}

}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace srv
{

namespace builder
{

class Init_ToolDOExecute_Response_error_id
{
public:
  explicit Init_ToolDOExecute_Response_error_id(::mg400_msgs::srv::ToolDOExecute_Response & msg)
  : msg_(msg)
  {}
  ::mg400_msgs::srv::ToolDOExecute_Response error_id(::mg400_msgs::srv::ToolDOExecute_Response::_error_id_type arg)
  {
    msg_.error_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::mg400_msgs::srv::ToolDOExecute_Response msg_;
};

class Init_ToolDOExecute_Response_result
{
public:
  Init_ToolDOExecute_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ToolDOExecute_Response_error_id result(::mg400_msgs::srv::ToolDOExecute_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return Init_ToolDOExecute_Response_error_id(msg_);
  }

private:
  ::mg400_msgs::srv::ToolDOExecute_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::mg400_msgs::srv::ToolDOExecute_Response>()
{
  return mg400_msgs::srv::builder::Init_ToolDOExecute_Response_result();
}

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__TOOL_DO_EXECUTE__BUILDER_HPP_
