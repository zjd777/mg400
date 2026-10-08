// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__type_support.cpp.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/msg/detail/command__rosidl_typesupport_fastrtps_cpp.hpp"
#include "mg400_msgs/msg/detail/command__struct.hpp"

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
bool cdr_serialize(
  const mg400_msgs::msg::MovJ &,
  eprosima::fastcdr::Cdr &);
bool cdr_deserialize(
  eprosima::fastcdr::Cdr &,
  mg400_msgs::msg::MovJ &);
size_t get_serialized_size(
  const mg400_msgs::msg::MovJ &,
  size_t current_alignment);
size_t
max_serialized_size_MovJ(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);
}  // namespace typesupport_fastrtps_cpp
}  // namespace msg
}  // namespace mg400_msgs

namespace mg400_msgs
{
namespace msg
{
namespace typesupport_fastrtps_cpp
{
bool cdr_serialize(
  const mg400_msgs::msg::MovL &,
  eprosima::fastcdr::Cdr &);
bool cdr_deserialize(
  eprosima::fastcdr::Cdr &,
  mg400_msgs::msg::MovL &);
size_t get_serialized_size(
  const mg400_msgs::msg::MovL &,
  size_t current_alignment);
size_t
max_serialized_size_MovL(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);
}  // namespace typesupport_fastrtps_cpp
}  // namespace msg
}  // namespace mg400_msgs

namespace mg400_msgs
{
namespace msg
{
namespace typesupport_fastrtps_cpp
{
bool cdr_serialize(
  const mg400_msgs::msg::JointMovJ &,
  eprosima::fastcdr::Cdr &);
bool cdr_deserialize(
  eprosima::fastcdr::Cdr &,
  mg400_msgs::msg::JointMovJ &);
size_t get_serialized_size(
  const mg400_msgs::msg::JointMovJ &,
  size_t current_alignment);
size_t
max_serialized_size_JointMovJ(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);
}  // namespace typesupport_fastrtps_cpp
}  // namespace msg
}  // namespace mg400_msgs

namespace mg400_msgs
{
namespace msg
{
namespace typesupport_fastrtps_cpp
{
bool cdr_serialize(
  const mg400_msgs::msg::MovJIO &,
  eprosima::fastcdr::Cdr &);
bool cdr_deserialize(
  eprosima::fastcdr::Cdr &,
  mg400_msgs::msg::MovJIO &);
size_t get_serialized_size(
  const mg400_msgs::msg::MovJIO &,
  size_t current_alignment);
size_t
max_serialized_size_MovJIO(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);
}  // namespace typesupport_fastrtps_cpp
}  // namespace msg
}  // namespace mg400_msgs

namespace mg400_msgs
{
namespace msg
{
namespace typesupport_fastrtps_cpp
{
bool cdr_serialize(
  const mg400_msgs::msg::MovLIO &,
  eprosima::fastcdr::Cdr &);
bool cdr_deserialize(
  eprosima::fastcdr::Cdr &,
  mg400_msgs::msg::MovLIO &);
size_t get_serialized_size(
  const mg400_msgs::msg::MovLIO &,
  size_t current_alignment);
size_t
max_serialized_size_MovLIO(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);
}  // namespace typesupport_fastrtps_cpp
}  // namespace msg
}  // namespace mg400_msgs


