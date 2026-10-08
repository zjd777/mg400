// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:action/JointMovJ.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/action/detail/joint_mov_j__functions.h"
#include "mg400_msgs/action/detail/joint_mov_j__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_Goal__init(message_memory);
}

void mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_Goal__fini(message_memory);
}

size_t mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__size_function__JointMovJ_Goal__joint_angles(
  const void * untyped_member)
{
  (void)untyped_member;
  return 4;
}

const void * mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__get_const_function__JointMovJ_Goal__joint_angles(
  const void * untyped_member, size_t index)
{
  const double * member =
    (const double *)(untyped_member);
  return &member[index];
}

void * mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__get_function__JointMovJ_Goal__joint_angles(
  void * untyped_member, size_t index)
{
  double * member =
    (double *)(untyped_member);
  return &member[index];
}

void mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__fetch_function__JointMovJ_Goal__joint_angles(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__get_const_function__JointMovJ_Goal__joint_angles(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__assign_function__JointMovJ_Goal__joint_angles(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__get_function__JointMovJ_Goal__joint_angles(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_member_array[7] = {
  {
    "joint_angles",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    4,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_Goal, joint_angles),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__size_function__JointMovJ_Goal__joint_angles,  // size() function pointer
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__get_const_function__JointMovJ_Goal__joint_angles,  // get_const(index) function pointer
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__get_function__JointMovJ_Goal__joint_angles,  // get(index) function pointer
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__fetch_function__JointMovJ_Goal__joint_angles,  // fetch(index, &value) function pointer
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__assign_function__JointMovJ_Goal__joint_angles,  // assign(index, value) function pointer
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
    offsetof(mg400_msgs__action__JointMovJ_Goal, set_speed_j),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__JointMovJ_Goal, speed_j),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__JointMovJ_Goal, set_acc_j),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__JointMovJ_Goal, acc_j),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__JointMovJ_Goal, set_cp),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__JointMovJ_Goal, cp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_Goal",  // message name
  7,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_Goal),
  mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_Goal)() {
  if (!mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_Goal__rosidl_typesupport_introspection_c__JointMovJ_Goal_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


// Include directives for member types
// Member `error_id`
#include "mg400_msgs/msg/error_id.h"
// Member `error_id`
#include "mg400_msgs/msg/detail/error_id__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_Result__init(message_memory);
}

void mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_Result__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_member_array[2] = {
  {
    "result",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_Result, result),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "error_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_Result, error_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_Result",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_Result),
  mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_Result)() {
  mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, ErrorID)();
  if (!mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_Result__rosidl_typesupport_introspection_c__JointMovJ_Result_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_Feedback__init(message_memory);
}

void mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_Feedback__fini(message_memory);
}

size_t mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__size_function__JointMovJ_Feedback__current_angles(
  const void * untyped_member)
{
  (void)untyped_member;
  return 4;
}

const void * mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__get_const_function__JointMovJ_Feedback__current_angles(
  const void * untyped_member, size_t index)
{
  const double * member =
    (const double *)(untyped_member);
  return &member[index];
}

void * mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__get_function__JointMovJ_Feedback__current_angles(
  void * untyped_member, size_t index)
{
  double * member =
    (double *)(untyped_member);
  return &member[index];
}

void mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__fetch_function__JointMovJ_Feedback__current_angles(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__get_const_function__JointMovJ_Feedback__current_angles(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__assign_function__JointMovJ_Feedback__current_angles(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__get_function__JointMovJ_Feedback__current_angles(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_member_array[1] = {
  {
    "current_angles",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    4,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_Feedback, current_angles),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__size_function__JointMovJ_Feedback__current_angles,  // size() function pointer
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__get_const_function__JointMovJ_Feedback__current_angles,  // get_const(index) function pointer
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__get_function__JointMovJ_Feedback__current_angles,  // get(index) function pointer
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__fetch_function__JointMovJ_Feedback__current_angles,  // fetch(index, &value) function pointer
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__assign_function__JointMovJ_Feedback__current_angles,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_Feedback",  // message name
  1,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_Feedback),
  mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_Feedback)() {
  if (!mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_Feedback__rosidl_typesupport_introspection_c__JointMovJ_Feedback_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


// Include directives for member types
// Member `goal_id`
#include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
#include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `goal`
#include "mg400_msgs/action/joint_mov_j.h"
// Member `goal`
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_SendGoal_Request__init(message_memory);
}

void mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_SendGoal_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_SendGoal_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "goal",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_SendGoal_Request, goal),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_SendGoal_Request",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_SendGoal_Request),
  mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal_Request)() {
  mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_Goal)();
  if (!mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_SendGoal_Request__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


// Include directives for member types
// Member `stamp`
#include "builtin_interfaces/msg/time.h"
// Member `stamp`
#include "builtin_interfaces/msg/detail/time__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_SendGoal_Response__init(message_memory);
}

void mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_SendGoal_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_member_array[2] = {
  {
    "accepted",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_SendGoal_Response, accepted),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "stamp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_SendGoal_Response, stamp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_SendGoal_Response",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_SendGoal_Response),
  mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal_Response)() {
  mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, builtin_interfaces, msg, Time)();
  if (!mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_SendGoal_Response__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_members = {
  "mg400_msgs__action",  // service namespace
  "JointMovJ_SendGoal",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Request_message_type_support_handle,
  NULL  // response message
  // mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_Response_message_type_support_handle
};

static rosidl_service_type_support_t mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_type_support_handle = {
  0,
  &mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal)() {
  if (!mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_SendGoal_Response)()->data;
  }

  return &mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_SendGoal_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_GetResult_Request__init(message_memory);
}

void mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_GetResult_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_member_array[1] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_GetResult_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_GetResult_Request",  // message name
  1,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_GetResult_Request),
  mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult_Request)() {
  mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  if (!mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_GetResult_Request__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


// Include directives for member types
// Member `result`
// already included above
// #include "mg400_msgs/action/joint_mov_j.h"
// Member `result`
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_GetResult_Response__init(message_memory);
}

void mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_GetResult_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_member_array[2] = {
  {
    "status",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_GetResult_Response, status),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "result",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_GetResult_Response, result),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_GetResult_Response",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_GetResult_Response),
  mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult_Response)() {
  mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_Result)();
  if (!mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_GetResult_Response__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_members = {
  "mg400_msgs__action",  // service namespace
  "JointMovJ_GetResult",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Request_message_type_support_handle,
  NULL  // response message
  // mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_Response_message_type_support_handle
};

static rosidl_service_type_support_t mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_type_support_handle = {
  0,
  &mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult)() {
  if (!mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_GetResult_Response)()->data;
  }

  return &mg400_msgs__action__detail__joint_mov_j__rosidl_typesupport_introspection_c__JointMovJ_GetResult_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__functions.h"
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `feedback`
// already included above
// #include "mg400_msgs/action/joint_mov_j.h"
// Member `feedback`
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__JointMovJ_FeedbackMessage__init(message_memory);
}

void mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_fini_function(void * message_memory)
{
  mg400_msgs__action__JointMovJ_FeedbackMessage__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_FeedbackMessage, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "feedback",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__JointMovJ_FeedbackMessage, feedback),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_members = {
  "mg400_msgs__action",  // message namespace
  "JointMovJ_FeedbackMessage",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__JointMovJ_FeedbackMessage),
  mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_member_array,  // message members
  mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_type_support_handle = {
  0,
  &mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_FeedbackMessage)() {
  mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, JointMovJ_Feedback)();
  if (!mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__JointMovJ_FeedbackMessage__rosidl_typesupport_introspection_c__JointMovJ_FeedbackMessage_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
