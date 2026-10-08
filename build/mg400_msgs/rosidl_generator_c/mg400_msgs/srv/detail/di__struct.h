// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/DI.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__DI__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__DI__STRUCT_H_

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
#include "mg400_msgs/msg/detail/di_index__struct.h"

/// Struct defined in srv/DI in the package mg400_msgs.
typedef struct mg400_msgs__srv__DI_Request
{
  mg400_msgs__msg__DIIndex index;
} mg400_msgs__srv__DI_Request;

// Struct for a sequence of mg400_msgs__srv__DI_Request.
typedef struct mg400_msgs__srv__DI_Request__Sequence
{
  mg400_msgs__srv__DI_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__DI_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/DI in the package mg400_msgs.
typedef struct mg400_msgs__srv__DI_Response
{
  uint8_t result;
  int32_t error_id;
} mg400_msgs__srv__DI_Response;

// Struct for a sequence of mg400_msgs__srv__DI_Response.
typedef struct mg400_msgs__srv__DI_Response__Sequence
{
  mg400_msgs__srv__DI_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__DI_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__DI__STRUCT_H_
