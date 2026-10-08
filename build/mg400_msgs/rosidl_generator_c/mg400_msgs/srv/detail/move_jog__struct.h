// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/MoveJog.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__MOVE_JOG__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__MOVE_JOG__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'jog'
#include "mg400_msgs/msg/detail/move_jog__struct.h"

/// Struct defined in srv/MoveJog in the package mg400_msgs.
typedef struct mg400_msgs__srv__MoveJog_Request
{
  mg400_msgs__msg__MoveJog jog;
} mg400_msgs__srv__MoveJog_Request;

// Struct for a sequence of mg400_msgs__srv__MoveJog_Request.
typedef struct mg400_msgs__srv__MoveJog_Request__Sequence
{
  mg400_msgs__srv__MoveJog_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__MoveJog_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/MoveJog in the package mg400_msgs.
typedef struct mg400_msgs__srv__MoveJog_Response
{
  int32_t error_id;
} mg400_msgs__srv__MoveJog_Response;

// Struct for a sequence of mg400_msgs__srv__MoveJog_Response.
typedef struct mg400_msgs__srv__MoveJog_Response__Sequence
{
  mg400_msgs__srv__MoveJog_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__MoveJog_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__MOVE_JOG__STRUCT_H_
