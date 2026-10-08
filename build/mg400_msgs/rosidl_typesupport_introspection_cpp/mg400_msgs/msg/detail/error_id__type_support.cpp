// generated from rosidl_typesupport_introspection_cpp/resource/idl__type_support.cpp.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#include "array"
#include "cstddef"
#include "string"
#include "vector"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_interface/macros.h"
#include "mg400_msgs/msg/detail/error_id__struct.hpp"
#include "rosidl_typesupport_introspection_cpp/field_types.hpp"
#include "rosidl_typesupport_introspection_cpp/identifier.hpp"
#include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
#include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace mg400_msgs
{

namespace msg
{

namespace rosidl_typesupport_introspection_cpp
{

void ErrorID_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) mg400_msgs::msg::ErrorID(_init);
}

void ErrorID_fini_function(void * message_memory)
{
  auto typed_message = static_cast<mg400_msgs::msg::ErrorID *>(message_memory);
  typed_message->~ErrorID();
}

size_t size_function__ErrorID__servo(const void * untyped_member)
{
  (void)untyped_member;
  return 5;
}

const void * get_const_function__ErrorID__servo(const void * untyped_member, size_t index)
{
  const auto & member =
    *reinterpret_cast<const std::array<mg400_msgs::msg::IDArray, 5> *>(untyped_member);
  return &member[index];
}

void * get_function__ErrorID__servo(void * untyped_member, size_t index)
{
  auto & member =
    *reinterpret_cast<std::array<mg400_msgs::msg::IDArray, 5> *>(untyped_member);
  return &member[index];
}

void fetch_function__ErrorID__servo(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const auto & item = *reinterpret_cast<const mg400_msgs::msg::IDArray *>(
    get_const_function__ErrorID__servo(untyped_member, index));
  auto & value = *reinterpret_cast<mg400_msgs::msg::IDArray *>(untyped_value);
  value = item;
}

void assign_function__ErrorID__servo(
  void * untyped_member, size_t index, const void * untyped_value)
{
  auto & item = *reinterpret_cast<mg400_msgs::msg::IDArray *>(
    get_function__ErrorID__servo(untyped_member, index));
  const auto & value = *reinterpret_cast<const mg400_msgs::msg::IDArray *>(untyped_value);
  item = value;
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember ErrorID_message_member_array[2] = {
  {
    "controller",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<mg400_msgs::msg::IDArray>(),  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs::msg::ErrorID, controller),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr  // resize(index) function pointer
  },
  {
    "servo",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<mg400_msgs::msg::IDArray>(),  // members of sub message
    true,  // is array
    5,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs::msg::ErrorID, servo),  // bytes offset in struct
    nullptr,  // default value
    size_function__ErrorID__servo,  // size() function pointer
    get_const_function__ErrorID__servo,  // get_const(index) function pointer
    get_function__ErrorID__servo,  // get(index) function pointer
    fetch_function__ErrorID__servo,  // fetch(index, &value) function pointer
    assign_function__ErrorID__servo,  // assign(index, value) function pointer
    nullptr  // resize(index) function pointer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers ErrorID_message_members = {
  "mg400_msgs::msg",  // message namespace
  "ErrorID",  // message name
  2,  // number of fields
  sizeof(mg400_msgs::msg::ErrorID),
  ErrorID_message_member_array,  // message members
  ErrorID_init_function,  // function to initialize message memory (memory has to be allocated)
  ErrorID_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t ErrorID_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &ErrorID_message_members,
  get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace msg

}  // namespace mg400_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<mg400_msgs::msg::ErrorID>()
{
  return &::mg400_msgs::msg::rosidl_typesupport_introspection_cpp::ErrorID_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, mg400_msgs, msg, ErrorID)() {
  return &::mg400_msgs::msg::rosidl_typesupport_introspection_cpp::ErrorID_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif
