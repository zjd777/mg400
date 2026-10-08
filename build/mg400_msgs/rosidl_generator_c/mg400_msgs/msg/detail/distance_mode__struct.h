// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/DistanceMode.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DISTANCE_MODE__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__DISTANCE_MODE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'PERCENTAGE'.
enum
{
  mg400_msgs__msg__DistanceMode__PERCENTAGE = 0
};

/// Constant 'FROM_START_OR_TARGET'.
enum
{
  mg400_msgs__msg__DistanceMode__FROM_START_OR_TARGET = 1
};

/// Struct defined in msg/DistanceMode in the package mg400_msgs.
typedef struct mg400_msgs__msg__DistanceMode
{
  uint8_t mode;
} mg400_msgs__msg__DistanceMode;

// Struct for a sequence of mg400_msgs__msg__DistanceMode.
typedef struct mg400_msgs__msg__DistanceMode__Sequence
{
  mg400_msgs__msg__DistanceMode * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__DistanceMode__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__DISTANCE_MODE__STRUCT_H_
