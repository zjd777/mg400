// generated from rosidl_typesupport_introspection_cpp/resource/idl__type_support.cpp.em
// with input from mg400_msgs:msg/MoveJog.idl
// generated code does not contain a copyright notice

#include "array"
#include "cstddef"
#include "string"
#include "vector"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_interface/macros.h"
#include "mg400_msgs/msg/detail/move_jog__struct.hpp"
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

void MoveJog_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) mg400_msgs::msg::MoveJog(_init);
}

void MoveJog_fini_function(void * message_memory)
{
  auto typed_message = static_cast<mg400_msgs::msg::MoveJog *>(message_memory);
  typed_message->~MoveJog();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember MoveJog_message_member_array[1] = {
  {
    "jog_mode",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs::msg::MoveJog, jog_mode),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr  // resize(index) function pointer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers MoveJog_message_members = {
  "mg400_msgs::msg",  // message namespace
  "MoveJog",  // message name
  1,  // number of fields
  sizeof(mg400_msgs::msg::MoveJog),
  MoveJog_message_member_array,  // message members
  MoveJog_init_function,  // function to initialize message memory (memory has to be allocated)
  MoveJog_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t MoveJog_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &MoveJog_message_members,
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
get_message_type_support_handle<mg400_msgs::msg::MoveJog>()
{
  return &::mg400_msgs::msg::rosidl_typesupport_introspection_cpp::MoveJog_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, mg400_msgs, msg, MoveJog)() {
  return &::mg400_msgs::msg::rosidl_typesupport_introspection_cpp::MoveJog_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif
