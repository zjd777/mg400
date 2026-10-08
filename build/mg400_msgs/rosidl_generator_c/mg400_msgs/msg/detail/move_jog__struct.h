// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/MoveJog.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOVE_JOG__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__MOVE_JOG__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'J1_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__J1_NEGATIVE = "j1-";

/// Constant 'J1_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__J1_POSITIVE = "j1+";

/// Constant 'J2_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__J2_NEGATIVE = "j2-";

/// Constant 'J2_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__J2_POSITIVE = "j2+";

/// Constant 'J3_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__J3_NEGATIVE = "j3-";

/// Constant 'J3_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__J3_POSITIVE = "j3+";

/// Constant 'J4_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__J4_NEGATIVE = "j4-";

/// Constant 'J4_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__J4_POSITIVE = "j4+";

/// Constant 'X_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__X_NEGATIVE = "X-";

/// Constant 'X_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__X_POSITIVE = "X+";

/// Constant 'Y_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__Y_NEGATIVE = "Y-";

/// Constant 'Y_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__Y_POSITIVE = "Y+";

/// Constant 'Z_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__Z_NEGATIVE = "Z-";

/// Constant 'Z_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__Z_POSITIVE = "Z+";

/// Constant 'RX_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__RX_NEGATIVE = "Rx-";

/// Constant 'RX_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__RX_POSITIVE = "Rx+";

/// Constant 'RY_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__RY_NEGATIVE = "Ry-";

/// Constant 'RY_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__RY_POSITIVE = "Ry+";

/// Constant 'RZ_NEGATIVE'.
static const char * const mg400_msgs__msg__MoveJog__RZ_NEGATIVE = "Rz-";

/// Constant 'RZ_POSITIVE'.
static const char * const mg400_msgs__msg__MoveJog__RZ_POSITIVE = "Rz+";

/// Constant 'STOP'.
static const char * const mg400_msgs__msg__MoveJog__STOP = "";

// Include directives for member types
// Member 'jog_mode'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/MoveJog in the package mg400_msgs.
typedef struct mg400_msgs__msg__MoveJog
{
  rosidl_runtime_c__String jog_mode;
} mg400_msgs__msg__MoveJog;

// Struct for a sequence of mg400_msgs__msg__MoveJog.
typedef struct mg400_msgs__msg__MoveJog__Sequence
{
  mg400_msgs__msg__MoveJog * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__MoveJog__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__MOVE_JOG__STRUCT_H_
