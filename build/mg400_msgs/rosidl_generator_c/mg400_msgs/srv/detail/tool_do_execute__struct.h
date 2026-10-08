// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/ToolDOExecute.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__TOOL_DO_EXECUTE__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__TOOL_DO_EXECUTE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'index'
#include "mg400_msgs/msg/detail/tool_do_index__struct.h"
// Member 'status'
#include "mg400_msgs/msg/detail/do_status__struct.h"

/// Struct defined in srv/ToolDOExecute in the package mg400_msgs.
typedef struct mg400_msgs__srv__ToolDOExecute_Request
{
  mg400_msgs__msg__ToolDOIndex index;
  mg400_msgs__msg__DOStatus status;
} mg400_msgs__srv__ToolDOExecute_Request;

// Struct for a sequence of mg400_msgs__srv__ToolDOExecute_Request.
typedef struct mg400_msgs__srv__ToolDOExecute_Request__Sequence
{
  mg400_msgs__srv__ToolDOExecute_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__ToolDOExecute_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/ToolDOExecute in the package mg400_msgs.
typedef struct mg400_msgs__srv__ToolDOExecute_Response
{
  bool result;
  int32_t error_id;
} mg400_msgs__srv__ToolDOExecute_Response;

// Struct for a sequence of mg400_msgs__srv__ToolDOExecute_Response.
typedef struct mg400_msgs__srv__ToolDOExecute_Response__Sequence
{
  mg400_msgs__srv__ToolDOExecute_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__ToolDOExecute_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__TOOL_DO_EXECUTE__STRUCT_H_
