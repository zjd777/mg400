// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "mg400_msgs/msg/detail/command__struct.h"
#include "mg400_msgs/msg/detail/command__functions.h"

bool mg400_msgs__msg__mov_j__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__mov_j__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__mov_l__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__mov_l__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__joint_mov_j__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__joint_mov_j__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__mov_jio__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__mov_jio__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__mov_lio__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__mov_lio__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__msg__command__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[32];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("mg400_msgs.msg._command.Command", full_classname_dest, 31) == 0);
  }
  mg400_msgs__msg__Command * ros_message = _ros_message;
  {  // command_type
    PyObject * field = PyObject_GetAttrString(_pymsg, "command_type");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->command_type = (uint16_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // mov_j_params
    PyObject * field = PyObject_GetAttrString(_pymsg, "mov_j_params");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__mov_j__convert_from_py(field, &ros_message->mov_j_params)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // mov_l_params
    PyObject * field = PyObject_GetAttrString(_pymsg, "mov_l_params");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__mov_l__convert_from_py(field, &ros_message->mov_l_params)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // joint_mov_j_params
    PyObject * field = PyObject_GetAttrString(_pymsg, "joint_mov_j_params");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__joint_mov_j__convert_from_py(field, &ros_message->joint_mov_j_params)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // mov_jio_params
    PyObject * field = PyObject_GetAttrString(_pymsg, "mov_jio_params");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__mov_jio__convert_from_py(field, &ros_message->mov_jio_params)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // mov_lio_params
    PyObject * field = PyObject_GetAttrString(_pymsg, "mov_lio_params");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__mov_lio__convert_from_py(field, &ros_message->mov_lio_params)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__msg__command__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of Command */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.msg._command");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "Command");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__msg__Command * ros_message = (mg400_msgs__msg__Command *)raw_ros_message;
  {  // command_type
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->command_type);
    {
      int rc = PyObject_SetAttrString(_pymessage, "command_type", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // mov_j_params
    PyObject * field = NULL;
    field = mg400_msgs__msg__mov_j__convert_to_py(&ros_message->mov_j_params);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "mov_j_params", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // mov_l_params
    PyObject * field = NULL;
    field = mg400_msgs__msg__mov_l__convert_to_py(&ros_message->mov_l_params);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "mov_l_params", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // joint_mov_j_params
    PyObject * field = NULL;
    field = mg400_msgs__msg__joint_mov_j__convert_to_py(&ros_message->joint_mov_j_params);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "joint_mov_j_params", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // mov_jio_params
    PyObject * field = NULL;
    field = mg400_msgs__msg__mov_jio__convert_to_py(&ros_message->mov_jio_params);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "mov_jio_params", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // mov_lio_params
    PyObject * field = NULL;
    field = mg400_msgs__msg__mov_lio__convert_to_py(&ros_message->mov_lio_params);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "mov_lio_params", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
