// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/ToolDIIndex.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__TOOL_DI_INDEX__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__TOOL_DI_INDEX__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'D1'.
enum
{
  mg400_msgs__msg__ToolDIIndex__D1 = 1ul
};

/// Constant 'D2'.
enum
{
  mg400_msgs__msg__ToolDIIndex__D2 = 2ul
};

/// Struct defined in msg/ToolDIIndex in the package mg400_msgs.
typedef struct mg400_msgs__msg__ToolDIIndex
{
  uint32_t index;
} mg400_msgs__msg__ToolDIIndex;

// Struct for a sequence of mg400_msgs__msg__ToolDIIndex.
typedef struct mg400_msgs__msg__ToolDIIndex__Sequence
{
  mg400_msgs__msg__ToolDIIndex * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__ToolDIIndex__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__TOOL_DI_INDEX__STRUCT_H_
