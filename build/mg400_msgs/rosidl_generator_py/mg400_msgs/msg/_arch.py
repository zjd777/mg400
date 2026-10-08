# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/Arch.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_Arch(type):
    """Metaclass of message 'Arch'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'INDEX0': 0,
        'INDEX1': 1,
        'INDEX2': 2,
        'INDEX3': 3,
        'INDEX4': 4,
        'INDEX5': 5,
        'INDEX6': 6,
        'INDEX7': 7,
        'INDEX8': 8,
        'INDEX9': 9,
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
                'mg400_msgs.msg.Arch')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__arch
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__arch
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__arch
            cls._TYPE_SUPPORT = module.type_support_msg__msg__arch
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__arch

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'INDEX0': cls.__constants['INDEX0'],
            'INDEX1': cls.__constants['INDEX1'],
            'INDEX2': cls.__constants['INDEX2'],
            'INDEX3': cls.__constants['INDEX3'],
            'INDEX4': cls.__constants['INDEX4'],
            'INDEX5': cls.__constants['INDEX5'],
            'INDEX6': cls.__constants['INDEX6'],
            'INDEX7': cls.__constants['INDEX7'],
            'INDEX8': cls.__constants['INDEX8'],
            'INDEX9': cls.__constants['INDEX9'],
        }

    @property
    def INDEX0(self):
        """Message constant 'INDEX0'."""
        return Metaclass_Arch.__constants['INDEX0']

    @property
    def INDEX1(self):
        """Message constant 'INDEX1'."""
        return Metaclass_Arch.__constants['INDEX1']

    @property
    def INDEX2(self):
        """Message constant 'INDEX2'."""
        return Metaclass_Arch.__constants['INDEX2']

    @property
    def INDEX3(self):
        """Message constant 'INDEX3'."""
        return Metaclass_Arch.__constants['INDEX3']

    @property
    def INDEX4(self):
        """Message constant 'INDEX4'."""
        return Metaclass_Arch.__constants['INDEX4']

    @property
    def INDEX5(self):
        """Message constant 'INDEX5'."""
        return Metaclass_Arch.__constants['INDEX5']

    @property
    def INDEX6(self):
        """Message constant 'INDEX6'."""
        return Metaclass_Arch.__constants['INDEX6']

    @property
    def INDEX7(self):
        """Message constant 'INDEX7'."""
        return Metaclass_Arch.__constants['INDEX7']

    @property
    def INDEX8(self):
        """Message constant 'INDEX8'."""
        return Metaclass_Arch.__constants['INDEX8']

    @property
    def INDEX9(self):
        """Message constant 'INDEX9'."""
        return Metaclass_Arch.__constants['INDEX9']


class Arch(metaclass=Metaclass_Arch):
    """
    Message class 'Arch'.

    Constants:
      INDEX0
      INDEX1
      INDEX2
      INDEX3
      INDEX4
      INDEX5
      INDEX6
      INDEX7
      INDEX8
      INDEX9
    """

    __slots__ = [
        '_index',
    ]

    _fields_and_field_types = {
        'index': 'uint8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.index = kwargs.get('index', int())

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
        if self.index != other.index:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def index(self):
        """Message field 'index'."""
        return self._index

    @index.setter
    def index(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'index' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'index' field must be an unsigned integer in [0, 255]"
        self._index = value
