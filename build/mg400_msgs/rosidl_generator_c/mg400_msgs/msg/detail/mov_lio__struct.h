// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/MovLIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_LIO__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__MOV_LIO__STRUCT_H_

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
// Member 'mode'
#include "mg400_msgs/msg/detail/distance_mode__struct.h"
// Member 'index'
#include "mg400_msgs/msg/detail/do_index__struct.h"
// Member 'status'
#include "mg400_msgs/msg/detail/do_status__struct.h"

/// Struct defined in msg/MovLIO in the package mg400_msgs.
typedef struct mg400_msgs__msg__MovLIO
{
  geometry_msgs__msg__PoseStamped pose;
  mg400_msgs__msg__DistanceMode mode;
  int32_t distance;
  mg400_msgs__msg__DOIndex index;
  mg400_msgs__msg__DOStatus status;
  bool set_speed_l;
  uint8_t speed_l;
  bool set_acc_l;
  uint8_t acc_l;
} mg400_msgs__msg__MovLIO;

// Struct for a sequence of mg400_msgs__msg__MovLIO.
typedef struct mg400_msgs__msg__MovLIO__Sequence
{
  mg400_msgs__msg__MovLIO * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__MovLIO__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__MOV_LIO__STRUCT_H_
