# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/User.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_User(type):
    """Metaclass of message 'User'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'USER0': 0,
        'USER1': 1,
        'USER2': 2,
        'USER3': 3,
        'USER4': 4,
        'USER5': 5,
        'USER6': 6,
        'USER7': 7,
        'USER8': 8,
        'USER9': 9,
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
                'mg400_msgs.msg.User')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__user
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__user
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__user
            cls._TYPE_SUPPORT = module.type_support_msg__msg__user
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__user

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'USER0': cls.__constants['USER0'],
            'USER1': cls.__constants['USER1'],
            'USER2': cls.__constants['USER2'],
            'USER3': cls.__constants['USER3'],
            'USER4': cls.__constants['USER4'],
            'USER5': cls.__constants['USER5'],
            'USER6': cls.__constants['USER6'],
            'USER7': cls.__constants['USER7'],
            'USER8': cls.__constants['USER8'],
            'USER9': cls.__constants['USER9'],
        }

    @property
    def USER0(self):
        """Message constant 'USER0'."""
        return Metaclass_User.__constants['USER0']

    @property
    def USER1(self):
        """Message constant 'USER1'."""
        return Metaclass_User.__constants['USER1']

    @property
    def USER2(self):
        """Message constant 'USER2'."""
        return Metaclass_User.__constants['USER2']

    @property
    def USER3(self):
        """Message constant 'USER3'."""
        return Metaclass_User.__constants['USER3']

    @property
    def USER4(self):
        """Message constant 'USER4'."""
        return Metaclass_User.__constants['USER4']

    @property
    def USER5(self):
        """Message constant 'USER5'."""
        return Metaclass_User.__constants['USER5']

    @property
    def USER6(self):
        """Message constant 'USER6'."""
        return Metaclass_User.__constants['USER6']

    @property
    def USER7(self):
        """Message constant 'USER7'."""
        return Metaclass_User.__constants['USER7']

    @property
    def USER8(self):
        """Message constant 'USER8'."""
        return Metaclass_User.__constants['USER8']

    @property
    def USER9(self):
        """Message constant 'USER9'."""
        return Metaclass_User.__constants['USER9']


class User(metaclass=Metaclass_User):
    """
    Message class 'User'.

    Constants:
      USER0
      USER1
      USER2
      USER3
      USER4
      USER5
      USER6
      USER7
      USER8
      USER9
    """

    __slots__ = [
        '_user',
    ]

    _fields_and_field_types = {
        'user': 'uint8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.user = kwargs.get('user', int())

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
        if self.user != other.user:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def user(self):
        """Message field 'user'."""
        return self._user

    @user.setter
    def user(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'user' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'user' field must be an unsigned integer in [0, 255]"
        self._user = value
