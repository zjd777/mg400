// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:msg/DOStatus.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/msg/detail/do_status__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/msg/detail/do_status__functions.h"
#include "mg400_msgs/msg/detail/do_status__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__msg__DOStatus__init(message_memory);
}

void mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_fini_function(void * message_memory)
{
  mg400_msgs__msg__DOStatus__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_member_array[1] = {
  {
    "status",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__DOStatus, status),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_members = {
  "mg400_msgs__msg",  // message namespace
  "DOStatus",  // message name
  1,  // number of fields
  sizeof(mg400_msgs__msg__DOStatus),
  mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_member_array,  // message members
  mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_type_support_handle = {
  0,
  &mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, DOStatus)() {
  if (!mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__msg__DOStatus__rosidl_typesupport_introspection_c__DOStatus_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
