// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/CollisionLevel.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'OFF'.
enum
{
  mg400_msgs__msg__CollisionLevel__OFF = 0
};

/// Constant 'LEVEL1'.
enum
{
  mg400_msgs__msg__CollisionLevel__LEVEL1 = 1
};

/// Constant 'LEVEL2'.
enum
{
  mg400_msgs__msg__CollisionLevel__LEVEL2 = 2
};

/// Constant 'LEVEL3'.
enum
{
  mg400_msgs__msg__CollisionLevel__LEVEL3 = 3
};

/// Constant 'LEVEL4'.
enum
{
  mg400_msgs__msg__CollisionLevel__LEVEL4 = 4
};

/// Constant 'LEVEL5'.
enum
{
  mg400_msgs__msg__CollisionLevel__LEVEL5 = 5
};

/// Struct defined in msg/CollisionLevel in the package mg400_msgs.
typedef struct mg400_msgs__msg__CollisionLevel
{
  uint8_t level;
} mg400_msgs__msg__CollisionLevel;

// Struct for a sequence of mg400_msgs__msg__CollisionLevel.
typedef struct mg400_msgs__msg__CollisionLevel__Sequence
{
  mg400_msgs__msg__CollisionLevel * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__CollisionLevel__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__STRUCT_H_
