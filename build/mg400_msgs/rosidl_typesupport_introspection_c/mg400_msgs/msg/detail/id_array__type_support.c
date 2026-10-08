// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:msg/IDArray.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/msg/detail/id_array__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/msg/detail/id_array__functions.h"
#include "mg400_msgs/msg/detail/id_array__struct.h"


// Include directives for member types
// Member `ids`
#include "rosidl_runtime_c/primitives_sequence_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__msg__IDArray__init(message_memory);
}

void mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_fini_function(void * message_memory)
{
  mg400_msgs__msg__IDArray__fini(message_memory);
}

size_t mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__size_function__IDArray__ids(
  const void * untyped_member)
{
  const rosidl_runtime_c__int32__Sequence * member =
    (const rosidl_runtime_c__int32__Sequence *)(untyped_member);
  return member->size;
}

const void * mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__get_const_function__IDArray__ids(
  const void * untyped_member, size_t index)
{
  const rosidl_runtime_c__int32__Sequence * member =
    (const rosidl_runtime_c__int32__Sequence *)(untyped_member);
  return &member->data[index];
}

void * mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__get_function__IDArray__ids(
  void * untyped_member, size_t index)
{
  rosidl_runtime_c__int32__Sequence * member =
    (rosidl_runtime_c__int32__Sequence *)(untyped_member);
  return &member->data[index];
}

void mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__fetch_function__IDArray__ids(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const int32_t * item =
    ((const int32_t *)
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__get_const_function__IDArray__ids(untyped_member, index));
  int32_t * value =
    (int32_t *)(untyped_value);
  *value = *item;
}

void mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__assign_function__IDArray__ids(
  void * untyped_member, size_t index, const void * untyped_value)
{
  int32_t * item =
    ((int32_t *)
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__get_function__IDArray__ids(untyped_member, index));
  const int32_t * value =
    (const int32_t *)(untyped_value);
  *item = *value;
}

bool mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__resize_function__IDArray__ids(
  void * untyped_member, size_t size)
{
  rosidl_runtime_c__int32__Sequence * member =
    (rosidl_runtime_c__int32__Sequence *)(untyped_member);
  rosidl_runtime_c__int32__Sequence__fini(member);
  return rosidl_runtime_c__int32__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_member_array[1] = {
  {
    "ids",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT32,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__IDArray, ids),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__size_function__IDArray__ids,  // size() function pointer
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__get_const_function__IDArray__ids,  // get_const(index) function pointer
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__get_function__IDArray__ids,  // get(index) function pointer
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__fetch_function__IDArray__ids,  // fetch(index, &value) function pointer
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__assign_function__IDArray__ids,  // assign(index, value) function pointer
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__resize_function__IDArray__ids  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_members = {
  "mg400_msgs__msg",  // message namespace
  "IDArray",  // message name
  1,  // number of fields
  sizeof(mg400_msgs__msg__IDArray),
  mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_member_array,  // message members
  mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_type_support_handle = {
  0,
  &mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, IDArray)() {
  if (!mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__msg__IDArray__rosidl_typesupport_introspection_c__IDArray_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
