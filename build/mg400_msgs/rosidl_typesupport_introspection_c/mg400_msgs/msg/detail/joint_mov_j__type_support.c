// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/msg/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/msg/detail/joint_mov_j__functions.h"
#include "mg400_msgs/msg/detail/joint_mov_j__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__msg__JointMovJ__init(message_memory);
}

void mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_fini_function(void * message_memory)
{
  mg400_msgs__msg__JointMovJ__fini(message_memory);
}

size_t mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__size_function__JointMovJ__joint_angles(
  const void * untyped_member)
{
  (void)untyped_member;
  return 4;
}

const void * mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__get_const_function__JointMovJ__joint_angles(
  const void * untyped_member, size_t index)
{
  const double * member =
    (const double *)(untyped_member);
  return &member[index];
}

void * mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__get_function__JointMovJ__joint_angles(
  void * untyped_member, size_t index)
{
  double * member =
    (double *)(untyped_member);
  return &member[index];
}

void mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__fetch_function__JointMovJ__joint_angles(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__get_const_function__JointMovJ__joint_angles(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__assign_function__JointMovJ__joint_angles(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__get_function__JointMovJ__joint_angles(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_member_array[7] = {
  {
    "joint_angles",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    4,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, joint_angles),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__size_function__JointMovJ__joint_angles,  // size() function pointer
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__get_const_function__JointMovJ__joint_angles,  // get_const(index) function pointer
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__get_function__JointMovJ__joint_angles,  // get(index) function pointer
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__fetch_function__JointMovJ__joint_angles,  // fetch(index, &value) function pointer
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__assign_function__JointMovJ__joint_angles,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "set_speed_j",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, set_speed_j),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "speed_j",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, speed_j),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "set_acc_j",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, set_acc_j),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "acc_j",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, acc_j),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "set_cp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, set_cp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "cp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__JointMovJ, cp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_members = {
  "mg400_msgs__msg",  // message namespace
  "JointMovJ",  // message name
  7,  // number of fields
  sizeof(mg400_msgs__msg__JointMovJ),
  mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_member_array,  // message members
  mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_type_support_handle = {
  0,
  &mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, JointMovJ)() {
  if (!mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__msg__JointMovJ__rosidl_typesupport_introspection_c__JointMovJ_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
