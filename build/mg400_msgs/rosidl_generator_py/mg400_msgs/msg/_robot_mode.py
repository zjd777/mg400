# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/RobotMode.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_RobotMode(type):
    """Metaclass of message 'RobotMode'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'INIT': 1,
        'BRAKE_OPEN': 2,
        'DISABLED': 4,
        'ENABLE': 5,
        'BACKDRIVE': 6,
        'RUNNING': 7,
        'RECORDING': 8,
        'ERROR': 9,
        'PAUSE': 10,
        'JOG': 11,
        'INVALID': 12,
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
                'mg400_msgs.msg.RobotMode')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__robot_mode
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__robot_mode
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__robot_mode
            cls._TYPE_SUPPORT = module.type_support_msg__msg__robot_mode
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__robot_mode

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'INIT': cls.__constants['INIT'],
            'BRAKE_OPEN': cls.__constants['BRAKE_OPEN'],
            'DISABLED': cls.__constants['DISABLED'],
            'ENABLE': cls.__constants['ENABLE'],
            'BACKDRIVE': cls.__constants['BACKDRIVE'],
            'RUNNING': cls.__constants['RUNNING'],
            'RECORDING': cls.__constants['RECORDING'],
            'ERROR': cls.__constants['ERROR'],
            'PAUSE': cls.__constants['PAUSE'],
            'JOG': cls.__constants['JOG'],
            'INVALID': cls.__constants['INVALID'],
        }

    @property
    def INIT(self):
        """Message constant 'INIT'."""
        return Metaclass_RobotMode.__constants['INIT']

    @property
    def BRAKE_OPEN(self):
        """Message constant 'BRAKE_OPEN'."""
        return Metaclass_RobotMode.__constants['BRAKE_OPEN']

    @property
    def DISABLED(self):
        """Message constant 'DISABLED'."""
        return Metaclass_RobotMode.__constants['DISABLED']

    @property
    def ENABLE(self):
        """Message constant 'ENABLE'."""
        return Metaclass_RobotMode.__constants['ENABLE']

    @property
    def BACKDRIVE(self):
        """Message constant 'BACKDRIVE'."""
        return Metaclass_RobotMode.__constants['BACKDRIVE']

    @property
    def RUNNING(self):
        """Message constant 'RUNNING'."""
        return Metaclass_RobotMode.__constants['RUNNING']

    @property
    def RECORDING(self):
        """Message constant 'RECORDING'."""
        return Metaclass_RobotMode.__constants['RECORDING']

    @property
    def ERROR(self):
        """Message constant 'ERROR'."""
        return Metaclass_RobotMode.__constants['ERROR']

    @property
    def PAUSE(self):
        """Message constant 'PAUSE'."""
        return Metaclass_RobotMode.__constants['PAUSE']

    @property
    def JOG(self):
        """Message constant 'JOG'."""
        return Metaclass_RobotMode.__constants['JOG']

    @property
    def INVALID(self):
        """Message constant 'INVALID'."""
        return Metaclass_RobotMode.__constants['INVALID']


class RobotMode(metaclass=Metaclass_RobotMode):
    """
    Message class 'RobotMode'.

    Constants:
      INIT
      BRAKE_OPEN
      DISABLED
      ENABLE
      BACKDRIVE
      RUNNING
      RECORDING
      ERROR
      PAUSE
      JOG
      INVALID
    """

    __slots__ = [
        '_robot_mode',
    ]

    _fields_and_field_types = {
        'robot_mode': 'uint64',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint64'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.robot_mode = kwargs.get('robot_mode', int())

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
        if self.robot_mode != other.robot_mode:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def robot_mode(self):
        """Message field 'robot_mode'."""
        return self._robot_mode

    @robot_mode.setter
    def robot_mode(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'robot_mode' field must be of type 'int'"
            assert value >= 0 and value < 18446744073709551616, \
                "The 'robot_mode' field must be an unsigned integer in [0, 18446744073709551615]"
        self._robot_mode = value