namespace mg400_msgs
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
cdr_serialize(
  const mg400_msgs::msg::Command & ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  // Member: command_type
  cdr << ros_message.command_type;
  // Member: mov_j_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_serialize(
    ros_message.mov_j_params,
    cdr);
  // Member: mov_l_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_serialize(
    ros_message.mov_l_params,
    cdr);
  // Member: joint_mov_j_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_serialize(
    ros_message.joint_mov_j_params,
    cdr);
  // Member: mov_jio_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_serialize(
    ros_message.mov_jio_params,
    cdr);
  // Member: mov_lio_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_serialize(
    ros_message.mov_lio_params,
    cdr);
  return true;
}

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  mg400_msgs::msg::Command & ros_message)
{
  // Member: command_type
  cdr >> ros_message.command_type;

  // Member: mov_j_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_deserialize(
    cdr, ros_message.mov_j_params);

  // Member: mov_l_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_deserialize(
    cdr, ros_message.mov_l_params);

  // Member: joint_mov_j_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_deserialize(
    cdr, ros_message.joint_mov_j_params);

  // Member: mov_jio_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_deserialize(
    cdr, ros_message.mov_jio_params);

  // Member: mov_lio_params
  mg400_msgs::msg::typesupport_fastrtps_cpp::cdr_deserialize(
    cdr, ros_message.mov_lio_params);

  return true;
}  // NOLINT(readability/fn_size)

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
get_serialized_size(
  const mg400_msgs::msg::Command & ros_message,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // Member: command_type
  {
    size_t item_size = sizeof(ros_message.command_type);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // Member: mov_j_params

  current_alignment +=
    mg400_msgs::msg::typesupport_fastrtps_cpp::get_serialized_size(
    ros_message.mov_j_params, current_alignment);
  // Member: mov_l_params

  current_alignment +=
    mg400_msgs::msg::typesupport_fastrtps_cpp::get_serialized_size(
    ros_message.mov_l_params, current_alignment);
  // Member: joint_mov_j_params

  current_alignment +=
    mg400_msgs::msg::typesupport_fastrtps_cpp::get_serialized_size(
    ros_message.joint_mov_j_params, current_alignment);
  // Member: mov_jio_params

  current_alignment +=
    mg400_msgs::msg::typesupport_fastrtps_cpp::get_serialized_size(
    ros_message.mov_jio_params, current_alignment);
  // Member: mov_lio_params

  current_alignment +=
    mg400_msgs::msg::typesupport_fastrtps_cpp::get_serialized_size(
    ros_message.mov_lio_params, current_alignment);

  return current_alignment - initial_alignment;
}

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_mg400_msgs
max_serialized_size_Command(
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


  // Member: command_type
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint16_t);
    current_alignment += array_size * sizeof(uint16_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint16_t));
  }

  // Member: mov_j_params
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size =
        mg400_msgs::msg::typesupport_fastrtps_cpp::max_serialized_size_MovJ(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Member: mov_l_params
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size =
        mg400_msgs::msg::typesupport_fastrtps_cpp::max_serialized_size_MovL(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Member: joint_mov_j_params
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size =
        mg400_msgs::msg::typesupport_fastrtps_cpp::max_serialized_size_JointMovJ(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Member: mov_jio_params
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size =
        mg400_msgs::msg::typesupport_fastrtps_cpp::max_serialized_size_MovJIO(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  // Member: mov_lio_params
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size =
        mg400_msgs::msg::typesupport_fastrtps_cpp::max_serialized_size_MovLIO(
        inner_full_bounded, inner_is_plain, current_alignment);
      last_member_size += inner_size;
      current_alignment += inner_size;
      full_bounded &= inner_full_bounded;
      is_plain &= inner_is_plain;
    }
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = mg400_msgs::msg::Command;
    is_plain =
      (
      offsetof(DataType, mov_lio_params) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static bool _Command__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  auto typed_message =
    static_cast<const mg400_msgs::msg::Command *>(
    untyped_ros_message);
  return cdr_serialize(*typed_message, cdr);
}

static bool _Command__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  auto typed_message =
    static_cast<mg400_msgs::msg::Command *>(
    untyped_ros_message);
  return cdr_deserialize(cdr, *typed_message);
}

static uint32_t _Command__get_serialized_size(
  const void * untyped_ros_message)
{
  auto typed_message =
    static_cast<const mg400_msgs::msg::Command *>(
    untyped_ros_message);
  return static_cast<uint32_t>(get_serialized_size(*typed_message, 0));
}

static size_t _Command__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_Command(full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}

static message_type_support_callbacks_t _Command__callbacks = {
  "mg400_msgs::msg",
  "Command",
  _Command__cdr_serialize,
  _Command__cdr_deserialize,
  _Command__get_serialized_size,
  _Command__max_serialized_size
};

static rosidl_message_type_support_t _Command__handle = {
  rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
  &_Command__callbacks,
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
get_message_type_support_handle<mg400_msgs::msg::Command>()
{
  return &mg400_msgs::msg::typesupport_fastrtps_cpp::_Command__handle;
}

}  // namespace rosidl_typesupport_fastrtps_cpp

#ifdef __cplusplus
extern "C"
{
#endif

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, mg400_msgs, msg, Command)() {
  return &mg400_msgs::msg::typesupport_fastrtps_cpp::_Command__handle;
}

#ifdef __cplusplus
}
#endif
