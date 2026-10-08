// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/DOStatus.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DO_STATUS__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__DO_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'LOW'.
enum
{
  mg400_msgs__msg__DOStatus__LOW = 0
};

/// Constant 'HIGH'.
enum
{
  mg400_msgs__msg__DOStatus__HIGH = 1
};

/// Struct defined in msg/DOStatus in the package mg400_msgs.
typedef struct mg400_msgs__msg__DOStatus
{
  uint8_t status;
} mg400_msgs__msg__DOStatus;

// Struct for a sequence of mg400_msgs__msg__DOStatus.
typedef struct mg400_msgs__msg__DOStatus__Sequence
{
  mg400_msgs__msg__DOStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__DOStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__DO_STATUS__STRUCT_H_
