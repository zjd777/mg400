// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from mg400_msgs:msg/MovJ.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/msg/detail/mov_j__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `pose`
#include "geometry_msgs/msg/detail/pose_stamped__functions.h"

bool
mg400_msgs__msg__MovJ__init(mg400_msgs__msg__MovJ * msg)
{
  if (!msg) {
    return false;
  }
  // pose
  if (!geometry_msgs__msg__PoseStamped__init(&msg->pose)) {
    mg400_msgs__msg__MovJ__fini(msg);
    return false;
  }
  // set_speed_j
  // speed_j
  // set_acc_j
  // acc_j
  // set_cp
  // cp
  return true;
}

void
mg400_msgs__msg__MovJ__fini(mg400_msgs__msg__MovJ * msg)
{
  if (!msg) {
    return;
  }
  // pose
  geometry_msgs__msg__PoseStamped__fini(&msg->pose);
  // set_speed_j
  // speed_j
  // set_acc_j
  // acc_j
  // set_cp
  // cp
}

bool
mg400_msgs__msg__MovJ__are_equal(const mg400_msgs__msg__MovJ * lhs, const mg400_msgs__msg__MovJ * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // pose
  if (!geometry_msgs__msg__PoseStamped__are_equal(
      &(lhs->pose), &(rhs->pose)))
  {
    return false;
  }
  // set_speed_j
  if (lhs->set_speed_j != rhs->set_speed_j) {
    return false;
  }
  // speed_j
  if (lhs->speed_j != rhs->speed_j) {
    return false;
  }
  // set_acc_j
  if (lhs->set_acc_j != rhs->set_acc_j) {
    return false;
  }
  // acc_j
  if (lhs->acc_j != rhs->acc_j) {
    return false;
  }
  // set_cp
  if (lhs->set_cp != rhs->set_cp) {
    return false;
  }
  // cp
  if (lhs->cp != rhs->cp) {
    return false;
  }
  return true;
}

bool
mg400_msgs__msg__MovJ__copy(
  const mg400_msgs__msg__MovJ * input,
  mg400_msgs__msg__MovJ * output)
{
  if (!input || !output) {
    return false;
  }
  // pose
  if (!geometry_msgs__msg__PoseStamped__copy(
      &(input->pose), &(output->pose)))
  {
    return false;
  }
  // set_speed_j
  output->set_speed_j = input->set_speed_j;
  // speed_j
  output->speed_j = input->speed_j;
  // set_acc_j
  output->set_acc_j = input->set_acc_j;
  // acc_j
  output->acc_j = input->acc_j;
  // set_cp
  output->set_cp = input->set_cp;
  // cp
  output->cp = input->cp;
  return true;
}

mg400_msgs__msg__MovJ *
mg400_msgs__msg__MovJ__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__MovJ * msg = (mg400_msgs__msg__MovJ *)allocator.allocate(sizeof(mg400_msgs__msg__MovJ), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(mg400_msgs__msg__MovJ));
  bool success = mg400_msgs__msg__MovJ__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
mg400_msgs__msg__MovJ__destroy(mg400_msgs__msg__MovJ * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    mg400_msgs__msg__MovJ__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
mg400_msgs__msg__MovJ__Sequence__init(mg400_msgs__msg__MovJ__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__MovJ * data = NULL;

  if (size) {
    data = (mg400_msgs__msg__MovJ *)allocator.zero_allocate(size, sizeof(mg400_msgs__msg__MovJ), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = mg400_msgs__msg__MovJ__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        mg400_msgs__msg__MovJ__fini(&data[i - 1]);
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
mg400_msgs__msg__MovJ__Sequence__fini(mg400_msgs__msg__MovJ__Sequence * array)
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
      mg400_msgs__msg__MovJ__fini(&array->data[i]);
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

mg400_msgs__msg__MovJ__Sequence *
mg400_msgs__msg__MovJ__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__msg__MovJ__Sequence * array = (mg400_msgs__msg__MovJ__Sequence *)allocator.allocate(sizeof(mg400_msgs__msg__MovJ__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = mg400_msgs__msg__MovJ__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
mg400_msgs__msg__MovJ__Sequence__destroy(mg400_msgs__msg__MovJ__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    mg400_msgs__msg__MovJ__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
mg400_msgs__msg__MovJ__Sequence__are_equal(const mg400_msgs__msg__MovJ__Sequence * lhs, const mg400_msgs__msg__MovJ__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!mg400_msgs__msg__MovJ__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
mg400_msgs__msg__MovJ__Sequence__copy(
  const mg400_msgs__msg__MovJ__Sequence * input,
  mg400_msgs__msg__MovJ__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(mg400_msgs__msg__MovJ);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    mg400_msgs__msg__MovJ * data =
      (mg400_msgs__msg__MovJ *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!mg400_msgs__msg__MovJ__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          mg400_msgs__msg__MovJ__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!mg400_msgs__msg__MovJ__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
