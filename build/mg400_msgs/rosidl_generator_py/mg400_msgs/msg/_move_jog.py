# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/MoveJog.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_MoveJog(type):
    """Metaclass of message 'MoveJog'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'J1_NEGATIVE': 'j1-',
        'J1_POSITIVE': 'j1+',
        'J2_NEGATIVE': 'j2-',
        'J2_POSITIVE': 'j2+',
        'J3_NEGATIVE': 'j3-',
        'J3_POSITIVE': 'j3+',
        'J4_NEGATIVE': 'j4-',
        'J4_POSITIVE': 'j4+',
        'X_NEGATIVE': 'X-',
        'X_POSITIVE': 'X+',
        'Y_NEGATIVE': 'Y-',
        'Y_POSITIVE': 'Y+',
        'Z_NEGATIVE': 'Z-',
        'Z_POSITIVE': 'Z+',
        'RX_NEGATIVE': 'Rx-',
        'RX_POSITIVE': 'Rx+',
        'RY_NEGATIVE': 'Ry-',
        'RY_POSITIVE': 'Ry+',
        'RZ_NEGATIVE': 'Rz-',
        'RZ_POSITIVE': 'Rz+',
        'STOP': '',
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
                'mg400_msgs.msg.MoveJog')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__move_jog
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__move_jog
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__move_jog
            cls._TYPE_SUPPORT = module.type_support_msg__msg__move_jog
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__move_jog

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'J1_NEGATIVE': cls.__constants['J1_NEGATIVE'],
            'J1_POSITIVE': cls.__constants['J1_POSITIVE'],
            'J2_NEGATIVE': cls.__constants['J2_NEGATIVE'],
            'J2_POSITIVE': cls.__constants['J2_POSITIVE'],
            'J3_NEGATIVE': cls.__constants['J3_NEGATIVE'],
            'J3_POSITIVE': cls.__constants['J3_POSITIVE'],
            'J4_NEGATIVE': cls.__constants['J4_NEGATIVE'],
            'J4_POSITIVE': cls.__constants['J4_POSITIVE'],
            'X_NEGATIVE': cls.__constants['X_NEGATIVE'],
            'X_POSITIVE': cls.__constants['X_POSITIVE'],
            'Y_NEGATIVE': cls.__constants['Y_NEGATIVE'],
            'Y_POSITIVE': cls.__constants['Y_POSITIVE'],
            'Z_NEGATIVE': cls.__constants['Z_NEGATIVE'],
            'Z_POSITIVE': cls.__constants['Z_POSITIVE'],
            'RX_NEGATIVE': cls.__constants['RX_NEGATIVE'],
            'RX_POSITIVE': cls.__constants['RX_POSITIVE'],
            'RY_NEGATIVE': cls.__constants['RY_NEGATIVE'],
            'RY_POSITIVE': cls.__constants['RY_POSITIVE'],
            'RZ_NEGATIVE': cls.__constants['RZ_NEGATIVE'],
            'RZ_POSITIVE': cls.__constants['RZ_POSITIVE'],
            'STOP': cls.__constants['STOP'],
        }

    @property
    def J1_NEGATIVE(self):
        """Message constant 'J1_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['J1_NEGATIVE']

    @property
    def J1_POSITIVE(self):
        """Message constant 'J1_POSITIVE'."""
        return Metaclass_MoveJog.__constants['J1_POSITIVE']

    @property
    def J2_NEGATIVE(self):
        """Message constant 'J2_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['J2_NEGATIVE']

    @property
    def J2_POSITIVE(self):
        """Message constant 'J2_POSITIVE'."""
        return Metaclass_MoveJog.__constants['J2_POSITIVE']

    @property
    def J3_NEGATIVE(self):
        """Message constant 'J3_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['J3_NEGATIVE']

    @property
    def J3_POSITIVE(self):
        """Message constant 'J3_POSITIVE'."""
        return Metaclass_MoveJog.__constants['J3_POSITIVE']

    @property
    def J4_NEGATIVE(self):
        """Message constant 'J4_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['J4_NEGATIVE']

    @property
    def J4_POSITIVE(self):
        """Message constant 'J4_POSITIVE'."""
        return Metaclass_MoveJog.__constants['J4_POSITIVE']

    @property
    def X_NEGATIVE(self):
        """Message constant 'X_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['X_NEGATIVE']

    @property
    def X_POSITIVE(self):
        """Message constant 'X_POSITIVE'."""
        return Metaclass_MoveJog.__constants['X_POSITIVE']

    @property
    def Y_NEGATIVE(self):
        """Message constant 'Y_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['Y_NEGATIVE']

    @property
    def Y_POSITIVE(self):
        """Message constant 'Y_POSITIVE'."""
        return Metaclass_MoveJog.__constants['Y_POSITIVE']

    @property
    def Z_NEGATIVE(self):
        """Message constant 'Z_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['Z_NEGATIVE']

    @property
    def Z_POSITIVE(self):
        """Message constant 'Z_POSITIVE'."""
        return Metaclass_MoveJog.__constants['Z_POSITIVE']

    @property
    def RX_NEGATIVE(self):
        """Message constant 'RX_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['RX_NEGATIVE']

    @property
    def RX_POSITIVE(self):
        """Message constant 'RX_POSITIVE'."""
        return Metaclass_MoveJog.__constants['RX_POSITIVE']

    @property
    def RY_NEGATIVE(self):
        """Message constant 'RY_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['RY_NEGATIVE']

    @property
    def RY_POSITIVE(self):
        """Message constant 'RY_POSITIVE'."""
        return Metaclass_MoveJog.__constants['RY_POSITIVE']

    @property
    def RZ_NEGATIVE(self):
        """Message constant 'RZ_NEGATIVE'."""
        return Metaclass_MoveJog.__constants['RZ_NEGATIVE']

    @property
    def RZ_POSITIVE(self):
        """Message constant 'RZ_POSITIVE'."""
        return Metaclass_MoveJog.__constants['RZ_POSITIVE']

    @property
    def STOP(self):
        """Message constant 'STOP'."""
        return Metaclass_MoveJog.__constants['STOP']


class MoveJog(metaclass=Metaclass_MoveJog):
    """
    Message class 'MoveJog'.

    Constants:
      J1_NEGATIVE
      J1_POSITIVE
      J2_NEGATIVE
      J2_POSITIVE
      J3_NEGATIVE
      J3_POSITIVE
      J4_NEGATIVE
      J4_POSITIVE
      X_NEGATIVE
      X_POSITIVE
      Y_NEGATIVE
      Y_POSITIVE
      Z_NEGATIVE
      Z_POSITIVE
      RX_NEGATIVE
      RX_POSITIVE
      RY_NEGATIVE
      RY_POSITIVE
      RZ_NEGATIVE
      RZ_POSITIVE
      STOP
    """

    __slots__ = [
        '_jog_mode',
    ]

    _fields_and_field_types = {
        'jog_mode': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.jog_mode = kwargs.get('jog_mode', str())

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
        if self.jog_mode != other.jog_mode:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def jog_mode(self):
        """Message field 'jog_mode'."""
        return self._jog_mode

    @jog_mode.setter
    def jog_mode(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'jog_mode' field must be of type 'str'"
        self._jog_mode = value
