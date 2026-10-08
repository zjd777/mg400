// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from mg400_msgs:action/MovLIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__MOV_LIO__FUNCTIONS_H_
#define MG400_MSGS__ACTION__DETAIL__MOV_LIO__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "mg400_msgs/msg/rosidl_generator_c__visibility_control.h"

#include "mg400_msgs/action/detail/mov_lio__struct.h"

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_Goal
 * )) before or use
 * mg400_msgs__action__MovLIO_Goal__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Goal__init(mg400_msgs__action__MovLIO_Goal * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Goal__fini(mg400_msgs__action__MovLIO_Goal * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_Goal__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_Goal *
mg400_msgs__action__MovLIO_Goal__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Goal__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Goal__destroy(mg400_msgs__action__MovLIO_Goal * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Goal__are_equal(const mg400_msgs__action__MovLIO_Goal * lhs, const mg400_msgs__action__MovLIO_Goal * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_Goal__copy(
  const mg400_msgs__action__MovLIO_Goal * input,
  mg400_msgs__action__MovLIO_Goal * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_Goal__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Goal__Sequence__init(mg400_msgs__action__MovLIO_Goal__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Goal__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Goal__Sequence__fini(mg400_msgs__action__MovLIO_Goal__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_Goal__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_Goal__Sequence *
mg400_msgs__action__MovLIO_Goal__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Goal__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Goal__Sequence__destroy(mg400_msgs__action__MovLIO_Goal__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Goal__Sequence__are_equal(const mg400_msgs__action__MovLIO_Goal__Sequence * lhs, const mg400_msgs__action__MovLIO_Goal__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_Goal__Sequence__copy(
  const mg400_msgs__action__MovLIO_Goal__Sequence * input,
  mg400_msgs__action__MovLIO_Goal__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_Result
 * )) before or use
 * mg400_msgs__action__MovLIO_Result__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Result__init(mg400_msgs__action__MovLIO_Result * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Result__fini(mg400_msgs__action__MovLIO_Result * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_Result__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_Result *
mg400_msgs__action__MovLIO_Result__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Result__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Result__destroy(mg400_msgs__action__MovLIO_Result * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Result__are_equal(const mg400_msgs__action__MovLIO_Result * lhs, const mg400_msgs__action__MovLIO_Result * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_Result__copy(
  const mg400_msgs__action__MovLIO_Result * input,
  mg400_msgs__action__MovLIO_Result * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_Result__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Result__Sequence__init(mg400_msgs__action__MovLIO_Result__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Result__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Result__Sequence__fini(mg400_msgs__action__MovLIO_Result__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_Result__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_Result__Sequence *
mg400_msgs__action__MovLIO_Result__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Result__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Result__Sequence__destroy(mg400_msgs__action__MovLIO_Result__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Result__Sequence__are_equal(const mg400_msgs__action__MovLIO_Result__Sequence * lhs, const mg400_msgs__action__MovLIO_Result__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_Result__Sequence__copy(
  const mg400_msgs__action__MovLIO_Result__Sequence * input,
  mg400_msgs__action__MovLIO_Result__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_Feedback
 * )) before or use
 * mg400_msgs__action__MovLIO_Feedback__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Feedback__init(mg400_msgs__action__MovLIO_Feedback * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Feedback__fini(mg400_msgs__action__MovLIO_Feedback * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_Feedback__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_Feedback *
mg400_msgs__action__MovLIO_Feedback__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Feedback__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Feedback__destroy(mg400_msgs__action__MovLIO_Feedback * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Feedback__are_equal(const mg400_msgs__action__MovLIO_Feedback * lhs, const mg400_msgs__action__MovLIO_Feedback * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_Feedback__copy(
  const mg400_msgs__action__MovLIO_Feedback * input,
  mg400_msgs__action__MovLIO_Feedback * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_Feedback__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Feedback__Sequence__init(mg400_msgs__action__MovLIO_Feedback__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Feedback__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Feedback__Sequence__fini(mg400_msgs__action__MovLIO_Feedback__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_Feedback__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_Feedback__Sequence *
mg400_msgs__action__MovLIO_Feedback__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_Feedback__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_Feedback__Sequence__destroy(mg400_msgs__action__MovLIO_Feedback__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_Feedback__Sequence__are_equal(const mg400_msgs__action__MovLIO_Feedback__Sequence * lhs, const mg400_msgs__action__MovLIO_Feedback__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_Feedback__Sequence__copy(
  const mg400_msgs__action__MovLIO_Feedback__Sequence * input,
  mg400_msgs__action__MovLIO_Feedback__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_SendGoal_Request
 * )) before or use
 * mg400_msgs__action__MovLIO_SendGoal_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Request__init(mg400_msgs__action__MovLIO_SendGoal_Request * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Request__fini(mg400_msgs__action__MovLIO_SendGoal_Request * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_SendGoal_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_SendGoal_Request *
mg400_msgs__action__MovLIO_SendGoal_Request__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_SendGoal_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Request__destroy(mg400_msgs__action__MovLIO_SendGoal_Request * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Request__are_equal(const mg400_msgs__action__MovLIO_SendGoal_Request * lhs, const mg400_msgs__action__MovLIO_SendGoal_Request * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_SendGoal_Request__copy(
  const mg400_msgs__action__MovLIO_SendGoal_Request * input,
  mg400_msgs__action__MovLIO_SendGoal_Request * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_SendGoal_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__init(mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_SendGoal_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__fini(mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence *
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__destroy(mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__are_equal(const mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * lhs, const mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__copy(
  const mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * input,
  mg400_msgs__action__MovLIO_SendGoal_Request__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_SendGoal_Response
 * )) before or use
 * mg400_msgs__action__MovLIO_SendGoal_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Response__init(mg400_msgs__action__MovLIO_SendGoal_Response * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Response__fini(mg400_msgs__action__MovLIO_SendGoal_Response * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_SendGoal_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_SendGoal_Response *
mg400_msgs__action__MovLIO_SendGoal_Response__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_SendGoal_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Response__destroy(mg400_msgs__action__MovLIO_SendGoal_Response * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Response__are_equal(const mg400_msgs__action__MovLIO_SendGoal_Response * lhs, const mg400_msgs__action__MovLIO_SendGoal_Response * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_SendGoal_Response__copy(
  const mg400_msgs__action__MovLIO_SendGoal_Response * input,
  mg400_msgs__action__MovLIO_SendGoal_Response * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_SendGoal_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__init(mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_SendGoal_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__fini(mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence *
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__destroy(mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__are_equal(const mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * lhs, const mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__copy(
  const mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * input,
  mg400_msgs__action__MovLIO_SendGoal_Response__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_GetResult_Request
 * )) before or use
 * mg400_msgs__action__MovLIO_GetResult_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Request__init(mg400_msgs__action__MovLIO_GetResult_Request * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Request__fini(mg400_msgs__action__MovLIO_GetResult_Request * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_GetResult_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_GetResult_Request *
mg400_msgs__action__MovLIO_GetResult_Request__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_GetResult_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Request__destroy(mg400_msgs__action__MovLIO_GetResult_Request * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Request__are_equal(const mg400_msgs__action__MovLIO_GetResult_Request * lhs, const mg400_msgs__action__MovLIO_GetResult_Request * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_GetResult_Request__copy(
  const mg400_msgs__action__MovLIO_GetResult_Request * input,
  mg400_msgs__action__MovLIO_GetResult_Request * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_GetResult_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Request__Sequence__init(mg400_msgs__action__MovLIO_GetResult_Request__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_GetResult_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Request__Sequence__fini(mg400_msgs__action__MovLIO_GetResult_Request__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_GetResult_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_GetResult_Request__Sequence *
mg400_msgs__action__MovLIO_GetResult_Request__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_GetResult_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Request__Sequence__destroy(mg400_msgs__action__MovLIO_GetResult_Request__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Request__Sequence__are_equal(const mg400_msgs__action__MovLIO_GetResult_Request__Sequence * lhs, const mg400_msgs__action__MovLIO_GetResult_Request__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_GetResult_Request__Sequence__copy(
  const mg400_msgs__action__MovLIO_GetResult_Request__Sequence * input,
  mg400_msgs__action__MovLIO_GetResult_Request__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_GetResult_Response
 * )) before or use
 * mg400_msgs__action__MovLIO_GetResult_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Response__init(mg400_msgs__action__MovLIO_GetResult_Response * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Response__fini(mg400_msgs__action__MovLIO_GetResult_Response * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_GetResult_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_GetResult_Response *
mg400_msgs__action__MovLIO_GetResult_Response__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_GetResult_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Response__destroy(mg400_msgs__action__MovLIO_GetResult_Response * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Response__are_equal(const mg400_msgs__action__MovLIO_GetResult_Response * lhs, const mg400_msgs__action__MovLIO_GetResult_Response * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_GetResult_Response__copy(
  const mg400_msgs__action__MovLIO_GetResult_Response * input,
  mg400_msgs__action__MovLIO_GetResult_Response * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_GetResult_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Response__Sequence__init(mg400_msgs__action__MovLIO_GetResult_Response__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_GetResult_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Response__Sequence__fini(mg400_msgs__action__MovLIO_GetResult_Response__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_GetResult_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_GetResult_Response__Sequence *
mg400_msgs__action__MovLIO_GetResult_Response__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_GetResult_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_GetResult_Response__Sequence__destroy(mg400_msgs__action__MovLIO_GetResult_Response__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_GetResult_Response__Sequence__are_equal(const mg400_msgs__action__MovLIO_GetResult_Response__Sequence * lhs, const mg400_msgs__action__MovLIO_GetResult_Response__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_GetResult_Response__Sequence__copy(
  const mg400_msgs__action__MovLIO_GetResult_Response__Sequence * input,
  mg400_msgs__action__MovLIO_GetResult_Response__Sequence * output);

/// Initialize action/MovLIO message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__action__MovLIO_FeedbackMessage
 * )) before or use
 * mg400_msgs__action__MovLIO_FeedbackMessage__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_FeedbackMessage__init(mg400_msgs__action__MovLIO_FeedbackMessage * msg);

/// Finalize action/MovLIO message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_FeedbackMessage__fini(mg400_msgs__action__MovLIO_FeedbackMessage * msg);

/// Create action/MovLIO message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__action__MovLIO_FeedbackMessage__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_FeedbackMessage *
mg400_msgs__action__MovLIO_FeedbackMessage__create();

/// Destroy action/MovLIO message.
/**
 * It calls
 * mg400_msgs__action__MovLIO_FeedbackMessage__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_FeedbackMessage__destroy(mg400_msgs__action__MovLIO_FeedbackMessage * msg);

/// Check for action/MovLIO message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_FeedbackMessage__are_equal(const mg400_msgs__action__MovLIO_FeedbackMessage * lhs, const mg400_msgs__action__MovLIO_FeedbackMessage * rhs);

/// Copy a action/MovLIO message.
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
mg400_msgs__action__MovLIO_FeedbackMessage__copy(
  const mg400_msgs__action__MovLIO_FeedbackMessage * input,
  mg400_msgs__action__MovLIO_FeedbackMessage * output);

/// Initialize array of action/MovLIO messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__action__MovLIO_FeedbackMessage__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__init(mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * array, size_t size);

/// Finalize array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_FeedbackMessage__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__fini(mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * array);

/// Create array of action/MovLIO messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence *
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__create(size_t size);

/// Destroy array of action/MovLIO messages.
/**
 * It calls
 * mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__destroy(mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * array);

/// Check for action/MovLIO message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__are_equal(const mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * lhs, const mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * rhs);

/// Copy an array of action/MovLIO messages.
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
mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__copy(
  const mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * input,
  mg400_msgs__action__MovLIO_FeedbackMessage__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__ACTION__DETAIL__MOV_LIO__FUNCTIONS_H_
