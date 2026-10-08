// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from mg400_msgs:msg/ErrorID.idl
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
#include "mg400_msgs/msg/detail/error_id__struct.h"
#include "mg400_msgs/msg/detail/error_id__functions.h"

#include "rosidl_runtime_c/primitives_sequence.h"
#include "rosidl_runtime_c/primitives_sequence_functions.h"

// Nested array functions includes
#include "mg400_msgs/msg/detail/id_array__functions.h"
// end nested array functions include
bool mg400_msgs__msg__id_array__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__id_array__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__id_array__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__id_array__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__msg__error_id__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[33];
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
    assert(strncmp("mg400_msgs.msg._error_id.ErrorID", full_classname_dest, 32) == 0);
  }
  mg400_msgs__msg__ErrorID * ros_message = _ros_message;
  {  // controller
    PyObject * field = PyObject_GetAttrString(_pymsg, "controller");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__id_array__convert_from_py(field, &ros_message->controller)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // servo
    PyObject * field = PyObject_GetAttrString(_pymsg, "servo");
    if (!field) {
      return false;
    }
    PyObject * seq_field = PySequence_Fast(field, "expected a sequence in 'servo'");
    if (!seq_field) {
      Py_DECREF(field);
      return false;
    }
    Py_ssize_t size = 5;
    mg400_msgs__msg__IDArray * dest = ros_message->servo;
    for (Py_ssize_t i = 0; i < size; ++i) {
      if (!mg400_msgs__msg__id_array__convert_from_py(PySequence_Fast_GET_ITEM(seq_field, i), &dest[i])) {
        Py_DECREF(seq_field);
        Py_DECREF(field);
        return false;
      }
    }
    Py_DECREF(seq_field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__msg__error_id__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of ErrorID */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.msg._error_id");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "ErrorID");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__msg__ErrorID * ros_message = (mg400_msgs__msg__ErrorID *)raw_ros_message;
  {  // controller
    PyObject * field = NULL;
    field = mg400_msgs__msg__id_array__convert_to_py(&ros_message->controller);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "controller", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // servo
    PyObject * field = NULL;
    size_t size = 5;
    field = PyList_New(size);
    if (!field) {
      return NULL;
    }
    mg400_msgs__msg__IDArray * item;
    for (size_t i = 0; i < size; ++i) {
      item = &(ros_message->servo[i]);
      PyObject * pyitem = mg400_msgs__msg__id_array__convert_to_py(item);
      if (!pyitem) {
        Py_DECREF(field);
        return NULL;
      }
      int rc = PyList_SetItem(field, i, pyitem);
      (void)rc;
      assert(rc == 0);
    }
    assert(PySequence_Check(field));
    {
      int rc = PyObject_SetAttrString(_pymessage, "servo", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
