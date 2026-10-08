// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from mg400_msgs:action/CommandQueue.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
#include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "mg400_msgs/action/detail/command_queue__functions.h"
#include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `commands`
#include "mg400_msgs/msg/command.h"
// Member `commands`
#include "mg400_msgs/msg/detail/command__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_Goal__init(message_memory);
}

void mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_Goal__fini(message_memory);
}

size_t mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__size_function__CommandQueue_Goal__commands(
  const void * untyped_member)
{
  const mg400_msgs__msg__Command__Sequence * member =
    (const mg400_msgs__msg__Command__Sequence *)(untyped_member);
  return member->size;
}

const void * mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__get_const_function__CommandQueue_Goal__commands(
  const void * untyped_member, size_t index)
{
  const mg400_msgs__msg__Command__Sequence * member =
    (const mg400_msgs__msg__Command__Sequence *)(untyped_member);
  return &member->data[index];
}

void * mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__get_function__CommandQueue_Goal__commands(
  void * untyped_member, size_t index)
{
  mg400_msgs__msg__Command__Sequence * member =
    (mg400_msgs__msg__Command__Sequence *)(untyped_member);
  return &member->data[index];
}

void mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__fetch_function__CommandQueue_Goal__commands(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const mg400_msgs__msg__Command * item =
    ((const mg400_msgs__msg__Command *)
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__get_const_function__CommandQueue_Goal__commands(untyped_member, index));
  mg400_msgs__msg__Command * value =
    (mg400_msgs__msg__Command *)(untyped_value);
  *value = *item;
}

void mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__assign_function__CommandQueue_Goal__commands(
  void * untyped_member, size_t index, const void * untyped_value)
{
  mg400_msgs__msg__Command * item =
    ((mg400_msgs__msg__Command *)
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__get_function__CommandQueue_Goal__commands(untyped_member, index));
  const mg400_msgs__msg__Command * value =
    (const mg400_msgs__msg__Command *)(untyped_value);
  *item = *value;
}

bool mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__resize_function__CommandQueue_Goal__commands(
  void * untyped_member, size_t size)
{
  mg400_msgs__msg__Command__Sequence * member =
    (mg400_msgs__msg__Command__Sequence *)(untyped_member);
  mg400_msgs__msg__Command__Sequence__fini(member);
  return mg400_msgs__msg__Command__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_member_array[1] = {
  {
    "commands",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_Goal, commands),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__size_function__CommandQueue_Goal__commands,  // size() function pointer
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__get_const_function__CommandQueue_Goal__commands,  // get_const(index) function pointer
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__get_function__CommandQueue_Goal__commands,  // get(index) function pointer
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__fetch_function__CommandQueue_Goal__commands,  // fetch(index, &value) function pointer
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__assign_function__CommandQueue_Goal__commands,  // assign(index, value) function pointer
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__resize_function__CommandQueue_Goal__commands  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_Goal",  // message name
  1,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_Goal),
  mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_Goal)() {
  mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, Command)();
  if (!mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_Goal__rosidl_typesupport_introspection_c__CommandQueue_Goal_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `error_id`
#include "mg400_msgs/msg/error_id.h"
// Member `error_id`
#include "mg400_msgs/msg/detail/error_id__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_Result__init(message_memory);
}

void mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_Result__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_member_array[2] = {
  {
    "result",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_Result, result),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__CommandQueue_Result, error_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_Result",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_Result),
  mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_Result)() {
  mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, msg, ErrorID)();
  if (!mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_Result__rosidl_typesupport_introspection_c__CommandQueue_Result_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `current_pose`
#include "geometry_msgs/msg/pose_stamped.h"
// Member `current_pose`
#include "geometry_msgs/msg/detail/pose_stamped__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_Feedback__init(message_memory);
}

void mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_Feedback__fini(message_memory);
}

size_t mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__size_function__CommandQueue_Feedback__current_angles(
  const void * untyped_member)
{
  (void)untyped_member;
  return 4;
}

const void * mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__get_const_function__CommandQueue_Feedback__current_angles(
  const void * untyped_member, size_t index)
{
  const double * member =
    (const double *)(untyped_member);
  return &member[index];
}

void * mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__get_function__CommandQueue_Feedback__current_angles(
  void * untyped_member, size_t index)
{
  double * member =
    (double *)(untyped_member);
  return &member[index];
}

void mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__fetch_function__CommandQueue_Feedback__current_angles(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const double * item =
    ((const double *)
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__get_const_function__CommandQueue_Feedback__current_angles(untyped_member, index));
  double * value =
    (double *)(untyped_value);
  *value = *item;
}

void mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__assign_function__CommandQueue_Feedback__current_angles(
  void * untyped_member, size_t index, const void * untyped_value)
{
  double * item =
    ((double *)
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__get_function__CommandQueue_Feedback__current_angles(untyped_member, index));
  const double * value =
    (const double *)(untyped_value);
  *item = *value;
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_member_array[2] = {
  {
    "current_pose",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_Feedback, current_pose),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "current_angles",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    4,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_Feedback, current_angles),  // bytes offset in struct
    NULL,  // default value
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__size_function__CommandQueue_Feedback__current_angles,  // size() function pointer
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__get_const_function__CommandQueue_Feedback__current_angles,  // get_const(index) function pointer
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__get_function__CommandQueue_Feedback__current_angles,  // get(index) function pointer
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__fetch_function__CommandQueue_Feedback__current_angles,  // fetch(index, &value) function pointer
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__assign_function__CommandQueue_Feedback__current_angles,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_Feedback",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_Feedback),
  mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_Feedback)() {
  mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, geometry_msgs, msg, PoseStamped)();
  if (!mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_Feedback__rosidl_typesupport_introspection_c__CommandQueue_Feedback_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `goal_id`
#include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
#include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `goal`
#include "mg400_msgs/action/command_queue.h"
// Member `goal`
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_SendGoal_Request__init(message_memory);
}

void mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_SendGoal_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_SendGoal_Request, goal_id),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__CommandQueue_SendGoal_Request, goal),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_SendGoal_Request",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_SendGoal_Request),
  mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal_Request)() {
  mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_Goal)();
  if (!mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_SendGoal_Request__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `stamp`
#include "builtin_interfaces/msg/time.h"
// Member `stamp`
#include "builtin_interfaces/msg/detail/time__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_SendGoal_Response__init(message_memory);
}

void mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_SendGoal_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_member_array[2] = {
  {
    "accepted",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_SendGoal_Response, accepted),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__CommandQueue_SendGoal_Response, stamp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_SendGoal_Response",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_SendGoal_Response),
  mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal_Response)() {
  mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, builtin_interfaces, msg, Time)();
  if (!mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_SendGoal_Response__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_members = {
  "mg400_msgs__action",  // service namespace
  "CommandQueue_SendGoal",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Request_message_type_support_handle,
  NULL  // response message
  // mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_Response_message_type_support_handle
};

static rosidl_service_type_support_t mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_type_support_handle = {
  0,
  &mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal)() {
  if (!mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_SendGoal_Response)()->data;
  }

  return &mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_SendGoal_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


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

void mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_GetResult_Request__init(message_memory);
}

void mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_GetResult_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_member_array[1] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_GetResult_Request, goal_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_GetResult_Request",  // message name
  1,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_GetResult_Request),
  mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult_Request)() {
  mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  if (!mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_GetResult_Request__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `result`
// already included above
// #include "mg400_msgs/action/command_queue.h"
// Member `result`
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_GetResult_Response__init(message_memory);
}

void mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_GetResult_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_member_array[2] = {
  {
    "status",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_GetResult_Response, status),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__CommandQueue_GetResult_Response, result),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_GetResult_Response",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_GetResult_Response),
  mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult_Response)() {
  mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_Result)();
  if (!mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_GetResult_Response__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_members = {
  "mg400_msgs__action",  // service namespace
  "CommandQueue_GetResult",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Request_message_type_support_handle,
  NULL  // response message
  // mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_Response_message_type_support_handle
};

static rosidl_service_type_support_t mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_type_support_handle = {
  0,
  &mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult)() {
  if (!mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_GetResult_Response)()->data;
  }

  return &mg400_msgs__action__detail__command_queue__rosidl_typesupport_introspection_c__CommandQueue_GetResult_service_type_support_handle;
}

// already included above
// #include <stddef.h>
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__functions.h"
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/uuid.h"
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__rosidl_typesupport_introspection_c.h"
// Member `feedback`
// already included above
// #include "mg400_msgs/action/command_queue.h"
// Member `feedback`
// already included above
// #include "mg400_msgs/action/detail/command_queue__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  mg400_msgs__action__CommandQueue_FeedbackMessage__init(message_memory);
}

void mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_fini_function(void * message_memory)
{
  mg400_msgs__action__CommandQueue_FeedbackMessage__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_member_array[2] = {
  {
    "goal_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(mg400_msgs__action__CommandQueue_FeedbackMessage, goal_id),  // bytes offset in struct
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
    offsetof(mg400_msgs__action__CommandQueue_FeedbackMessage, feedback),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_members = {
  "mg400_msgs__action",  // message namespace
  "CommandQueue_FeedbackMessage",  // message name
  2,  // number of fields
  sizeof(mg400_msgs__action__CommandQueue_FeedbackMessage),
  mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_member_array,  // message members
  mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_init_function,  // function to initialize message memory (memory has to be allocated)
  mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_type_support_handle = {
  0,
  &mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_FeedbackMessage)() {
  mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, unique_identifier_msgs, msg, UUID)();
  mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, mg400_msgs, action, CommandQueue_Feedback)();
  if (!mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_type_support_handle.typesupport_identifier) {
    mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &mg400_msgs__action__CommandQueue_FeedbackMessage__rosidl_typesupport_introspection_c__CommandQueue_FeedbackMessage_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
