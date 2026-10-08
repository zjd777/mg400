// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__type_support.cpp.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/msg/detail/joint_mov_j__rosidl_typesupport_fastrtps_cpp.hpp"
#include "mg400_msgs/msg/detail/joint_mov_j__struct.hpp"

#include <limits>
#include <stdexcept>
#include <string>
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_fastrtps_cpp/identifier.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_fastrtps_cpp/wstring_conversion.hpp"
#include "fastcdr/Cdr.h"


// forward declaration of message dependencies and their conversion functions

namespace mg400_msgs
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
cdr_serialize(
  const mg400_msgs::msg::JointMovJ & ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Member: joint_angles
  {
    cdr << ros_message.joint_angles;
  }
  // Member: set_speed_j
  cdr << (ros_message.set_speed_j ? true : false);
  // Member: speed_j
  cdr << ros_message.speed_j;
  // Member: set_acc_j
  cdr << (ros_message.set_acc_j ? true : false);
  // Member: acc_j
  cdr << ros_message.acc_j;
  // Member: set_cp
  cdr << (ros_message.set_cp ? true : false);
  // Member: cp
  cdr << ros_message.cp;
  return true;
}

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  mg400_msgs::msg::JointMovJ & ros_message)
{
  // Member: joint_angles
  {
    cdr >> ros_message.joint_angles;
  }

  // Member: set_speed_j
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.set_speed_j = tmp ? true : false;
  }

  // Member: speed_j
  cdr >> ros_message.speed_j;

  // Member: set_acc_j
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.set_acc_j = tmp ? true : false;
  }

  // Member: acc_j
  cdr >> ros_message.acc_j;

  // Member: set_cp
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message.set_cp = tmp ? true : false;
  }

  // Member: cp
  cdr >> ros_message.cp;

  return true;
}  // NOLINT(readability/fn_size)

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
get_serialized_size(
  const mg400_msgs::msg::JointMovJ & ros_message,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Member: joint_angles
  {
    size_t array_size = 4;
    size_t item_size = sizeof(ros_message.joint_angles[0]);
    current_alignment += array_size * item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: set_speed_j
  {
    size_t item_size = sizeof(ros_message.set_speed_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: speed_j
  {
    size_t item_size = sizeof(ros_message.speed_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: set_acc_j
  {
    size_t item_size = sizeof(ros_message.set_acc_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: acc_j
  {
    size_t item_size = sizeof(ros_message.acc_j);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: set_cp
  {
    size_t item_size = sizeof(ros_message.set_cp);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: cp
  {
    size_t item_size = sizeof(ros_message.cp);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
max_serialized_size_JointMovJ(
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


  // Member: joint_angles
  {
    size_t array_size = 4;

    last_member_size = array_size * sizeof(uint64_t);
    current_alignment += array_size * sizeof(uint64_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint64_t));
  }

  // Member: set_speed_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: speed_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: set_acc_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: acc_j
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: set_cp
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  // Member: cp
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
    using DataType = mg400_msgs::msg::JointMovJ;
    is_plain =
      (
      offsetof(DataType, cp) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static bool _JointMovJ__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  auto typed_message =
    static_cast<const mg400_msgs::msg::JointMovJ *>(
    untyped_ros_message);
  return cdr_serialize(*typed_message, cdr);
}

static bool _JointMovJ__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  auto typed_message =
    static_cast<mg400_msgs::msg::JointMovJ *>(
    untyped_ros_message);
  return cdr_deserialize(cdr, *typed_message);
}

static uint32_t _JointMovJ__get_serialized_size(
  const void * untyped_ros_message)
{
  auto typed_message =
    static_cast<const mg400_msgs::msg::JointMovJ *>(
    untyped_ros_message);
  return static_cast<uint32_t>(get_serialized_size(*typed_message, 0));
}

static size_t _JointMovJ__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_JointMovJ(full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}

static message_type_support_callbacks_t _JointMovJ__callbacks = {
  "mg400_msgs::msg",
  "JointMovJ",
  _JointMovJ__cdr_serialize,
  _JointMovJ__cdr_deserialize,
  _JointMovJ__get_serialized_size,
  _JointMovJ__max_serialized_size
};

static rosidl_message_type_support_t _JointMovJ__handle = {
  rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
  &_JointMovJ__callbacks,
  get_message_typesupport_handle_function,
};

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace mg400_msgs

namespace rosidl_typesupport_fastrtps_cpp
{

template<>
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_EXPORT_mg400_msgs
const rosidl_message_type_support_t *
get_message_type_support_handle<mg400_msgs::msg::JointMovJ>()
{
  return &mg400_msgs::msg::typesupport_fastrtps_cpp::_JointMovJ__handle;
}

}  // namespace rosidl_typesupport_fastrtps_cpp

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, mg400_msgs, msg, JointMovJ)() {
  return &mg400_msgs::msg::typesupport_fastrtps_cpp::_JointMovJ__handle;
}

#ifdef __cplusplus
}
#endif
