// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/JointMovJ in the package mg400_msgs.
typedef struct mg400_msgs__srv__JointMovJ_Request
{
  double j1;
  double j2;
  double j3;
  double j4;
  double j5;
  double j6;
} mg400_msgs__srv__JointMovJ_Request;

// Struct for a sequence of mg400_msgs__srv__JointMovJ_Request.
typedef struct mg400_msgs__srv__JointMovJ_Request__Sequence
{
  mg400_msgs__srv__JointMovJ_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__JointMovJ_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/JointMovJ in the package mg400_msgs.
typedef struct mg400_msgs__srv__JointMovJ_Response
{
  int32_t error_id;
} mg400_msgs__srv__JointMovJ_Response;

// Struct for a sequence of mg400_msgs__srv__JointMovJ_Response.
typedef struct mg400_msgs__srv__JointMovJ_Response__Sequence
{
  mg400_msgs__srv__JointMovJ_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__JointMovJ_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__STRUCT_H_
