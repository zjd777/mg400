// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/msg/detail/joint_mov_j__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "mg400_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "mg400_msgs/msg/detail/joint_mov_j__struct.h"
#include "mg400_msgs/msg/detail/joint_mov_j__functions.h"
#include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif


// forward declare type support functions


using _JointMovJ__ros_msg_type = mg400_msgs__msg__JointMovJ;

static bool _JointMovJ__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _JointMovJ__ros_msg_type * ros_message = static_cast<const _JointMovJ__ros_msg_type *>(untyped_ros_message);
  // Field name: joint_angles
  {
    size_t size = 4;
    auto array_ptr = ros_message->joint_angles;
    cdr.serializeArray(array_ptr, size);
  }

  // Field name: set_speed_j
  {
    cdr << (ros_message->set_speed_j ? true : false);
  }

  // Field name: speed_j
  {
    cdr << ros_message->speed_j;
  }

  // Field name: set_acc_j
  {
    cdr << (ros_message->set_acc_j ? true : false);
  }

  // Field name: acc_j
  {
    cdr << ros_message->acc_j;
  }

  // Field name: set_cp
  {
    cdr << (ros_message->set_cp ? true : false);
  }

  // Field name: cp
  {
    cdr << ros_message->cp;
  }

  return true;
}

static bool _JointMovJ__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _JointMovJ__ros_msg_type * ros_message = static_cast<_JointMovJ__ros_msg_type *>(untyped_ros_message);
  // Field name: joint_angles
  {
    size_t size = 4;
    auto array_ptr = ros_message->joint_angles;
    cdr.deserializeArray(array_ptr, size);
  }

  // Field name: set_speed_j
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->set_speed_j = tmp ? true : false;
  }

  // Field name: speed_j
  {
    cdr >> ros_message->speed_j;
  }

  // Field name: set_acc_j
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->set_acc_j = tmp ? true : false;
  }

  // Field name: acc_j
  {
    cdr >> ros_message->acc_j;
  }

  // Field name: set_cp
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->set_cp = tmp ? true : false;
  }

  // Field name: cp
  {
    cdr >> ros_message->cp;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_mg400_msgs
size_t get_serialized_size_mg400_msgs__msg__JointMovJ(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _JointMovJ__ros_msg_type * ros_message = static_cast<const _JointMovJ__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name joint_angles
  {
    size_t array_size = 4;
    auto array_ptr = ros_message->joint_angles;
    (void)array_ptr;
    size_t item_size = sizeof(array_ptr[0]);
    current_alignment += array_size * item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name set_speed_j
  {
    size_t item_size = sizeof(ros_message->set_speed_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name speed_j
  {
    size_t item_size = sizeof(ros_message->speed_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name set_acc_j
  {
    size_t item_size = sizeof(ros_message->set_acc_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name acc_j
  {
    size_t item_size = sizeof(ros_message->acc_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name set_cp
  {
    size_t item_size = sizeof(ros_message->set_cp);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name cp
  {
    size_t item_size = sizeof(ros_message->cp);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _JointMovJ__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_mg400_msgs__msg__JointMovJ(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_mg400_msgs
size_t max_serialized_size_mg400_msgs__msg__JointMovJ(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: joint_angles
  {
    size_t array_size = 4;

    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }
  // member: set_speed_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: speed_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: set_acc_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: acc_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: set_cp
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: cp
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = mg400_msgs__msg__JointMovJ;
    is_plain =
      (
      offsetof(DataType, cp) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _JointMovJ__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_mg400_msgs__msg__JointMovJ(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_JointMovJ = {
  "mg400_msgs::msg",
  "JointMovJ",
  _JointMovJ__cdr_serialize,
  _JointMovJ__cdr_deserialize,
  _JointMovJ__get_serialized_size,
  _JointMovJ__max_serialized_size
};

static rosidl_message_type_support_t _JointMovJ__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_JointMovJ,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, msg, JointMovJ)() {
  return &_JointMovJ__type_support;
}

#if defined(__cplusplus)
}
#endif
