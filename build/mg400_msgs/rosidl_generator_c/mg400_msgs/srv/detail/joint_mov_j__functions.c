// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from mg400_msgs:srv/JointMovJ.idl
// generated code does not contain a copyright notice
#include "mg400_msgs/srv/detail/joint_mov_j__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

bool
mg400_msgs__srv__JointMovJ_Request__init(mg400_msgs__srv__JointMovJ_Request * msg)
{
  if (!msg) {
    return false;
  }
  // j1
  // j2
  // j3
  // j4
  // j5
  // j6
  return true;
}

void
mg400_msgs__srv__JointMovJ_Request__fini(mg400_msgs__srv__JointMovJ_Request * msg)
{
  if (!msg) {
    return;
  }
  // j1
  // j2
  // j3
  // j4
  // j5
  // j6
}

bool
mg400_msgs__srv__JointMovJ_Request__are_equal(const mg400_msgs__srv__JointMovJ_Request * lhs, const mg400_msgs__srv__JointMovJ_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // j1
  if (lhs->j1 != rhs->j1) {
    return false;
  }
  // j2
  if (lhs->j2 != rhs->j2) {
    return false;
  }
  // j3
  if (lhs->j3 != rhs->j3) {
    return false;
  }
  // j4
  if (lhs->j4 != rhs->j4) {
    return false;
  }
  // j5
  if (lhs->j5 != rhs->j5) {
    return false;
  }
  // j6
  if (lhs->j6 != rhs->j6) {
    return false;
  }
  return true;
}

bool
mg400_msgs__srv__JointMovJ_Request__copy(
  const mg400_msgs__srv__JointMovJ_Request * input,
  mg400_msgs__srv__JointMovJ_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // j1
  output->j1 = input->j1;
  // j2
  output->j2 = input->j2;
  // j3
  output->j3 = input->j3;
  // j4
  output->j4 = input->j4;
  // j5
  output->j5 = input->j5;
  // j6
  output->j6 = input->j6;
  return true;
}

