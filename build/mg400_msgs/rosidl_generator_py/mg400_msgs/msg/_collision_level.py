# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/CollisionLevel.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_CollisionLevel(type):
    """Metaclass of message 'CollisionLevel'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'OFF': 0,
        'LEVEL1': 1,
        'LEVEL2': 2,
        'LEVEL3': 3,
        'LEVEL4': 4,
        'LEVEL5': 5,
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
                'mg400_msgs.msg.CollisionLevel')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__collision_level
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__collision_level
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__collision_level
            cls._TYPE_SUPPORT = module.type_support_msg__msg__collision_level
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__collision_level

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'OFF': cls.__constants['OFF'],
            'LEVEL1': cls.__constants['LEVEL1'],
            'LEVEL2': cls.__constants['LEVEL2'],
            'LEVEL3': cls.__constants['LEVEL3'],
            'LEVEL4': cls.__constants['LEVEL4'],
            'LEVEL5': cls.__constants['LEVEL5'],
        }

    @property
    def OFF(self):
        """Message constant 'OFF'."""
        return Metaclass_CollisionLevel.__constants['OFF']

    @property
    def LEVEL1(self):
        """Message constant 'LEVEL1'."""
        return Metaclass_CollisionLevel.__constants['LEVEL1']

    @property
    def LEVEL2(self):
        """Message constant 'LEVEL2'."""
        return Metaclass_CollisionLevel.__constants['LEVEL2']

    @property
    def LEVEL3(self):
        """Message constant 'LEVEL3'."""
        return Metaclass_CollisionLevel.__constants['LEVEL3']

    @property
    def LEVEL4(self):
        """Message constant 'LEVEL4'."""
        return Metaclass_CollisionLevel.__constants['LEVEL4']

    @property
    def LEVEL5(self):
        """Message constant 'LEVEL5'."""
        return Metaclass_CollisionLevel.__constants['LEVEL5']


class CollisionLevel(metaclass=Metaclass_CollisionLevel):
    """
    Message class 'CollisionLevel'.

    Constants:
      OFF
      LEVEL1
      LEVEL2
      LEVEL3
      LEVEL4
      LEVEL5
    """

    __slots__ = [
        '_level',
    ]

    _fields_and_field_types = {
        'level': 'uint8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.level = kwargs.get('level', int())

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
        if self.level != other.level:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def level(self):
        """Message field 'level'."""
        return self._level

    @level.setter
    def level(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'level' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'level' field must be an unsigned integer in [0, 255]"
        self._level = value
