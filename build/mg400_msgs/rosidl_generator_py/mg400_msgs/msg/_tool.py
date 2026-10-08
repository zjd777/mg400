# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/Tool.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_Tool(type):
    """Metaclass of message 'Tool'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'TOOL0': 0,
        'TOOL1': 1,
        'TOOL2': 2,
        'TOOL3': 3,
        'TOOL4': 4,
        'TOOL5': 5,
        'TOOL6': 6,
        'TOOL7': 7,
        'TOOL8': 8,
        'TOOL9': 9,
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
                'mg400_msgs.msg.Tool')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__tool
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__tool
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__tool
            cls._TYPE_SUPPORT = module.type_support_msg__msg__tool
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__tool

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'TOOL0': cls.__constants['TOOL0'],
            'TOOL1': cls.__constants['TOOL1'],
            'TOOL2': cls.__constants['TOOL2'],
            'TOOL3': cls.__constants['TOOL3'],
            'TOOL4': cls.__constants['TOOL4'],
            'TOOL5': cls.__constants['TOOL5'],
            'TOOL6': cls.__constants['TOOL6'],
            'TOOL7': cls.__constants['TOOL7'],
            'TOOL8': cls.__constants['TOOL8'],
            'TOOL9': cls.__constants['TOOL9'],
        }

    @property
    def TOOL0(self):
        """Message constant 'TOOL0'."""
        return Metaclass_Tool.__constants['TOOL0']

    @property
    def TOOL1(self):
        """Message constant 'TOOL1'."""
        return Metaclass_Tool.__constants['TOOL1']

    @property
    def TOOL2(self):
        """Message constant 'TOOL2'."""
        return Metaclass_Tool.__constants['TOOL2']

    @property
    def TOOL3(self):
        """Message constant 'TOOL3'."""
        return Metaclass_Tool.__constants['TOOL3']

    @property
    def TOOL4(self):
        """Message constant 'TOOL4'."""
        return Metaclass_Tool.__constants['TOOL4']

    @property
    def TOOL5(self):
        """Message constant 'TOOL5'."""
        return Metaclass_Tool.__constants['TOOL5']

    @property
    def TOOL6(self):
        """Message constant 'TOOL6'."""
        return Metaclass_Tool.__constants['TOOL6']

    @property
    def TOOL7(self):
        """Message constant 'TOOL7'."""
        return Metaclass_Tool.__constants['TOOL7']

    @property
    def TOOL8(self):
        """Message constant 'TOOL8'."""
        return Metaclass_Tool.__constants['TOOL8']

    @property
    def TOOL9(self):
        """Message constant 'TOOL9'."""
        return Metaclass_Tool.__constants['TOOL9']


class Tool(metaclass=Metaclass_Tool):
    """
    Message class 'Tool'.

    Constants:
      TOOL0
      TOOL1
      TOOL2
      TOOL3
      TOOL4
      TOOL5
      TOOL6
      TOOL7
      TOOL8
      TOOL9
    """

    __slots__ = [
        '_tool',
    ]

    _fields_and_field_types = {
        'tool': 'uint8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.tool = kwargs.get('tool', int())

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
        if self.tool != other.tool:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def tool(self):
        """Message field 'tool'."""
        return self._tool

    @tool.setter
    def tool(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'tool' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'tool' field must be an unsigned integer in [0, 255]"
        self._tool = value
