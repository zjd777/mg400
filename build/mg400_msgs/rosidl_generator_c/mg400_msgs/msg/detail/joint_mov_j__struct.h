// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/JointMovJ in the package mg400_msgs.
typedef struct mg400_msgs__msg__JointMovJ
{
  /// Joint angles in radian
  double joint_angles[4];
  bool set_speed_j;
  uint8_t speed_j;
  bool set_acc_j;
  uint8_t acc_j;
  bool set_cp;
  uint8_t cp;
} mg400_msgs__msg__JointMovJ;

// Struct for a sequence of mg400_msgs__msg__JointMovJ.
typedef struct mg400_msgs__msg__JointMovJ__Sequence
{
  mg400_msgs__msg__JointMovJ * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__JointMovJ__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__STRUCT_H_
