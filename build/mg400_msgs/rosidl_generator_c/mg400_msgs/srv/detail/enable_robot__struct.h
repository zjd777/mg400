// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:srv/EnableRobot.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__STRUCT_H_
#define MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'NO_PARAM'.
enum
{
  mg400_msgs__srv__EnableRobot_Request__NO_PARAM = 0
};

/// Constant 'ONE_PARAM'.
enum
{
  mg400_msgs__srv__EnableRobot_Request__ONE_PARAM = 1
};

/// Constant 'FOUR_PARAM'.
enum
{
  mg400_msgs__srv__EnableRobot_Request__FOUR_PARAM = 4
};

/// Struct defined in srv/EnableRobot in the package mg400_msgs.
typedef struct mg400_msgs__srv__EnableRobot_Request
{
  int8_t num_of_params;
  double load;
  double center_x;
  double center_y;
  double center_z;
} mg400_msgs__srv__EnableRobot_Request;

// Struct for a sequence of mg400_msgs__srv__EnableRobot_Request.
typedef struct mg400_msgs__srv__EnableRobot_Request__Sequence
{
  mg400_msgs__srv__EnableRobot_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__EnableRobot_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/EnableRobot in the package mg400_msgs.
typedef struct mg400_msgs__srv__EnableRobot_Response
{
  bool result;
  int32_t error_id;
} mg400_msgs__srv__EnableRobot_Response;

// Struct for a sequence of mg400_msgs__srv__EnableRobot_Response.
typedef struct mg400_msgs__srv__EnableRobot_Response__Sequence
{
  mg400_msgs__srv__EnableRobot_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__srv__EnableRobot_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__STRUCT_H_
