# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:srv/GetPose.idl
# generated code does not contain a copyright notice


# Import statements for member types

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GetPose_Request(type):
    """Metaclass of message 'GetPose_Request'."""

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
                'mg400_msgs.srv.GetPose_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_pose__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_pose__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_pose__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_pose__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_pose__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetPose_Request(metaclass=Metaclass_GetPose_Request):
    """Message class 'GetPose_Request'."""

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


class Metaclass_GetPose_Response(type):
    """Metaclass of message 'GetPose_Response'."""

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
                'mg400_msgs.srv.GetPose_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_pose__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_pose__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_pose__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_pose__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_pose__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetPose_Response(metaclass=Metaclass_GetPose_Response):
    """Message class 'GetPose_Response'."""

    __slots__ = [
        '_error_id',
        '_pose1',
        '_pose2',
        '_pose3',
        '_pose4',
        '_pose5',
        '_pose6',
    ]

    _fields_and_field_types = {
        'error_id': 'int32',
        'pose1': 'double',
        'pose2': 'double',
        'pose3': 'double',
        'pose4': 'double',
        'pose5': 'double',
        'pose6': 'double',
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
        self.pose1 = kwargs.get('pose1', float())
        self.pose2 = kwargs.get('pose2', float())
        self.pose3 = kwargs.get('pose3', float())
        self.pose4 = kwargs.get('pose4', float())
        self.pose5 = kwargs.get('pose5', float())
        self.pose6 = kwargs.get('pose6', float())

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
        if self.pose1 != other.pose1:
            return False
        if self.pose2 != other.pose2:
            return False
        if self.pose3 != other.pose3:
            return False
        if self.pose4 != other.pose4:
            return False
        if self.pose5 != other.pose5:
            return False
        if self.pose6 != other.pose6:
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
    def pose1(self):
        """Message field 'pose1'."""
        return self._pose1

    @pose1.setter
    def pose1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'pose1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'pose1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._pose1 = value

    @builtins.property
    def pose2(self):
        """Message field 'pose2'."""
        return self._pose2

    @pose2.setter
    def pose2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'pose2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'pose2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._pose2 = value

    @builtins.property
    def pose3(self):
        """Message field 'pose3'."""
        return self._pose3

    @pose3.setter
    def pose3(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'pose3' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'pose3' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._pose3 = value

    @builtins.property
    def pose4(self):
        """Message field 'pose4'."""
        return self._pose4

    @pose4.setter
    def pose4(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'pose4' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'pose4' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._pose4 = value

    @builtins.property
    def pose5(self):
        """Message field 'pose5'."""
        return self._pose5

    @pose5.setter
    def pose5(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'pose5' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'pose5' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._pose5 = value

    @builtins.property
    def pose6(self):
        """Message field 'pose6'."""
        return self._pose6

    @pose6.setter
    def pose6(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'pose6' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'pose6' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._pose6 = value


class Metaclass_GetPose(type):
    """Metaclass of service 'GetPose'."""

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
                'mg400_msgs.srv.GetPose')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__get_pose

            from mg400_msgs.srv import _get_pose
            if _get_pose.Metaclass_GetPose_Request._TYPE_SUPPORT is None:
                _get_pose.Metaclass_GetPose_Request.__import_type_support__()
            if _get_pose.Metaclass_GetPose_Response._TYPE_SUPPORT is None:
                _get_pose.Metaclass_GetPose_Response.__import_type_support__()


class GetPose(metaclass=Metaclass_GetPose):
    from mg400_msgs.srv._get_pose import GetPose_Request as Request
    from mg400_msgs.srv._get_pose import GetPose_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
