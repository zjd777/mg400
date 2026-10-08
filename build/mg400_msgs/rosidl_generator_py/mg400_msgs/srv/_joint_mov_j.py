# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:srv/JointMovJ.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_JointMovJ_Request(type):
    """Metaclass of message 'JointMovJ_Request'."""

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
                'mg400_msgs.srv.JointMovJ_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__joint_mov_j__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__joint_mov_j__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__joint_mov_j__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__joint_mov_j__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__joint_mov_j__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class JointMovJ_Request(metaclass=Metaclass_JointMovJ_Request):
    """Message class 'JointMovJ_Request'."""

    __slots__ = [
        '_j1',
        '_j2',
        '_j3',
        '_j4',
        '_j5',
        '_j6',
    ]

    _fields_and_field_types = {
        'j1': 'double',
        'j2': 'double',
        'j3': 'double',
        'j4': 'double',
        'j5': 'double',
        'j6': 'double',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.j1 = kwargs.get('j1', float())
        self.j2 = kwargs.get('j2', float())
        self.j3 = kwargs.get('j3', float())
        self.j4 = kwargs.get('j4', float())
        self.j5 = kwargs.get('j5', float())
        self.j6 = kwargs.get('j6', float())

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
        if self.j1 != other.j1:
            return False
        if self.j2 != other.j2:
            return False
        if self.j3 != other.j3:
            return False
        if self.j4 != other.j4:
            return False
        if self.j5 != other.j5:
            return False
        if self.j6 != other.j6:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def j1(self):
        """Message field 'j1'."""
        return self._j1

    @j1.setter
    def j1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'j1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'j1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._j1 = value

    @builtins.property
    def j2(self):
        """Message field 'j2'."""
        return self._j2

    @j2.setter
    def j2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'j2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'j2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._j2 = value

    @builtins.property
    def j3(self):
        """Message field 'j3'."""
        return self._j3

    @j3.setter
    def j3(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'j3' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'j3' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._j3 = value

    @builtins.property
    def j4(self):
        """Message field 'j4'."""
        return self._j4

    @j4.setter
    def j4(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'j4' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'j4' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._j4 = value

    @builtins.property
    def j5(self):
        """Message field 'j5'."""
        return self._j5

    @j5.setter
    def j5(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'j5' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'j5' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._j5 = value

    @builtins.property
    def j6(self):
        """Message field 'j6'."""
        return self._j6

    @j6.setter
    def j6(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'j6' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'j6' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._j6 = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_JointMovJ_Response(type):
    """Metaclass of message 'JointMovJ_Response'."""

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
                'mg400_msgs.srv.JointMovJ_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__joint_mov_j__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__joint_mov_j__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__joint_mov_j__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__joint_mov_j__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__joint_mov_j__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class JointMovJ_Response(metaclass=Metaclass_JointMovJ_Response):
    """Message class 'JointMovJ_Response'."""

    __slots__ = [
        '_error_id',
    ]

    _fields_and_field_types = {
        'error_id': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.error_id = kwargs.get('error_id', int())

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
        if self.error_id != other.error_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def error_id(self):
        """Message field 'error_id'."""
        return self._error_id

    @error_id.setter
    def error_id(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'error_id' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'error_id' field must be an integer in [-2147483648, 2147483647]"
        self._error_id = value


class Metaclass_JointMovJ(type):
    """Metaclass of service 'JointMovJ'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('mg400_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'mg400_msgs.srv.JointMovJ')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__joint_mov_j

            from mg400_msgs.srv import _joint_mov_j
            if _joint_mov_j.Metaclass_JointMovJ_Request._TYPE_SUPPORT is None:
                _joint_mov_j.Metaclass_JointMovJ_Request.__import_type_support__()
            if _joint_mov_j.Metaclass_JointMovJ_Response._TYPE_SUPPORT is None:
                _joint_mov_j.Metaclass_JointMovJ_Response.__import_type_support__()


class JointMovJ(metaclass=Metaclass_JointMovJ):
    from mg400_msgs.srv._joint_mov_j import JointMovJ_Request as Request
    from mg400_msgs.srv._joint_mov_j import JointMovJ_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
