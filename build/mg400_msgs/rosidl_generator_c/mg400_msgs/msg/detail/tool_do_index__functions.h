// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from mg400_msgs:msg/ToolDOIndex.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__TOOL_DO_INDEX__FUNCTIONS_H_
#define MG400_MSGS__MSG__DETAIL__TOOL_DO_INDEX__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "mg400_msgs/msg/rosidl_generator_c__visibility_control.h"

#include "mg400_msgs/msg/detail/tool_do_index__struct.h"

/// Initialize msg/ToolDOIndex message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__msg__ToolDOIndex
 * )) before or use
 * mg400_msgs__msg__ToolDOIndex__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ToolDOIndex__init(mg400_msgs__msg__ToolDOIndex * msg);

/// Finalize msg/ToolDOIndex message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ToolDOIndex__fini(mg400_msgs__msg__ToolDOIndex * msg);

/// Create msg/ToolDOIndex message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__msg__ToolDOIndex__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__msg__ToolDOIndex *
mg400_msgs__msg__ToolDOIndex__create();

/// Destroy msg/ToolDOIndex message.
/**
 * It calls
 * mg400_msgs__msg__ToolDOIndex__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ToolDOIndex__destroy(mg400_msgs__msg__ToolDOIndex * msg);

/// Check for msg/ToolDOIndex message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ToolDOIndex__are_equal(const mg400_msgs__msg__ToolDOIndex * lhs, const mg400_msgs__msg__ToolDOIndex * rhs);

/// Copy a msg/ToolDOIndex message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ToolDOIndex__copy(
  const mg400_msgs__msg__ToolDOIndex * input,
  mg400_msgs__msg__ToolDOIndex * output);

/// Initialize array of msg/ToolDOIndex messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__msg__ToolDOIndex__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ToolDOIndex__Sequence__init(mg400_msgs__msg__ToolDOIndex__Sequence * array, size_t size);

/// Finalize array of msg/ToolDOIndex messages.
/**
 * It calls
 * mg400_msgs__msg__ToolDOIndex__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ToolDOIndex__Sequence__fini(mg400_msgs__msg__ToolDOIndex__Sequence * array);

/// Create array of msg/ToolDOIndex messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__msg__ToolDOIndex__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__msg__ToolDOIndex__Sequence *
mg400_msgs__msg__ToolDOIndex__Sequence__create(size_t size);

/// Destroy array of msg/ToolDOIndex messages.
/**
 * It calls
 * mg400_msgs__msg__ToolDOIndex__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ToolDOIndex__Sequence__destroy(mg400_msgs__msg__ToolDOIndex__Sequence * array);

/// Check for msg/ToolDOIndex message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ToolDOIndex__Sequence__are_equal(const mg400_msgs__msg__ToolDOIndex__Sequence * lhs, const mg400_msgs__msg__ToolDOIndex__Sequence * rhs);

/// Copy an array of msg/ToolDOIndex messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ToolDOIndex__Sequence__copy(
  const mg400_msgs__msg__ToolDOIndex__Sequence * input,
  mg400_msgs__msg__ToolDOIndex__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__TOOL_DO_INDEX__FUNCTIONS_H_
