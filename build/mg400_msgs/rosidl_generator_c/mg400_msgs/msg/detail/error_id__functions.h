// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ERROR_ID__FUNCTIONS_H_
#define MG400_MSGS__MSG__DETAIL__ERROR_ID__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "mg400_msgs/msg/rosidl_generator_c__visibility_control.h"

#include "mg400_msgs/msg/detail/error_id__struct.h"

/// Initialize msg/ErrorID message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__msg__ErrorID
 * )) before or use
 * mg400_msgs__msg__ErrorID__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ErrorID__init(mg400_msgs__msg__ErrorID * msg);

/// Finalize msg/ErrorID message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ErrorID__fini(mg400_msgs__msg__ErrorID * msg);

/// Create msg/ErrorID message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__msg__ErrorID__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__msg__ErrorID *
mg400_msgs__msg__ErrorID__create();

/// Destroy msg/ErrorID message.
/**
 * It calls
 * mg400_msgs__msg__ErrorID__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ErrorID__destroy(mg400_msgs__msg__ErrorID * msg);

/// Check for msg/ErrorID message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ErrorID__are_equal(const mg400_msgs__msg__ErrorID * lhs, const mg400_msgs__msg__ErrorID * rhs);

/// Copy a msg/ErrorID message.
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
mg400_msgs__msg__ErrorID__copy(
  const mg400_msgs__msg__ErrorID * input,
  mg400_msgs__msg__ErrorID * output);

/// Initialize array of msg/ErrorID messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__msg__ErrorID__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ErrorID__Sequence__init(mg400_msgs__msg__ErrorID__Sequence * array, size_t size);

/// Finalize array of msg/ErrorID messages.
/**
 * It calls
 * mg400_msgs__msg__ErrorID__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ErrorID__Sequence__fini(mg400_msgs__msg__ErrorID__Sequence * array);

/// Create array of msg/ErrorID messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__msg__ErrorID__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__msg__ErrorID__Sequence *
mg400_msgs__msg__ErrorID__Sequence__create(size_t size);

/// Destroy array of msg/ErrorID messages.
/**
 * It calls
 * mg400_msgs__msg__ErrorID__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__msg__ErrorID__Sequence__destroy(mg400_msgs__msg__ErrorID__Sequence * array);

/// Check for msg/ErrorID message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__msg__ErrorID__Sequence__are_equal(const mg400_msgs__msg__ErrorID__Sequence * lhs, const mg400_msgs__msg__ErrorID__Sequence * rhs);

/// Copy an array of msg/ErrorID messages.
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
mg400_msgs__msg__ErrorID__Sequence__copy(
  const mg400_msgs__msg__ErrorID__Sequence * input,
  mg400_msgs__msg__ErrorID__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__MSG__DETAIL__ERROR_ID__FUNCTIONS_H_
