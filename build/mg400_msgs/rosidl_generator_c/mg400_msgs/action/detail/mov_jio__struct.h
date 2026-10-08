// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:action/MovJIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__MOV_JIO__STRUCT_H_
#define MG400_MSGS__ACTION__DETAIL__MOV_JIO__STRUCT_H_

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

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_Goal
{
  geometry_msgs__msg__PoseStamped pose;
  mg400_msgs__msg__DistanceMode mode;
  int32_t distance;
  mg400_msgs__msg__DOIndex index;
  mg400_msgs__msg__DOStatus status;
  bool set_speed_j;
  uint8_t speed_j;
  bool set_acc_j;
  uint8_t acc_j;
  bool set_cp;
  uint8_t cp;
} mg400_msgs__action__MovJIO_Goal;

// Struct for a sequence of mg400_msgs__action__MovJIO_Goal.
typedef struct mg400_msgs__action__MovJIO_Goal__Sequence
{
  mg400_msgs__action__MovJIO_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'error_id'
#include "mg400_msgs/msg/detail/error_id__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_Result
{
  bool result;
  mg400_msgs__msg__ErrorID error_id;
} mg400_msgs__action__MovJIO_Result;

// Struct for a sequence of mg400_msgs__action__MovJIO_Result.
typedef struct mg400_msgs__action__MovJIO_Result__Sequence
{
  mg400_msgs__action__MovJIO_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'current_pose'
// already included above
// #include "geometry_msgs/msg/detail/pose_stamped__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_Feedback
{
  geometry_msgs__msg__PoseStamped current_pose;
} mg400_msgs__action__MovJIO_Feedback;

// Struct for a sequence of mg400_msgs__action__MovJIO_Feedback.
typedef struct mg400_msgs__action__MovJIO_Feedback__Sequence
{
  mg400_msgs__action__MovJIO_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "mg400_msgs/action/detail/mov_jio__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  mg400_msgs__action__MovJIO_Goal goal;
} mg400_msgs__action__MovJIO_SendGoal_Request;

// Struct for a sequence of mg400_msgs__action__MovJIO_SendGoal_Request.
typedef struct mg400_msgs__action__MovJIO_SendGoal_Request__Sequence
{
  mg400_msgs__action__MovJIO_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} mg400_msgs__action__MovJIO_SendGoal_Response;

// Struct for a sequence of mg400_msgs__action__MovJIO_SendGoal_Response.
typedef struct mg400_msgs__action__MovJIO_SendGoal_Response__Sequence
{
  mg400_msgs__action__MovJIO_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} mg400_msgs__action__MovJIO_GetResult_Request;

// Struct for a sequence of mg400_msgs__action__MovJIO_GetResult_Request.
typedef struct mg400_msgs__action__MovJIO_GetResult_Request__Sequence
{
  mg400_msgs__action__MovJIO_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "mg400_msgs/action/detail/mov_jio__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_GetResult_Response
{
  int8_t status;
  mg400_msgs__action__MovJIO_Result result;
} mg400_msgs__action__MovJIO_GetResult_Response;

// Struct for a sequence of mg400_msgs__action__MovJIO_GetResult_Response.
typedef struct mg400_msgs__action__MovJIO_GetResult_Response__Sequence
{
  mg400_msgs__action__MovJIO_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "mg400_msgs/action/detail/mov_jio__struct.h"

/// Struct defined in action/MovJIO in the package mg400_msgs.
typedef struct mg400_msgs__action__MovJIO_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  mg400_msgs__action__MovJIO_Feedback feedback;
} mg400_msgs__action__MovJIO_FeedbackMessage;

// Struct for a sequence of mg400_msgs__action__MovJIO_FeedbackMessage.
typedef struct mg400_msgs__action__MovJIO_FeedbackMessage__Sequence
{
  mg400_msgs__action__MovJIO_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__MovJIO_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__ACTION__DETAIL__MOV_JIO__STRUCT_H_
