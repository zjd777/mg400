// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/IDArray.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ID_ARRAY__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__ID_ARRAY__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'ids'
#include "rosidl_runtime_c/primitives_sequence.h"

/// Struct defined in msg/IDArray in the package mg400_msgs.
typedef struct mg400_msgs__msg__IDArray
{
  rosidl_runtime_c__int32__Sequence ids;
} mg400_msgs__msg__IDArray;

// Struct for a sequence of mg400_msgs__msg__IDArray.
typedef struct mg400_msgs__msg__IDArray__Sequence
{
  mg400_msgs__msg__IDArray * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__IDArray__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__ID_ARRAY__STRUCT_H_
