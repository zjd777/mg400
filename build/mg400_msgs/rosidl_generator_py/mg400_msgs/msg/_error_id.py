# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/ErrorID.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ErrorID(type):
    """Metaclass of message 'ErrorID'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('mg400_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'mg400_msgs.msg.ErrorID')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__error_id
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__error_id
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__error_id
            cls._TYPE_SUPPORT = module.type_support_msg__msg__error_id
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__error_id

            from mg400_msgs.msg import IDArray
            if IDArray.__class__._TYPE_SUPPORT is None:
                IDArray.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ErrorID(metaclass=Metaclass_ErrorID):
    """Message class 'ErrorID'."""

    __slots__ = [
        '_controller',
        '_servo',
    ]

    _fields_and_field_types = {
        'controller': 'mg400_msgs/IDArray',
        'servo': 'mg400_msgs/IDArray[5]',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'IDArray'),  # noqa: E501
        rosidl_parser.definition.Array(rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'IDArray'), 5),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from mg400_msgs.msg import IDArray
        self.controller = kwargs.get('controller', IDArray())
        from mg400_msgs.msg import IDArray
        self.servo = kwargs.get(
            'servo',
            [IDArray() for x in range(5)]
        )

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.controller != other.controller:
            return False
        if self.servo != other.servo:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def controller(self):
        """Message field 'controller'."""
        return self._controller

    @controller.setter
    def controller(self, value):
        if __debug__:
            from mg400_msgs.msg import IDArray
            assert \
                isinstance(value, IDArray), \
                "The 'controller' field must be a sub message of type 'IDArray'"
        self._controller = value

    @builtins.property
    def servo(self):
        """Message field 'servo'."""
        return self._servo

    @servo.setter
    def servo(self, value):
        if __debug__:
            from mg400_msgs.msg import IDArray
            from collections.abc import Sequence
            from collections.abc import Set
            from collections import UserList
            from collections import UserString
            assert \
                ((isinstance(value, Sequence) or
                  isinstance(value, Set) or
                  isinstance(value, UserList)) and
                 not isinstance(value, str) and
                 not isinstance(value, UserString) and
                 len(value) == 5 and
                 all(isinstance(v, IDArray) for v in value) and
                 True), \
                "The 'servo' field must be a set or sequence with length 5 and each value of type 'IDArray'"
        self._servo = value
