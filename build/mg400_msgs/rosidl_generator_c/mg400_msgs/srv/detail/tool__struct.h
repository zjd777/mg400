// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/Tool.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__TOOL__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__TOOL__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'tool'
#include "mg400_msgs/msg/detail/tool__struct.h"

/// Struct defined in srv/Tool in the package mg400_msgs.
typedef struct mg400_msgs__srv__Tool_Request
{
  mg400_msgs__msg__Tool tool;
} mg400_msgs__srv__Tool_Request;

// Struct for a sequence of mg400_msgs__srv__Tool_Request.
typedef struct mg400_msgs__srv__Tool_Request__Sequence
{
  mg400_msgs__srv__Tool_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__Tool_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/Tool in the package mg400_msgs.
typedef struct mg400_msgs__srv__Tool_Response
{
  bool result;
  int32_t error_id;
} mg400_msgs__srv__Tool_Response;

// Struct for a sequence of mg400_msgs__srv__Tool_Response.
typedef struct mg400_msgs__srv__Tool_Response__Sequence
{
  mg400_msgs__srv__Tool_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__Tool_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__TOOL__STRUCT_H_
