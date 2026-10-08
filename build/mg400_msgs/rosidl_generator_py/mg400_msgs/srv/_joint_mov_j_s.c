// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from mg400_msgs:srv/JointMovJ.idl
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
#include "mg400_msgs/srv/detail/joint_mov_j__struct.h"
#include "mg400_msgs/srv/detail/joint_mov_j__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__srv__joint_mov_j__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[46];
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
    assert(strncmp("mg400_msgs.srv._joint_mov_j.JointMovJ_Request", full_classname_dest, 45) == 0);
  }
  mg400_msgs__srv__JointMovJ_Request * ros_message = _ros_message;
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
  {  // j5
    PyObject * field = PyObject_GetAttrString(_pymsg, "j5");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->j5 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // j6
    PyObject * field = PyObject_GetAttrString(_pymsg, "j6");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->j6 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__srv__joint_mov_j__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of JointMovJ_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.srv._joint_mov_j");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "JointMovJ_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__srv__JointMovJ_Request * ros_message = (mg400_msgs__srv__JointMovJ_Request *)raw_ros_message;
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
  {  // j5
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->j5);
    {
      int rc = PyObject_SetAttrString(_pymessage, "j5", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // j6
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->j6);
    {
      int rc = PyObject_SetAttrString(_pymessage, "j6", field);
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
// #include "mg400_msgs/srv/detail/joint_mov_j__struct.h"
// already included above
// #include "mg400_msgs/srv/detail/joint_mov_j__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__srv__joint_mov_j__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[47];
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
    assert(strncmp("mg400_msgs.srv._joint_mov_j.JointMovJ_Response", full_classname_dest, 46) == 0);
  }
  mg400_msgs__srv__JointMovJ_Response * ros_message = _ros_message;
  {  // error_id
    PyObject * field = PyObject_GetAttrString(_pymsg, "error_id");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->error_id = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__srv__joint_mov_j__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of JointMovJ_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.srv._joint_mov_j");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "JointMovJ_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__srv__JointMovJ_Response * ros_message = (mg400_msgs__srv__JointMovJ_Response *)raw_ros_message;
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

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
