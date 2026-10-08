// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COMMAND__STRUCT_H_
#define MG400_MSGS__MSG__DETAIL__COMMAND__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'CT_MOV_J'.
enum
{
  mg400_msgs__msg__Command__CT_MOV_J = 1
};

/// Constant 'CT_MOV_L'.
enum
{
  mg400_msgs__msg__Command__CT_MOV_L = 2
};

/// Constant 'CT_JOINT_MOV_J'.
enum
{
  mg400_msgs__msg__Command__CT_JOINT_MOV_J = 3
};

/// Constant 'CT_MOV_JIO'.
enum
{
  mg400_msgs__msg__Command__CT_MOV_JIO = 4
};

/// Constant 'CT_MOV_LIO'.
enum
{
  mg400_msgs__msg__Command__CT_MOV_LIO = 5
};

/// Constant 'CT_TOOL_DO'.
/**
  * uint16 CT_ARC = 6
  * uint16 CT_CIRCLE = 7
  * uint16 CT_MOVE_JOG = 8
  * uint16 CT_SYNC = 9
  * uint16 CT_REL_MOV_J_USER = 10
  * uint16 CT_REL_MOV_L_USER = 11
  * uint16 CT_REL_JOINT_MOV_J = 12
  * uint16 CT_REL_MOV_J_EXT = 13
  * uint16 CT_SYNC_ALL = 14
 */
enum
{
  mg400_msgs__msg__Command__CT_TOOL_DO = 25
};

// Include directives for member types
// Member 'mov_j_params'
#include "mg400_msgs/msg/detail/mov_j__struct.h"
// Member 'mov_l_params'
#include "mg400_msgs/msg/detail/mov_l__struct.h"
// Member 'joint_mov_j_params'
#include "mg400_msgs/msg/detail/joint_mov_j__struct.h"
// Member 'mov_jio_params'
#include "mg400_msgs/msg/detail/mov_jio__struct.h"
// Member 'mov_lio_params'
#include "mg400_msgs/msg/detail/mov_lio__struct.h"

/// Struct defined in msg/Command in the package mg400_msgs.
/**
  * ===============================================================================
  *  List of command-type IDs
  *  (Commented-out commands have not been implemented.)
  * ===============================================================================
 */
typedef struct mg400_msgs__msg__Command
{
  /// ===============================================================================
  uint16_t command_type;
  mg400_msgs__msg__MovJ mov_j_params;
  mg400_msgs__msg__MovL mov_l_params;
  mg400_msgs__msg__JointMovJ joint_mov_j_params;
  mg400_msgs__msg__MovJIO mov_jio_params;
  mg400_msgs__msg__MovLIO mov_lio_params;
} mg400_msgs__msg__Command;

// Struct for a sequence of mg400_msgs__msg__Command.
typedef struct mg400_msgs__msg__Command__Sequence
{
  mg400_msgs__msg__Command * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__msg__Command__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__COMMAND__STRUCT_H_
