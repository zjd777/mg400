# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:srv/GetAngle.idl
# generated code does not contain a copyright notice


# Import statements for member types

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GetAngle_Request(type):
    """Metaclass of message 'GetAngle_Request'."""

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
                'mg400_msgs.srv.GetAngle_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_angle__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_angle__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_angle__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_angle__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_angle__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetAngle_Request(metaclass=Metaclass_GetAngle_Request):
    """Message class 'GetAngle_Request'."""

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

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
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

# already imported above
# import rosidl_parser.definition


class Metaclass_GetAngle_Response(type):
    """Metaclass of message 'GetAngle_Response'."""

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
                'mg400_msgs.srv.GetAngle_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_angle__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_angle__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_angle__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_angle__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_angle__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetAngle_Response(metaclass=Metaclass_GetAngle_Response):
    """Message class 'GetAngle_Response'."""

    __slots__ = [
        '_error_id',
        '_joint1',
        '_joint2',
        '_joint3',
        '_joint4',
        '_joint5',
        '_joint6',
    ]

    _fields_and_field_types = {
        'error_id': 'int32',
        'joint1': 'double',
        'joint2': 'double',
        'joint3': 'double',
        'joint4': 'double',
        'joint5': 'double',
        'joint6': 'double',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
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
        self.error_id = kwargs.get('error_id', int())
        self.joint1 = kwargs.get('joint1', float())
        self.joint2 = kwargs.get('joint2', float())
        self.joint3 = kwargs.get('joint3', float())
        self.joint4 = kwargs.get('joint4', float())
        self.joint5 = kwargs.get('joint5', float())
        self.joint6 = kwargs.get('joint6', float())

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
        if self.joint1 != other.joint1:
            return False
        if self.joint2 != other.joint2:
            return False
        if self.joint3 != other.joint3:
            return False
        if self.joint4 != other.joint4:
            return False
        if self.joint5 != other.joint5:
            return False
        if self.joint6 != other.joint6:
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

    @builtins.property
    def joint1(self):
        """Message field 'joint1'."""
        return self._joint1

    @joint1.setter
    def joint1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'joint1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'joint1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._joint1 = value

    @builtins.property
    def joint2(self):
        """Message field 'joint2'."""
        return self._joint2

    @joint2.setter
    def joint2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'joint2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'joint2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._joint2 = value

    @builtins.property
    def joint3(self):
        """Message field 'joint3'."""
        return self._joint3

    @joint3.setter
    def joint3(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'joint3' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'joint3' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._joint3 = value

    @builtins.property
    def joint4(self):
        """Message field 'joint4'."""
        return self._joint4

    @joint4.setter
    def joint4(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'joint4' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'joint4' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._joint4 = value

    @builtins.property
    def joint5(self):
        """Message field 'joint5'."""
        return self._joint5

    @joint5.setter
    def joint5(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'joint5' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'joint5' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._joint5 = value

    @builtins.property
    def joint6(self):
        """Message field 'joint6'."""
        return self._joint6

    @joint6.setter
    def joint6(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'joint6' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'joint6' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._joint6 = value


class Metaclass_GetAngle(type):
    """Metaclass of service 'GetAngle'."""

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
                'mg400_msgs.srv.GetAngle')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__get_angle

            from mg400_msgs.srv import _get_angle
            if _get_angle.Metaclass_GetAngle_Request._TYPE_SUPPORT is None:
                _get_angle.Metaclass_GetAngle_Request.__import_type_support__()
            if _get_angle.Metaclass_GetAngle_Response._TYPE_SUPPORT is None:
                _get_angle.Metaclass_GetAngle_Response.__import_type_support__()


class GetAngle(metaclass=Metaclass_GetAngle):
    from mg400_msgs.srv._get_angle import GetAngle_Request as Request
    from mg400_msgs.srv._get_angle import GetAngle_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
