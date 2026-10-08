// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from mg400_msgs:srv/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__FUNCTIONS_H_
#define MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "mg400_msgs/msg/rosidl_generator_c__visibility_control.h"

#include "mg400_msgs/srv/detail/joint_mov_j__struct.h"

/// Initialize srv/JointMovJ message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__srv__JointMovJ_Request
 * )) before or use
 * mg400_msgs__srv__JointMovJ_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Request__init(mg400_msgs__srv__JointMovJ_Request * msg);

/// Finalize srv/JointMovJ message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Request__fini(mg400_msgs__srv__JointMovJ_Request * msg);

/// Create srv/JointMovJ message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__srv__JointMovJ_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__srv__JointMovJ_Request *
mg400_msgs__srv__JointMovJ_Request__create();

/// Destroy srv/JointMovJ message.
/**
 * It calls
 * mg400_msgs__srv__JointMovJ_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Request__destroy(mg400_msgs__srv__JointMovJ_Request * msg);

/// Check for srv/JointMovJ message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Request__are_equal(const mg400_msgs__srv__JointMovJ_Request * lhs, const mg400_msgs__srv__JointMovJ_Request * rhs);

/// Copy a srv/JointMovJ message.
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
mg400_msgs__srv__JointMovJ_Request__copy(
  const mg400_msgs__srv__JointMovJ_Request * input,
  mg400_msgs__srv__JointMovJ_Request * output);

/// Initialize array of srv/JointMovJ messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__srv__JointMovJ_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Request__Sequence__init(mg400_msgs__srv__JointMovJ_Request__Sequence * array, size_t size);

/// Finalize array of srv/JointMovJ messages.
/**
 * It calls
 * mg400_msgs__srv__JointMovJ_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Request__Sequence__fini(mg400_msgs__srv__JointMovJ_Request__Sequence * array);

/// Create array of srv/JointMovJ messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__srv__JointMovJ_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__srv__JointMovJ_Request__Sequence *
mg400_msgs__srv__JointMovJ_Request__Sequence__create(size_t size);

/// Destroy array of srv/JointMovJ messages.
/**
 * It calls
 * mg400_msgs__srv__JointMovJ_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Request__Sequence__destroy(mg400_msgs__srv__JointMovJ_Request__Sequence * array);

/// Check for srv/JointMovJ message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Request__Sequence__are_equal(const mg400_msgs__srv__JointMovJ_Request__Sequence * lhs, const mg400_msgs__srv__JointMovJ_Request__Sequence * rhs);

/// Copy an array of srv/JointMovJ messages.
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
mg400_msgs__srv__JointMovJ_Request__Sequence__copy(
  const mg400_msgs__srv__JointMovJ_Request__Sequence * input,
  mg400_msgs__srv__JointMovJ_Request__Sequence * output);

/// Initialize srv/JointMovJ message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * mg400_msgs__srv__JointMovJ_Response
 * )) before or use
 * mg400_msgs__srv__JointMovJ_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Response__init(mg400_msgs__srv__JointMovJ_Response * msg);

/// Finalize srv/JointMovJ message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Response__fini(mg400_msgs__srv__JointMovJ_Response * msg);

/// Create srv/JointMovJ message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * mg400_msgs__srv__JointMovJ_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__srv__JointMovJ_Response *
mg400_msgs__srv__JointMovJ_Response__create();

/// Destroy srv/JointMovJ message.
/**
 * It calls
 * mg400_msgs__srv__JointMovJ_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Response__destroy(mg400_msgs__srv__JointMovJ_Response * msg);

/// Check for srv/JointMovJ message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Response__are_equal(const mg400_msgs__srv__JointMovJ_Response * lhs, const mg400_msgs__srv__JointMovJ_Response * rhs);

/// Copy a srv/JointMovJ message.
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
mg400_msgs__srv__JointMovJ_Response__copy(
  const mg400_msgs__srv__JointMovJ_Response * input,
  mg400_msgs__srv__JointMovJ_Response * output);

/// Initialize array of srv/JointMovJ messages.
/**
 * It allocates the memory for the number of elements and calls
 * mg400_msgs__srv__JointMovJ_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Response__Sequence__init(mg400_msgs__srv__JointMovJ_Response__Sequence * array, size_t size);

/// Finalize array of srv/JointMovJ messages.
/**
 * It calls
 * mg400_msgs__srv__JointMovJ_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Response__Sequence__fini(mg400_msgs__srv__JointMovJ_Response__Sequence * array);

/// Create array of srv/JointMovJ messages.
/**
 * It allocates the memory for the array and calls
 * mg400_msgs__srv__JointMovJ_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
mg400_msgs__srv__JointMovJ_Response__Sequence *
mg400_msgs__srv__JointMovJ_Response__Sequence__create(size_t size);

/// Destroy array of srv/JointMovJ messages.
/**
 * It calls
 * mg400_msgs__srv__JointMovJ_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
void
mg400_msgs__srv__JointMovJ_Response__Sequence__destroy(mg400_msgs__srv__JointMovJ_Response__Sequence * array);

/// Check for srv/JointMovJ message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_mg400_msgs
bool
mg400_msgs__srv__JointMovJ_Response__Sequence__are_equal(const mg400_msgs__srv__JointMovJ_Response__Sequence * lhs, const mg400_msgs__srv__JointMovJ_Response__Sequence * rhs);

/// Copy an array of srv/JointMovJ messages.
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
mg400_msgs__srv__JointMovJ_Response__Sequence__copy(
  const mg400_msgs__srv__JointMovJ_Response__Sequence * input,
  mg400_msgs__srv__JointMovJ_Response__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // MG400_MSGS__SRV__DETAIL__JOINT_MOV_J__FUNCTIONS_H_