mg400_msgs__srv__JointMovJ_Request *
mg400_msgs__srv__JointMovJ_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__srv__JointMovJ_Request * msg = (mg400_msgs__srv__JointMovJ_Request *)allocator.allocate(sizeof(mg400_msgs__srv__JointMovJ_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(mg400_msgs__srv__JointMovJ_Request));
  bool success = mg400_msgs__srv__JointMovJ_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
mg400_msgs__srv__JointMovJ_Request__destroy(mg400_msgs__srv__JointMovJ_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    mg400_msgs__srv__JointMovJ_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
mg400_msgs__srv__JointMovJ_Request__Sequence__init(mg400_msgs__srv__JointMovJ_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__srv__JointMovJ_Request * data = NULL;

  if (size) {
    data = (mg400_msgs__srv__JointMovJ_Request *)allocator.zero_allocate(size, sizeof(mg400_msgs__srv__JointMovJ_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = mg400_msgs__srv__JointMovJ_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        mg400_msgs__srv__JointMovJ_Request__fini(&data[i - 1]);
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
mg400_msgs__srv__JointMovJ_Request__Sequence__fini(mg400_msgs__srv__JointMovJ_Request__Sequence * array)
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
      mg400_msgs__srv__JointMovJ_Request__fini(&array->data[i]);
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

mg400_msgs__srv__JointMovJ_Request__Sequence *
mg400_msgs__srv__JointMovJ_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__srv__JointMovJ_Request__Sequence * array = (mg400_msgs__srv__JointMovJ_Request__Sequence *)allocator.allocate(sizeof(mg400_msgs__srv__JointMovJ_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = mg400_msgs__srv__JointMovJ_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
mg400_msgs__srv__JointMovJ_Request__Sequence__destroy(mg400_msgs__srv__JointMovJ_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    mg400_msgs__srv__JointMovJ_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
mg400_msgs__srv__JointMovJ_Request__Sequence__are_equal(const mg400_msgs__srv__JointMovJ_Request__Sequence * lhs, const mg400_msgs__srv__JointMovJ_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!mg400_msgs__srv__JointMovJ_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
mg400_msgs__srv__JointMovJ_Request__Sequence__copy(
  const mg400_msgs__srv__JointMovJ_Request__Sequence * input,
  mg400_msgs__srv__JointMovJ_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(mg400_msgs__srv__JointMovJ_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    mg400_msgs__srv__JointMovJ_Request * data =
      (mg400_msgs__srv__JointMovJ_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!mg400_msgs__srv__JointMovJ_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          mg400_msgs__srv__JointMovJ_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!mg400_msgs__srv__JointMovJ_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


bool
mg400_msgs__srv__JointMovJ_Response__init(mg400_msgs__srv__JointMovJ_Response * msg)
{
  if (!msg) {
    return false;
  }
  // error_id
  return true;
}

void
mg400_msgs__srv__JointMovJ_Response__fini(mg400_msgs__srv__JointMovJ_Response * msg)
{
  if (!msg) {
    return;
  }
  // error_id
}

bool
mg400_msgs__srv__JointMovJ_Response__are_equal(const mg400_msgs__srv__JointMovJ_Response * lhs, const mg400_msgs__srv__JointMovJ_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // error_id
  if (lhs->error_id != rhs->error_id) {
    return false;
  }
  return true;
}

bool
mg400_msgs__srv__JointMovJ_Response__copy(
  const mg400_msgs__srv__JointMovJ_Response * input,
  mg400_msgs__srv__JointMovJ_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // error_id
  output->error_id = input->error_id;
  return true;
}

mg400_msgs__srv__JointMovJ_Response *
mg400_msgs__srv__JointMovJ_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__srv__JointMovJ_Response * msg = (mg400_msgs__srv__JointMovJ_Response *)allocator.allocate(sizeof(mg400_msgs__srv__JointMovJ_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(mg400_msgs__srv__JointMovJ_Response));
  bool success = mg400_msgs__srv__JointMovJ_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
mg400_msgs__srv__JointMovJ_Response__destroy(mg400_msgs__srv__JointMovJ_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    mg400_msgs__srv__JointMovJ_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
mg400_msgs__srv__JointMovJ_Response__Sequence__init(mg400_msgs__srv__JointMovJ_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__srv__JointMovJ_Response * data = NULL;

  if (size) {
    data = (mg400_msgs__srv__JointMovJ_Response *)allocator.zero_allocate(size, sizeof(mg400_msgs__srv__JointMovJ_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = mg400_msgs__srv__JointMovJ_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        mg400_msgs__srv__JointMovJ_Response__fini(&data[i - 1]);
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
mg400_msgs__srv__JointMovJ_Response__Sequence__fini(mg400_msgs__srv__JointMovJ_Response__Sequence * array)
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
      mg400_msgs__srv__JointMovJ_Response__fini(&array->data[i]);
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

mg400_msgs__srv__JointMovJ_Response__Sequence *
mg400_msgs__srv__JointMovJ_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  mg400_msgs__srv__JointMovJ_Response__Sequence * array = (mg400_msgs__srv__JointMovJ_Response__Sequence *)allocator.allocate(sizeof(mg400_msgs__srv__JointMovJ_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = mg400_msgs__srv__JointMovJ_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
mg400_msgs__srv__JointMovJ_Response__Sequence__destroy(mg400_msgs__srv__JointMovJ_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    mg400_msgs__srv__JointMovJ_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
mg400_msgs__srv__JointMovJ_Response__Sequence__are_equal(const mg400_msgs__srv__JointMovJ_Response__Sequence * lhs, const mg400_msgs__srv__JointMovJ_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!mg400_msgs__srv__JointMovJ_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
mg400_msgs__srv__JointMovJ_Response__Sequence__copy(
  const mg400_msgs__srv__JointMovJ_Response__Sequence * input,
  mg400_msgs__srv__JointMovJ_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(mg400_msgs__srv__JointMovJ_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    mg400_msgs__srv__JointMovJ_Response * data =
      (mg400_msgs__srv__JointMovJ_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!mg400_msgs__srv__JointMovJ_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          mg400_msgs__srv__JointMovJ_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!mg400_msgs__srv__JointMovJ_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
