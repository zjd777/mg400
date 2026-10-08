// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/RobotMode.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ROBOT_MODE__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__ROBOT_MODE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'INIT'.
enum
{
  mg400_msgs__msg__RobotMode__INIT = 1ull
};

/// Constant 'BRAKE_OPEN'.
enum
{
  mg400_msgs__msg__RobotMode__BRAKE_OPEN = 2ull
};

/// Constant 'DISABLED'.
enum
{
  mg400_msgs__msg__RobotMode__DISABLED = 4ull
};

/// Constant 'ENABLE'.
enum
{
  mg400_msgs__msg__RobotMode__ENABLE = 5ull
};

/// Constant 'BACKDRIVE'.
enum
{
  mg400_msgs__msg__RobotMode__BACKDRIVE = 6ull
};

/// Constant 'RUNNING'.
enum
{
  mg400_msgs__msg__RobotMode__RUNNING = 7ull
};

/// Constant 'RECORDING'.
enum
{
  mg400_msgs__msg__RobotMode__RECORDING = 8ull
};

/// Constant 'ERROR'.
enum
{
  mg400_msgs__msg__RobotMode__ERROR = 9ull
};

/// Constant 'PAUSE'.
enum
{
  mg400_msgs__msg__RobotMode__PAUSE = 10ull
};

/// Constant 'JOG'.
enum
{
  mg400_msgs__msg__RobotMode__JOG = 11ull
};

/// Constant 'INVALID'.
enum
{
  mg400_msgs__msg__RobotMode__INVALID = 12ull
};

/// Struct defined in msg/RobotMode in the package mg400_msgs.
typedef struct mg400_msgs__msg__RobotMode
{
  uint64_t robot_mode;
} mg400_msgs__msg__RobotMode;

// Struct for a sequence of mg400_msgs__msg__RobotMode.
typedef struct mg400_msgs__msg__RobotMode__Sequence
{
  mg400_msgs__msg__RobotMode * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__RobotMode__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__ROBOT_MODE__STRUCT_H_
