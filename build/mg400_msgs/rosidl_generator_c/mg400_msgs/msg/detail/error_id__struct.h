// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ERROR_ID__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__ERROR_ID__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'controller'
// Member 'servo'
#include "mg400_msgs/msg/detail/id_array__struct.h"

/// Struct defined in msg/ErrorID in the package mg400_msgs.
typedef struct mg400_msgs__msg__ErrorID
{
  mg400_msgs__msg__IDArray controller;
  mg400_msgs__msg__IDArray servo[5];
} mg400_msgs__msg__ErrorID;

// Struct for a sequence of mg400_msgs__msg__ErrorID.
typedef struct mg400_msgs__msg__ErrorID__Sequence
{
  mg400_msgs__msg__ErrorID * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__ErrorID__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__ERROR_ID__STRUCT_H_
