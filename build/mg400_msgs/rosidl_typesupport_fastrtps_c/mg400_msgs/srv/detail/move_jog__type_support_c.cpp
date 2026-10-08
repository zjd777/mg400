// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from mg400_msgs:srv/MoveJog.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/srv/detail/move_jog__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "mg400_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "mg400_msgs/srv/detail/move_jog__struct.h"
#include "mg400_msgs/srv/detail/move_jog__functions.h"
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

#include "mg400_msgs/msg/detail/move_jog__functions.h"  // jog

// forward declare type support functions
size_t get_serialized_size_mg400_msgs__msg__MoveJog(
  const void * untyped_ros_message,
  size_t current_alignment);

size_t max_serialized_size_mg400_msgs__msg__MoveJog(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, msg, MoveJog)();


using _MoveJog_Request__ros_msg_type = mg400_msgs__srv__MoveJog_Request;

static bool _MoveJog_Request__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _MoveJog_Request__ros_msg_type * ros_message = static_cast<const _MoveJog_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: jog
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, mg400_msgs, msg, MoveJog
      )()->data);
    if (!callbacks->cdr_serialize(
        &ros_message->jog, cdr))
    {
      return false;
    }
  }

  return true;
}

static bool _MoveJog_Request__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _MoveJog_Request__ros_msg_type * ros_message = static_cast<_MoveJog_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: jog
  {
    const message_type_support_callbacks_t * callbacks =
      static_cast<const message_type_support_callbacks_t *>(
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(
        rosidl_typesupport_fastrtps_c, mg400_msgs, msg, MoveJog
      )()->data);
    if (!callbacks->cdr_deserialize(
        cdr, &ros_message->jog))
    {
      return false;
    }
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_mg400_msgs
size_t get_serialized_size_mg400_msgs__srv__MoveJog_Request(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _MoveJog_Request__ros_msg_type * ros_message = static_cast<const _MoveJog_Request__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name jog

  current_alignment += get_serialized_size_mg400_msgs__msg__MoveJog(
    &(ros_message->jog), current_alignment);

  return current_alignment - initial_alignment;
}

static uint32_t _MoveJog_Request__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_mg400_msgs__srv__MoveJog_Request(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_mg400_msgs
size_t max_serialized_size_mg400_msgs__srv__MoveJog_Request(
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

  // member: jog
  {
    size_t array_size = 1;


    last_member_size = 0;
    for (size_t index = 0; index < array_size; ++index) {
      bool inner_full_bounded;
      bool inner_is_plain;
      size_t inner_size;
      inner_size =
        max_serialized_size_mg400_msgs__msg__MoveJog(
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
    using DataType = mg400_msgs__srv__MoveJog_Request;
    is_plain =
      (
      offsetof(DataType, jog) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _MoveJog_Request__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_mg400_msgs__srv__MoveJog_Request(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_MoveJog_Request = {
  "mg400_msgs::srv",
  "MoveJog_Request",
  _MoveJog_Request__cdr_serialize,
  _MoveJog_Request__cdr_deserialize,
  _MoveJog_Request__get_serialized_size,
  _MoveJog_Request__max_serialized_size
};

static rosidl_message_type_support_t _MoveJog_Request__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_MoveJog_Request,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, srv, MoveJog_Request)() {
  return &_MoveJog_Request__type_support;
}

#if defined(__cplusplus)
}
#endif

// already included above
// #include <cassert>
// already included above
// #include <limits>
// already included above
// #include <string>
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
// already included above
// #include "mg400_msgs/srv/detail/move_jog__struct.h"
// already included above
// #include "mg400_msgs/srv/detail/move_jog__functions.h"
// already included above
// #include "fastcdr/Cdr.h"

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


using _MoveJog_Response__ros_msg_type = mg400_msgs__srv__MoveJog_Response;

static bool _MoveJog_Response__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _MoveJog_Response__ros_msg_type * ros_message = static_cast<const _MoveJog_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: error_id
  {
    cdr << ros_message->error_id;
  }

  return true;
}

static bool _MoveJog_Response__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _MoveJog_Response__ros_msg_type * ros_message = static_cast<_MoveJog_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: error_id
  {
    cdr >> ros_message->error_id;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_mg400_msgs
size_t get_serialized_size_mg400_msgs__srv__MoveJog_Response(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _MoveJog_Response__ros_msg_type * ros_message = static_cast<const _MoveJog_Response__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name error_id
  {
    size_t item_size = sizeof(ros_message->error_id);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _MoveJog_Response__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_mg400_msgs__srv__MoveJog_Response(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_mg400_msgs
size_t max_serialized_size_mg400_msgs__srv__MoveJog_Response(
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

  // member: error_id
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint32_t);
    current_alignment += array_size * sizeof(uint32_t) +
      eprosima::fastcdr::Cdr::alignment(current_alignment, sizeof(uint32_t));
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = mg400_msgs__srv__MoveJog_Response;
    is_plain =
      (
      offsetof(DataType, error_id) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _MoveJog_Response__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_mg400_msgs__srv__MoveJog_Response(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_MoveJog_Response = {
  "mg400_msgs::srv",
  "MoveJog_Response",
  _MoveJog_Response__cdr_serialize,
  _MoveJog_Response__cdr_deserialize,
  _MoveJog_Response__get_serialized_size,
  _MoveJog_Response__max_serialized_size
};

static rosidl_message_type_support_t _MoveJog_Response__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_MoveJog_Response,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, srv, MoveJog_Response)() {
  return &_MoveJog_Response__type_support;
}

#if defined(__cplusplus)
}
#endif

#include "rosidl_typesupport_fastrtps_cpp/service_type_support.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "mg400_msgs/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "mg400_msgs/srv/move_jog.h"

#if defined(__cplusplus)
extern "C"
{
#endif

static service_type_support_callbacks_t MoveJog__callbacks = {
  "mg400_msgs::srv",
  "MoveJog",
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, srv, MoveJog_Request)(),
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, srv, MoveJog_Response)(),
};

static rosidl_service_type_support_t MoveJog__handle = {
  rosidl_typesupport_fastrtps_c__identifier,
  &MoveJog__callbacks,
  get_service_typesupport_handle_function,
};

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, mg400_msgs, srv, MoveJog)() {
  return &MoveJog__handle;
}

#if defined(__cplusplus)
}
#endif
