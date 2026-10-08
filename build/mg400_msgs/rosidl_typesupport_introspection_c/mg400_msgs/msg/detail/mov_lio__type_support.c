// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:msg/MovLIO.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/msg/detail/mov_lio__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/msg/detail/mov_lio__functions.h"
#include "mg400_msgs/msg/detail/mov_lio__struct.h"


// Include directives for member types
// Member `pose`
#include "geometry_msgs/msg/pose_stamped.h"
// Member `pose`
#include "geometry_msgs/msg/detail/pose_stamped__rosidl_typesupport_introspection_c.h"
// Member `mode`
#include "mg400_msgs/msg/distance_mode.h"
// Member `mode`
#include "mg400_msgs/msg/detail/distance_mode__rosidl_typesupport_introspection_c.h"
// Member `index`
#include "mg400_msgs/msg/do_index.h"
// Member `index`
#include "mg400_msgs/msg/detail/do_index__rosidl_typesupport_introspection_c.h"
// Member `status`
#include "mg400_msgs/msg/do_status.h"
// Member `status`
#include "mg400_msgs/msg/detail/do_status__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__msg__MovLIO__init(message_memory);
}

void mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_fini_function(void * message_memory)
{
  mg400_msgs__msg__MovLIO__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_member_array[9] = {
  {
    "pose",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, pose),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "mode",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, mode),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "distance",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT32,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, distance),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "index",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, index),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "status",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, status),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "set_speed_l",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, set_speed_l),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "speed_l",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, speed_l),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "set_acc_l",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, set_acc_l),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "acc_l",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__msg__MovLIO, acc_l),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_members = {
  "mg400_msgs__msg",  // message namespace
  "MovLIO",  // message name
  9,  // number of fields
  sizeof(mg400_msgs__msg__MovLIO),
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_member_array,  // message members
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_type_support_handle = {
  0,
  &mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, MovLIO)() {
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, geometry_msgs, msg, PoseStamped)();
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, DistanceMode)();
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_member_array[3].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, DOIndex)();
  mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_member_array[4].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, DOStatus)();
  if (!mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__msg__MovLIO__rosidl_typesupport_introspection_c__MovLIO_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
