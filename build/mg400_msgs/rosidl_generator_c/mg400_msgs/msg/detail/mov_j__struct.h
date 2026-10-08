// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/MovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_J__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__MOV_J__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose_stamped__struct.h"

/// Struct defined in msg/MovJ in the package mg400_msgs.
typedef struct mg400_msgs__msg__MovJ
{
  geometry_msgs__msg__PoseStamped pose;
  bool set_speed_j;
  uint8_t speed_j;
  bool set_acc_j;
  uint8_t acc_j;
  bool set_cp;
  uint8_t cp;
} mg400_msgs__msg__MovJ;

// Struct for a sequence of mg400_msgs__msg__MovJ.
typedef struct mg400_msgs__msg__MovJ__Sequence
{
  mg400_msgs__msg__MovJ * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__MovJ__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__MOV_J__STRUCT_H_
