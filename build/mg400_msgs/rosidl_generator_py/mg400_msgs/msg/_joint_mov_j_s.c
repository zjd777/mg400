// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from mg400_msgs:msg/JointMovJ.idl
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
#include "mg400_msgs/msg/detail/joint_mov_j__struct.h"
#include "mg400_msgs/msg/detail/joint_mov_j__functions.h"

#include "rosidl_runtime_c/primitives_sequence.h"
#include "rosidl_runtime_c/primitives_sequence_functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool mg400_msgs__msg__joint_mov_j__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[38];
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
    assert(strncmp("mg400_msgs.msg._joint_mov_j.JointMovJ", full_classname_dest, 37) == 0);
  }
  mg400_msgs__msg__JointMovJ * ros_message = _ros_message;
  {  // joint_angles
    PyObject * field = PyObject_GetAttrString(_pymsg, "joint_angles");
    if (!field) {
      return false;
    }
    {
      // TODO(dirk-thomas) use a better way to check the type before casting
      assert(field->ob_type != NULL);
      assert(field->ob_type->tp_name != NULL);
      assert(strcmp(field->ob_type->tp_name, "numpy.ndarray") == 0);
      PyArrayObject * seq_field = (PyArrayObject *)field;
      Py_INCREF(seq_field);
      assert(PyArray_NDIM(seq_field) == 1);
      assert(PyArray_TYPE(seq_field) == NPY_FLOAT64);
      Py_ssize_t size = 4;
      double * dest = ros_message->joint_angles;
      for (Py_ssize_t i = 0; i < size; ++i) {
        double tmp = *(npy_float64 *)PyArray_GETPTR1(seq_field, i);
        memcpy(&dest[i], &tmp, sizeof(double));
      }
      Py_DECREF(seq_field);
    }
    Py_DECREF(field);
  }
  {  // set_speed_j
    PyObject * field = PyObject_GetAttrString(_pymsg, "set_speed_j");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->set_speed_j = (Py_True == field);
    Py_DECREF(field);
  }
  {  // speed_j
    PyObject * field = PyObject_GetAttrString(_pymsg, "speed_j");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->speed_j = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // set_acc_j
    PyObject * field = PyObject_GetAttrString(_pymsg, "set_acc_j");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->set_acc_j = (Py_True == field);
    Py_DECREF(field);
  }
  {  // acc_j
    PyObject * field = PyObject_GetAttrString(_pymsg, "acc_j");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->acc_j = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // set_cp
    PyObject * field = PyObject_GetAttrString(_pymsg, "set_cp");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->set_cp = (Py_True == field);
    Py_DECREF(field);
  }
  {  // cp
    PyObject * field = PyObject_GetAttrString(_pymsg, "cp");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->cp = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * mg400_msgs__msg__joint_mov_j__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of JointMovJ */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("mg400_msgs.msg._joint_mov_j");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "JointMovJ");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  mg400_msgs__msg__JointMovJ * ros_message = (mg400_msgs__msg__JointMovJ *)raw_ros_message;
  {  // joint_angles
    PyObject * field = NULL;
    field = PyObject_GetAttrString(_pymessage, "joint_angles");
    if (!field) {
      return NULL;
    }
    assert(field->ob_type != NULL);
    assert(field->ob_type->tp_name != NULL);
    assert(strcmp(field->ob_type->tp_name, "numpy.ndarray") == 0);
    PyArrayObject * seq_field = (PyArrayObject *)field;
    assert(PyArray_NDIM(seq_field) == 1);
    assert(PyArray_TYPE(seq_field) == NPY_FLOAT64);
    assert(sizeof(npy_float64) == sizeof(double));
    npy_float64 * dst = (npy_float64 *)PyArray_GETPTR1(seq_field, 0);
    double * src = &(ros_message->joint_angles[0]);
    memcpy(dst, src, 4 * sizeof(double));
    Py_DECREF(field);
  }
  {  // set_speed_j
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->set_speed_j ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "set_speed_j", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // speed_j
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->speed_j);
    {
      int rc = PyObject_SetAttrString(_pymessage, "speed_j", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // set_acc_j
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->set_acc_j ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "set_acc_j", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // acc_j
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->acc_j);
    {
      int rc = PyObject_SetAttrString(_pymessage, "acc_j", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // set_cp
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->set_cp ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "set_cp", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // cp
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->cp);
    {
      int rc = PyObject_SetAttrString(_pymessage, "cp", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
