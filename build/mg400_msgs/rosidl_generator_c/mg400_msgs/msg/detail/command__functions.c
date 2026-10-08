// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/msg/detail/command__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `mov_j_params`
#include "mg400_msgs/msg/detail/mov_j__functions.h"
// Member `mov_l_params`
#include "mg400_msgs/msg/detail/mov_l__functions.h"
// Member `joint_mov_j_params`
#include "mg400_msgs/msg/detail/joint_mov_j__functions.h"
// Member `mov_jio_params`
#include "mg400_msgs/msg/detail/mov_jio__functions.h"
// Member `mov_lio_params`
#include "mg400_msgs/msg/detail/mov_lio__functions.h"

bool
mg400_msgs__msg__Command__init(mg400_msgs__msg__Command * msg)
{
  if (!msg) {
    return false;
  }
  // command_type
  // mov_j_params
  if (!mg400_msgs__msg__MovJ__init(&msg->mov_j_params)) {
    mg400_msgs__msg__Command__fini(msg);
    return false;
  }
  // mov_l_params
  if (!mg400_msgs__msg__MovL__init(&msg->mov_l_params)) {
    mg400_msgs__msg__Command__fini(msg);
    return false;
  }
  // joint_mov_j_params
  if (!mg400_msgs__msg__JointMovJ__init(&msg->joint_mov_j_params)) {
    mg400_msgs__msg__Command__fini(msg);
    return false;
  }
  // mov_jio_params
  if (!mg400_msgs__msg__MovJIO__init(&msg->mov_jio_params)) {
    mg400_msgs__msg__Command__fini(msg);
    return false;
  }
  // mov_lio_params
  if (!mg400_msgs__msg__MovLIO__init(&msg->mov_lio_params)) {
    mg400_msgs__msg__Command__fini(msg);
    return false;
  }
  return true;
}

void
mg400_msgs__msg__Command__fini(mg400_msgs__msg__Command * msg)
{
  if (!msg) {
    return;
  }
  // command_type
  // mov_j_params
  mg400_msgs__msg__MovJ__fini(&msg->mov_j_params);
  // mov_l_params
  mg400_msgs__msg__MovL__fini(&msg->mov_l_params);
  // joint_mov_j_params
  mg400_msgs__msg__JointMovJ__fini(&msg->joint_mov_j_params);
  // mov_jio_params
  mg400_msgs__msg__MovJIO__fini(&msg->mov_jio_params);
  // mov_lio_params
  mg400_msgs__msg__MovLIO__fini(&msg->mov_lio_params);
}

bool
mg400_msgs__msg__Command__are_equal(const mg400_msgs__msg__Command * lhs, const mg400_msgs__msg__Command * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // command_type
  if (lhs->command_type != rhs->command_type) {
    return false;
  }
  // mov_j_params
  if (!mg400_msgs__msg__MovJ__are_equal(
      &(lhs->mov_j_params), &(rhs->mov_j_params)))
  {
    return false;
  }
  // mov_l_params
  if (!mg400_msgs__msg__MovL__are_equal(
      &(lhs->mov_l_params), &(rhs->mov_l_params)))
  {
    return false;
  }
  // joint_mov_j_params
  if (!mg400_msgs__msg__JointMovJ__are_equal(
      &(lhs->joint_mov_j_params), &(rhs->joint_mov_j_params)))
  {
    return false;
  }
  // mov_jio_params
  if (!mg400_msgs__msg__MovJIO__are_equal(
      &(lhs->mov_jio_params), &(rhs->mov_jio_params)))
  {
    return false;
  }
  // mov_lio_params
  if (!mg400_msgs__msg__MovLIO__are_equal(
      &(lhs->mov_lio_params), &(rhs->mov_lio_params)))
  {
    return false;
  }
  return true;
}

bool
mg400_msgs__msg__Command__copy(
  const mg400_msgs__msg__Command * input,
  mg400_msgs__msg__Command * output)
{
  if (!input || !output) {
    return false;
  }
  // command_type
  output->command_type = input->command_type;
  // mov_j_params
  if (!mg400_msgs__msg__MovJ__copy(
      &(input->mov_j_params), &(output->mov_j_params)))
  {
    return false;
  }
  // mov_l_params
  if (!mg400_msgs__msg__MovL__copy(
      &(input->mov_l_params), &(output->mov_l_params)))
  {
    return false;
  }
  // joint_mov_j_params
  if (!mg400_msgs__msg__JointMovJ__copy(
      &(input->joint_mov_j_params), &(output->joint_mov_j_params)))
  {
    return false;
  }
  // mov_jio_params
  if (!mg400_msgs__msg__MovJIO__copy(
      &(input->mov_jio_params), &(output->mov_jio_params)))
  {
    return false;
  }
  // mov_lio_params
  if (!mg400_msgs__msg__MovLIO__copy(
      &(input->mov_lio_params), &(output->mov_lio_params)))
  {
    return false;
  }
  return true;
}

mg400_msgs__msg__Command *
mg400_msgs__msg__Command__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__Command * msg = (mg400_msgs__msg__Command *)allocator.allocate(sizeof(mg400_msgs__msg__Command), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(mg400_msgs__msg__Command));
  bool success = mg400_msgs__msg__Command__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
mg400_msgs__msg__Command__destroy(mg400_msgs__msg__Command * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    mg400_msgs__msg__Command__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
mg400_msgs__msg__Command__Sequence__init(mg400_msgs__msg__Command__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__Command * data = NULL;

  if (size) {
    data = (mg400_msgs__msg__Command *)allocator.zero_allocate(size, sizeof(mg400_msgs__msg__Command), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = mg400_msgs__msg__Command__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        mg400_msgs__msg__Command__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
mg400_msgs__msg__Command__Sequence__fini(mg400_msgs__msg__Command__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      mg400_msgs__msg__Command__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

mg400_msgs__msg__Command__Sequence *
mg400_msgs__msg__Command__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__Command__Sequence * array = (mg400_msgs__msg__Command__Sequence *)allocator.allocate(sizeof(mg400_msgs__msg__Command__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = mg400_msgs__msg__Command__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
mg400_msgs__msg__Command__Sequence__destroy(mg400_msgs__msg__Command__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    mg400_msgs__msg__Command__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
mg400_msgs__msg__Command__Sequence__are_equal(const mg400_msgs__msg__Command__Sequence * lhs, const mg400_msgs__msg__Command__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!mg400_msgs__msg__Command__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
mg400_msgs__msg__Command__Sequence__copy(
  const mg400_msgs__msg__Command__Sequence * input,
  mg400_msgs__msg__Command__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(mg400_msgs__msg__Command);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    mg400_msgs__msg__Command * data =
      (mg400_msgs__msg__Command *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!mg400_msgs__msg__Command__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          mg400_msgs__msg__Command__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!mg400_msgs__msg__Command__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
