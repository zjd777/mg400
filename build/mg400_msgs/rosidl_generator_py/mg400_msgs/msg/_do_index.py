# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/DOIndex.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_DOIndex(type):
    """Metaclass of message 'DOIndex'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'D1': 1,
        'D2': 2,
        'D3': 3,
        'D4': 4,
        'D5': 5,
        'D6': 6,
        'D7': 7,
        'D8': 8,
        'D9': 9,
        'D10': 10,
        'D11': 11,
        'D12': 12,
        'D13': 13,
        'D14': 14,
        'D15': 15,
        'D16': 16,
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
                'mg400_msgs.msg.DOIndex')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__do_index
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__do_index
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__do_index
            cls._TYPE_SUPPORT = module.type_support_msg__msg__do_index
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__do_index

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'D1': cls.__constants['D1'],
            'D2': cls.__constants['D2'],
            'D3': cls.__constants['D3'],
            'D4': cls.__constants['D4'],
            'D5': cls.__constants['D5'],
            'D6': cls.__constants['D6'],
            'D7': cls.__constants['D7'],
            'D8': cls.__constants['D8'],
            'D9': cls.__constants['D9'],
            'D10': cls.__constants['D10'],
            'D11': cls.__constants['D11'],
            'D12': cls.__constants['D12'],
            'D13': cls.__constants['D13'],
            'D14': cls.__constants['D14'],
            'D15': cls.__constants['D15'],
            'D16': cls.__constants['D16'],
        }

    @property
    def D1(self):
        """Message constant 'D1'."""
        return Metaclass_DOIndex.__constants['D1']

    @property
    def D2(self):
        """Message constant 'D2'."""
        return Metaclass_DOIndex.__constants['D2']

    @property
    def D3(self):
        """Message constant 'D3'."""
        return Metaclass_DOIndex.__constants['D3']

    @property
    def D4(self):
        """Message constant 'D4'."""
        return Metaclass_DOIndex.__constants['D4']

    @property
    def D5(self):
        """Message constant 'D5'."""
        return Metaclass_DOIndex.__constants['D5']

    @property
    def D6(self):
        """Message constant 'D6'."""
        return Metaclass_DOIndex.__constants['D6']

    @property
    def D7(self):
        """Message constant 'D7'."""
        return Metaclass_DOIndex.__constants['D7']

    @property
    def D8(self):
        """Message constant 'D8'."""
        return Metaclass_DOIndex.__constants['D8']

    @property
    def D9(self):
        """Message constant 'D9'."""
        return Metaclass_DOIndex.__constants['D9']

    @property
    def D10(self):
        """Message constant 'D10'."""
        return Metaclass_DOIndex.__constants['D10']

    @property
    def D11(self):
        """Message constant 'D11'."""
        return Metaclass_DOIndex.__constants['D11']

    @property
    def D12(self):
        """Message constant 'D12'."""
        return Metaclass_DOIndex.__constants['D12']

    @property
    def D13(self):
        """Message constant 'D13'."""
        return Metaclass_DOIndex.__constants['D13']

    @property
    def D14(self):
        """Message constant 'D14'."""
        return Metaclass_DOIndex.__constants['D14']

    @property
    def D15(self):
        """Message constant 'D15'."""
        return Metaclass_DOIndex.__constants['D15']

    @property
    def D16(self):
        """Message constant 'D16'."""
        return Metaclass_DOIndex.__constants['D16']


class DOIndex(metaclass=Metaclass_DOIndex):
    """
    Message class 'DOIndex'.

    Constants:
      D1
      D2
      D3
      D4
      D5
      D6
      D7
      D8
      D9
      D10
      D11
      D12
      D13
      D14
      D15
      D16
    """

    __slots__ = [
        '_index',
    ]

    _fields_and_field_types = {
        'index': 'uint32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint32'),  # noqa: E501
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
            assert value >= 0 and value < 4294967296, \
                "The 'index' field must be an unsigned integer in [0, 4294967295]"
        self._index = value
