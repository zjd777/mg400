// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from mg400_msgs:msg/MovLIO.idl
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
#include "mg400_msgs/msg/detail/mov_lio__struct.h"
#include "mg400_msgs/msg/detail/mov_lio__functions.h"

ROSIDL_GENERATOR_C_IMPORT
bool geometry_msgs__msg__pose_stamped__convert_from_py(PyObject * _pymsg, void * _ros_message);
ROSIDL_GENERATOR_C_IMPORT
PyObject * geometry_msgs__msg__pose_stamped__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__distance_mode__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__distance_mode__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__do_index__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__do_index__convert_to_py(void * raw_ros_message);
bool mg400_msgs__msg__do_status__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * mg400_msgs__msg__do_status__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__msg__mov_lio__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[31];
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
    assert(strncmp("mg400_msgs.msg._mov_lio.MovLIO", full_classname_dest, 30) == 0);
  }
  mg400_msgs__msg__MovLIO * ros_message = _ros_message;
  {  // pose
    PyObject * field = PyObject_GetAttrString(_pymsg, "pose");
    if (!field) {
      return false;
    }
    if (!geometry_msgs__msg__pose_stamped__convert_from_py(field, &ros_message->pose)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // mode
    PyObject * field = PyObject_GetAttrString(_pymsg, "mode");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__distance_mode__convert_from_py(field, &ros_message->mode)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // distance
    PyObject * field = PyObject_GetAttrString(_pymsg, "distance");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->distance = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // index
    PyObject * field = PyObject_GetAttrString(_pymsg, "index");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__do_index__convert_from_py(field, &ros_message->index)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // status
    PyObject * field = PyObject_GetAttrString(_pymsg, "status");
    if (!field) {
      return false;
    }
    if (!mg400_msgs__msg__do_status__convert_from_py(field, &ros_message->status)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // set_speed_l
    PyObject * field = PyObject_GetAttrString(_pymsg, "set_speed_l");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->set_speed_l = (Py_True == field);
    Py_DECREF(field);
  }
  {  // speed_l
    PyObject * field = PyObject_GetAttrString(_pymsg, "speed_l");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->speed_l = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // set_acc_l
    PyObject * field = PyObject_GetAttrString(_pymsg, "set_acc_l");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->set_acc_l = (Py_True == field);
    Py_DECREF(field);
  }
  {  // acc_l
    PyObject * field = PyObject_GetAttrString(_pymsg, "acc_l");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->acc_l = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__msg__mov_lio__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of MovLIO */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.msg._mov_lio");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "MovLIO");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__msg__MovLIO * ros_message = (mg400_msgs__msg__MovLIO *)raw_ros_message;
  {  // pose
    PyObject * field = NULL;
    field = geometry_msgs__msg__pose_stamped__convert_to_py(&ros_message->pose);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "pose", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // mode
    PyObject * field = NULL;
    field = mg400_msgs__msg__distance_mode__convert_to_py(&ros_message->mode);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "mode", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // distance
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->distance);
    {
      int rc = PyObject_SetAttrString(_pymessage, "distance", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // index
    PyObject * field = NULL;
    field = mg400_msgs__msg__do_index__convert_to_py(&ros_message->index);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "index", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // status
    PyObject * field = NULL;
    field = mg400_msgs__msg__do_status__convert_to_py(&ros_message->status);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "status", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // set_speed_l
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->set_speed_l ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "set_speed_l", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // speed_l
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->speed_l);
    {
      int rc = PyObject_SetAttrString(_pymessage, "speed_l", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // set_acc_l
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->set_acc_l ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "set_acc_l", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // acc_l
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->acc_l);
    {
      int rc = PyObject_SetAttrString(_pymessage, "acc_l", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
