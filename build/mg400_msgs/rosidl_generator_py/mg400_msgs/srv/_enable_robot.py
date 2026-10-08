# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:srv/EnableRobot.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_EnableRobot_Request(type):
    """Metaclass of message 'EnableRobot_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'NO_PARAM': 0,
        'ONE_PARAM': 1,
        'FOUR_PARAM': 4,
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
                'mg400_msgs.srv.EnableRobot_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__enable_robot__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__enable_robot__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__enable_robot__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__enable_robot__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__enable_robot__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'NO_PARAM': cls.__constants['NO_PARAM'],
            'ONE_PARAM': cls.__constants['ONE_PARAM'],
            'FOUR_PARAM': cls.__constants['FOUR_PARAM'],
        }

    @property
    def NO_PARAM(self):
        """Message constant 'NO_PARAM'."""
        return Metaclass_EnableRobot_Request.__constants['NO_PARAM']

    @property
    def ONE_PARAM(self):
        """Message constant 'ONE_PARAM'."""
        return Metaclass_EnableRobot_Request.__constants['ONE_PARAM']

    @property
    def FOUR_PARAM(self):
        """Message constant 'FOUR_PARAM'."""
        return Metaclass_EnableRobot_Request.__constants['FOUR_PARAM']


class EnableRobot_Request(metaclass=Metaclass_EnableRobot_Request):
    """
    Message class 'EnableRobot_Request'.

    Constants:
      NO_PARAM
      ONE_PARAM
      FOUR_PARAM
    """

    __slots__ = [
        '_num_of_params',
        '_load',
        '_center_x',
        '_center_y',
        '_center_z',
    ]

    _fields_and_field_types = {
        'num_of_params': 'int8',
        'load': 'double',
        'center_x': 'double',
        'center_y': 'double',
        'center_z': 'double',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.num_of_params = kwargs.get('num_of_params', int())
        self.load = kwargs.get('load', float())
        self.center_x = kwargs.get('center_x', float())
        self.center_y = kwargs.get('center_y', float())
        self.center_z = kwargs.get('center_z', float())

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
        if self.num_of_params != other.num_of_params:
            return False
        if self.load != other.load:
            return False
        if self.center_x != other.center_x:
            return False
        if self.center_y != other.center_y:
            return False
        if self.center_z != other.center_z:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def num_of_params(self):
        """Message field 'num_of_params'."""
        return self._num_of_params

    @num_of_params.setter
    def num_of_params(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'num_of_params' field must be of type 'int'"
            assert value >= -128 and value < 128, \
                "The 'num_of_params' field must be an integer in [-128, 127]"
        self._num_of_params = value

    @builtins.property
    def load(self):
        """Message field 'load'."""
        return self._load

    @load.setter
    def load(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'load' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'load' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._load = value

    @builtins.property
    def center_x(self):
        """Message field 'center_x'."""
        return self._center_x

    @center_x.setter
    def center_x(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'center_x' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'center_x' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._center_x = value

    @builtins.property
    def center_y(self):
        """Message field 'center_y'."""
        return self._center_y

    @center_y.setter
    def center_y(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'center_y' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'center_y' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._center_y = value

    @builtins.property
    def center_z(self):
        """Message field 'center_z'."""
        return self._center_z

    @center_z.setter
    def center_z(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'center_z' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'center_z' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._center_z = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_EnableRobot_Response(type):
    """Metaclass of message 'EnableRobot_Response'."""

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
                'mg400_msgs.srv.EnableRobot_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__enable_robot__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__enable_robot__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__enable_robot__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__enable_robot__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__enable_robot__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class EnableRobot_Response(metaclass=Metaclass_EnableRobot_Response):
    """Message class 'EnableRobot_Response'."""

    __slots__ = [
        '_result',
        '_error_id',
    ]

    _fields_and_field_types = {
        'result': 'boolean',
        'error_id': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.result = kwargs.get('result', bool())
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
        if self.result != other.result:
            return False
        if self.error_id != other.error_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def result(self):
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'result' field must be of type 'bool'"
        self._result = value

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


class Metaclass_EnableRobot(type):
    """Metaclass of service 'EnableRobot'."""

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
                'mg400_msgs.srv.EnableRobot')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__enable_robot

            from mg400_msgs.srv import _enable_robot
            if _enable_robot.Metaclass_EnableRobot_Request._TYPE_SUPPORT is None:
                _enable_robot.Metaclass_EnableRobot_Request.__import_type_support__()
            if _enable_robot.Metaclass_EnableRobot_Response._TYPE_SUPPORT is None:
                _enable_robot.Metaclass_EnableRobot_Response.__import_type_support__()


class EnableRobot(metaclass=Metaclass_EnableRobot):
    from mg400_msgs.srv._enable_robot import EnableRobot_Request as Request
    from mg400_msgs.srv._enable_robot import EnableRobot_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
