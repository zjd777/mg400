// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/msg/detail/error_id__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/msg/detail/error_id__functions.h"
#include "mg400_msgs/msg/detail/error_id__struct.h"


// Include directives for member types
// Member `controller`
// Member `servo`
#include "mg400_msgs/msg/id_array.h"
// Member `controller`
// Member `servo`
#include "mg400_msgs/msg/detail/id_array__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__msg__ErrorID__init(message_memory);
}

void mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_fini_function(void * message_memory)
{
  mg400_msgs__msg__ErrorID__fini(message_memory);
}

size_t mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__size_function__ErrorID__servo(
  const void * untyped_member)
{
  (void)untyped_member;
  return 5;
}

const void * mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__get_const_function__ErrorID__servo(
  const void * untyped_member, size_t index)
{
  const mg400_msgs__msg__IDArray * member =
    (const mg400_msgs__msg__IDArray *)(untyped_member);
  return &member[index];
}

void * mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__get_function__ErrorID__servo(
  void * untyped_member, size_t index)
{
  mg400_msgs__msg__IDArray * member =
    (mg400_msgs__msg__IDArray *)(untyped_member);
  return &member[index];
}

void mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__fetch_function__ErrorID__servo(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const mg400_msgs__msg__IDArray * item =
    ((const mg400_msgs__msg__IDArray *)
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__get_const_function__ErrorID__servo(untyped_member, index));
  mg400_msgs__msg__IDArray * value =
    (mg400_msgs__msg__IDArray *)(untyped_value);
  *value = *item;
}

void mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__assign_function__ErrorID__servo(
  void * untyped_member, size_t index, const void * untyped_value)
{
  mg400_msgs__msg__IDArray * item =
    ((mg400_msgs__msg__IDArray *)
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__get_function__ErrorID__servo(untyped_member, index));
  const mg400_msgs__msg__IDArray * value =
    (const mg400_msgs__msg__IDArray *)(untyped_value);
  *item = *value;
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_member_array[2] = {
  {
    "controller",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__ErrorID, controller),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "servo",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    5,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__ErrorID, servo),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__size_function__ErrorID__servo,  // size() function pointer
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__get_const_function__ErrorID__servo,  // get_const(index) function pointer
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__get_function__ErrorID__servo,  // get(index) function pointer
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__fetch_function__ErrorID__servo,  // fetch(index, &value) function pointer
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__assign_function__ErrorID__servo,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_members = {
  "mg400_msgs__msg",  // message namespace
  "ErrorID",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__msg__ErrorID),
  mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_member_array,  // message members
  mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_type_support_handle = {
  0,
  &mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, ErrorID)() {
  mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, IDArray)();
  mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, IDArray)();
  if (!mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__msg__ErrorID__rosidl_typesupport_introspection_c__ErrorID_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
