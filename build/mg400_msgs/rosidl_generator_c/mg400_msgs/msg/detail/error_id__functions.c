// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/msg/detail/error_id__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `controller`
// Member `servo`
#include "mg400_msgs/msg/detail/id_array__functions.h"

bool
mg400_msgs__msg__ErrorID__init(mg400_msgs__msg__ErrorID * msg)
{
  if (!msg) {
    return false;
  }
  // controller
  if (!mg400_msgs__msg__IDArray__init(&msg->controller)) {
    mg400_msgs__msg__ErrorID__fini(msg);
    return false;
  }
  // servo
  for (size_t i = 0; i < 5; ++i) {
    if (!mg400_msgs__msg__IDArray__init(&msg->servo[i])) {
      mg400_msgs__msg__ErrorID__fini(msg);
      return false;
    }
  }
  return true;
}

void
mg400_msgs__msg__ErrorID__fini(mg400_msgs__msg__ErrorID * msg)
{
  if (!msg) {
    return;
  }
  // controller
  mg400_msgs__msg__IDArray__fini(&msg->controller);
  // servo
  for (size_t i = 0; i < 5; ++i) {
    mg400_msgs__msg__IDArray__fini(&msg->servo[i]);
  }
}

bool
mg400_msgs__msg__ErrorID__are_equal(const mg400_msgs__msg__ErrorID * lhs, const mg400_msgs__msg__ErrorID * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // controller
  if (!mg400_msgs__msg__IDArray__are_equal(
      &(lhs->controller), &(rhs->controller)))
  {
    return false;
  }
  // servo
  for (size_t i = 0; i < 5; ++i) {
    if (!mg400_msgs__msg__IDArray__are_equal(
        &(lhs->servo[i]), &(rhs->servo[i])))
    {
      return false;
    }
  }
  return true;
}

bool
mg400_msgs__msg__ErrorID__copy(
  const mg400_msgs__msg__ErrorID * input,
  mg400_msgs__msg__ErrorID * output)
{
  if (!input || !output) {
    return false;
  }
  // controller
  if (!mg400_msgs__msg__IDArray__copy(
      &(input->controller), &(output->controller)))
  {
    return false;
  }
  // servo
  for (size_t i = 0; i < 5; ++i) {
    if (!mg400_msgs__msg__IDArray__copy(
        &(input->servo[i]), &(output->servo[i])))
    {
      return false;
    }
  }
  return true;
}

mg400_msgs__msg__ErrorID *
mg400_msgs__msg__ErrorID__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__ErrorID * msg = (mg400_msgs__msg__ErrorID *)allocator.allocate(sizeof(mg400_msgs__msg__ErrorID), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(mg400_msgs__msg__ErrorID));
  bool success = mg400_msgs__msg__ErrorID__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
mg400_msgs__msg__ErrorID__destroy(mg400_msgs__msg__ErrorID * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    mg400_msgs__msg__ErrorID__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
mg400_msgs__msg__ErrorID__Sequence__init(mg400_msgs__msg__ErrorID__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__ErrorID * data = NULL;

  if (size) {
    data = (mg400_msgs__msg__ErrorID *)allocator.zero_allocate(size, sizeof(mg400_msgs__msg__ErrorID), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = mg400_msgs__msg__ErrorID__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        mg400_msgs__msg__ErrorID__fini(&data[i - 1]);
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
mg400_msgs__msg__ErrorID__Sequence__fini(mg400_msgs__msg__ErrorID__Sequence * array)
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
      mg400_msgs__msg__ErrorID__fini(&array->data[i]);
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

mg400_msgs__msg__ErrorID__Sequence *
mg400_msgs__msg__ErrorID__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__ErrorID__Sequence * array = (mg400_msgs__msg__ErrorID__Sequence *)allocator.allocate(sizeof(mg400_msgs__msg__ErrorID__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = mg400_msgs__msg__ErrorID__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
mg400_msgs__msg__ErrorID__Sequence__destroy(mg400_msgs__msg__ErrorID__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    mg400_msgs__msg__ErrorID__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
mg400_msgs__msg__ErrorID__Sequence__are_equal(const mg400_msgs__msg__ErrorID__Sequence * lhs, const mg400_msgs__msg__ErrorID__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!mg400_msgs__msg__ErrorID__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
mg400_msgs__msg__ErrorID__Sequence__copy(
  const mg400_msgs__msg__ErrorID__Sequence * input,
  mg400_msgs__msg__ErrorID__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(mg400_msgs__msg__ErrorID);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    mg400_msgs__msg__ErrorID * data =
      (mg400_msgs__msg__ErrorID *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!mg400_msgs__msg__ErrorID__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          mg400_msgs__msg__ErrorID__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!mg400_msgs__msg__ErrorID__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
