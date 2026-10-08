// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from mg400_msgs:action/CommandQueue.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__STRUCT_H_
#define MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'commands'
#include "mg400_msgs/msg/detail/command__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_Goal
{
  mg400_msgs__msg__Command__Sequence commands;
} mg400_msgs__action__CommandQueue_Goal;

// Struct for a sequence of mg400_msgs__action__CommandQueue_Goal.
typedef struct mg400_msgs__action__CommandQueue_Goal__Sequence
{
  mg400_msgs__action__CommandQueue_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'error_id'
#include "mg400_msgs/msg/detail/error_id__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_Result
{
  bool result;
  mg400_msgs__msg__ErrorID error_id;
} mg400_msgs__action__CommandQueue_Result;

// Struct for a sequence of mg400_msgs__action__CommandQueue_Result.
typedef struct mg400_msgs__action__CommandQueue_Result__Sequence
{
  mg400_msgs__action__CommandQueue_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'current_pose'
#include "geometry_msgs/msg/detail/pose_stamped__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_Feedback
{
  geometry_msgs__msg__PoseStamped current_pose;
  /// Current joint angles in radian
  double current_angles[4];
} mg400_msgs__action__CommandQueue_Feedback;

// Struct for a sequence of mg400_msgs__action__CommandQueue_Feedback.
typedef struct mg400_msgs__action__CommandQueue_Feedback__Sequence
{
  mg400_msgs__action__CommandQueue_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "mg400_msgs/action/detail/command_queue__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  mg400_msgs__action__CommandQueue_Goal goal;
} mg400_msgs__action__CommandQueue_SendGoal_Request;

// Struct for a sequence of mg400_msgs__action__CommandQueue_SendGoal_Request.
typedef struct mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence
{
  mg400_msgs__action__CommandQueue_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} mg400_msgs__action__CommandQueue_SendGoal_Response;

// Struct for a sequence of mg400_msgs__action__CommandQueue_SendGoal_Response.
typedef struct mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence
{
  mg400_msgs__action__CommandQueue_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} mg400_msgs__action__CommandQueue_GetResult_Request;

// Struct for a sequence of mg400_msgs__action__CommandQueue_GetResult_Request.
typedef struct mg400_msgs__action__CommandQueue_GetResult_Request__Sequence
{
  mg400_msgs__action__CommandQueue_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_GetResult_Response
{
  int8_t status;
  mg400_msgs__action__CommandQueue_Result result;
} mg400_msgs__action__CommandQueue_GetResult_Response;

// Struct for a sequence of mg400_msgs__action__CommandQueue_GetResult_Response.
typedef struct mg400_msgs__action__CommandQueue_GetResult_Response__Sequence
{
  mg400_msgs__action__CommandQueue_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.h"

/// Struct defined in action/CommandQueue in the package mg400_msgs.
typedef struct mg400_msgs__action__CommandQueue_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  mg400_msgs__action__CommandQueue_Feedback feedback;
} mg400_msgs__action__CommandQueue_FeedbackMessage;

// Struct for a sequence of mg400_msgs__action__CommandQueue_FeedbackMessage.
typedef struct mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence
{
  mg400_msgs__action__CommandQueue_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__STRUCT_H_
