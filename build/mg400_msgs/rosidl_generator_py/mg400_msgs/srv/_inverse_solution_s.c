// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from mg400_msgs:srv/InverseSolution.idl
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
#include "mg400_msgs/srv/detail/inverse_solution__struct.h"
#include "mg400_msgs/srv/detail/inverse_solution__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__srv__inverse_solution__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[57];
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
    assert(strncmp("mg400_msgs.srv._inverse_solution.InverseSolution_Request", full_classname_dest, 56) == 0);
  }
  mg400_msgs__srv__InverseSolution_Request * ros_message = _ros_message;
  {  // x
    PyObject * field = PyObject_GetAttrString(_pymsg, "x");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->x = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // y
    PyObject * field = PyObject_GetAttrString(_pymsg, "y");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->y = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // z
    PyObject * field = PyObject_GetAttrString(_pymsg, "z");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->z = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // r
    PyObject * field = PyObject_GetAttrString(_pymsg, "r");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->r = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // user
    PyObject * field = PyObject_GetAttrString(_pymsg, "user");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->user = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // tool
    PyObject * field = PyObject_GetAttrString(_pymsg, "tool");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->tool = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__srv__inverse_solution__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of InverseSolution_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.srv._inverse_solution");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "InverseSolution_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__srv__InverseSolution_Request * ros_message = (mg400_msgs__srv__InverseSolution_Request *)raw_ros_message;
  {  // x
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->x);
    {
      int rc = PyObject_SetAttrString(_pymessage, "x", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // y
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->y);
    {
      int rc = PyObject_SetAttrString(_pymessage, "y", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // z
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->z);
    {
      int rc = PyObject_SetAttrString(_pymessage, "z", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // r
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->r);
    {
      int rc = PyObject_SetAttrString(_pymessage, "r", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // user
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->user);
    {
      int rc = PyObject_SetAttrString(_pymessage, "user", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // tool
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->tool);
    {
      int rc = PyObject_SetAttrString(_pymessage, "tool", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "mg400_msgs/srv/detail/inverse_solution__struct.h"
// already included above
// #include "mg400_msgs/srv/detail/inverse_solution__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__srv__inverse_solution__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[58];
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
    assert(strncmp("mg400_msgs.srv._inverse_solution.InverseSolution_Response", full_classname_dest, 57) == 0);
  }
  mg400_msgs__srv__InverseSolution_Response * ros_message = _ros_message;
  {  // error_id
    PyObject * field = PyObject_GetAttrString(_pymsg, "error_id");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->error_id = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // j1
    PyObject * field = PyObject_GetAttrString(_pymsg, "j1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->j1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // j2
    PyObject * field = PyObject_GetAttrString(_pymsg, "j2");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->j2 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // j3
    PyObject * field = PyObject_GetAttrString(_pymsg, "j3");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->j3 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // j4
    PyObject * field = PyObject_GetAttrString(_pymsg, "j4");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->j4 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__srv__inverse_solution__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of InverseSolution_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.srv._inverse_solution");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "InverseSolution_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__srv__InverseSolution_Response * ros_message = (mg400_msgs__srv__InverseSolution_Response *)raw_ros_message;
  {  // error_id
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->error_id);
    {
      int rc = PyObject_SetAttrString(_pymessage, "error_id", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // j1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->j1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "j1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // j2
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->j2);
    {
      int rc = PyObject_SetAttrString(_pymessage, "j2", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // j3
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->j3);
    {
      int rc = PyObject_SetAttrString(_pymessage, "j3", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // j4
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->j4);
    {
      int rc = PyObject_SetAttrString(_pymessage, "j4", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
