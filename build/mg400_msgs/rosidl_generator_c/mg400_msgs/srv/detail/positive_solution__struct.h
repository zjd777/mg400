// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/PositiveSolution.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/PositiveSolution in the package mg400_msgs.
typedef struct mg400_msgs__srv__PositiveSolution_Request
{
  double j1;
  double j2;
  double j3;
  double j4;
  uint8_t user;
  uint8_t tool;
} mg400_msgs__srv__PositiveSolution_Request;

// Struct for a sequence of mg400_msgs__srv__PositiveSolution_Request.
typedef struct mg400_msgs__srv__PositiveSolution_Request__Sequence
{
  mg400_msgs__srv__PositiveSolution_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__PositiveSolution_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/PositiveSolution in the package mg400_msgs.
typedef struct mg400_msgs__srv__PositiveSolution_Response
{
  int32_t error_id;
  double x;
  double y;
  double z;
  double r;
} mg400_msgs__srv__PositiveSolution_Response;

// Struct for a sequence of mg400_msgs__srv__PositiveSolution_Response.
typedef struct mg400_msgs__srv__PositiveSolution_Response__Sequence
{
  mg400_msgs__srv__PositiveSolution_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__PositiveSolution_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__POSITIVE_SOLUTION__STRUCT_H_
