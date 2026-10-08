# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/Command.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_Command(type):
    """Metaclass of message 'Command'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'CT_MOV_J': 1,
        'CT_MOV_L': 2,
        'CT_JOINT_MOV_J': 3,
        'CT_MOV_JIO': 4,
        'CT_MOV_LIO': 5,
        'CT_TOOL_DO': 25,
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
                'mg400_msgs.msg.Command')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__command
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__command
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__command
            cls._TYPE_SUPPORT = module.type_support_msg__msg__command
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__command

            from mg400_msgs.msg import JointMovJ
            if JointMovJ.__class__._TYPE_SUPPORT is None:
                JointMovJ.__class__.__import_type_support__()

            from mg400_msgs.msg import MovJ
            if MovJ.__class__._TYPE_SUPPORT is None:
                MovJ.__class__.__import_type_support__()

            from mg400_msgs.msg import MovJIO
            if MovJIO.__class__._TYPE_SUPPORT is None:
                MovJIO.__class__.__import_type_support__()

            from mg400_msgs.msg import MovL
            if MovL.__class__._TYPE_SUPPORT is None:
                MovL.__class__.__import_type_support__()

            from mg400_msgs.msg import MovLIO
            if MovLIO.__class__._TYPE_SUPPORT is None:
                MovLIO.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'CT_MOV_J': cls.__constants['CT_MOV_J'],
            'CT_MOV_L': cls.__constants['CT_MOV_L'],
            'CT_JOINT_MOV_J': cls.__constants['CT_JOINT_MOV_J'],
            'CT_MOV_JIO': cls.__constants['CT_MOV_JIO'],
            'CT_MOV_LIO': cls.__constants['CT_MOV_LIO'],
            'CT_TOOL_DO': cls.__constants['CT_TOOL_DO'],
        }

    @property
    def CT_MOV_J(self):
        """Message constant 'CT_MOV_J'."""
        return Metaclass_Command.__constants['CT_MOV_J']

    @property
    def CT_MOV_L(self):
        """Message constant 'CT_MOV_L'."""
        return Metaclass_Command.__constants['CT_MOV_L']

    @property
    def CT_JOINT_MOV_J(self):
        """Message constant 'CT_JOINT_MOV_J'."""
        return Metaclass_Command.__constants['CT_JOINT_MOV_J']

    @property
    def CT_MOV_JIO(self):
        """Message constant 'CT_MOV_JIO'."""
        return Metaclass_Command.__constants['CT_MOV_JIO']

    @property
    def CT_MOV_LIO(self):
        """Message constant 'CT_MOV_LIO'."""
        return Metaclass_Command.__constants['CT_MOV_LIO']

    @property
    def CT_TOOL_DO(self):
        """Message constant 'CT_TOOL_DO'."""
        return Metaclass_Command.__constants['CT_TOOL_DO']


class Command(metaclass=Metaclass_Command):
    """
    Message class 'Command'.

    Constants:
      CT_MOV_J
      CT_MOV_L
      CT_JOINT_MOV_J
      CT_MOV_JIO
      CT_MOV_LIO
      CT_TOOL_DO
    """

    __slots__ = [
        '_command_type',
        '_mov_j_params',
        '_mov_l_params',
        '_joint_mov_j_params',
        '_mov_jio_params',
        '_mov_lio_params',
    ]

    _fields_and_field_types = {
        'command_type': 'uint16',
        'mov_j_params': 'mg400_msgs/MovJ',
        'mov_l_params': 'mg400_msgs/MovL',
        'joint_mov_j_params': 'mg400_msgs/JointMovJ',
        'mov_jio_params': 'mg400_msgs/MovJIO',
        'mov_lio_params': 'mg400_msgs/MovLIO',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint16'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'MovJ'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'MovL'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'JointMovJ'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'MovJIO'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'MovLIO'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.command_type = kwargs.get('command_type', int())
        from mg400_msgs.msg import MovJ
        self.mov_j_params = kwargs.get('mov_j_params', MovJ())
        from mg400_msgs.msg import MovL
        self.mov_l_params = kwargs.get('mov_l_params', MovL())
        from mg400_msgs.msg import JointMovJ
        self.joint_mov_j_params = kwargs.get('joint_mov_j_params', JointMovJ())
        from mg400_msgs.msg import MovJIO
        self.mov_jio_params = kwargs.get('mov_jio_params', MovJIO())
        from mg400_msgs.msg import MovLIO
        self.mov_lio_params = kwargs.get('mov_lio_params', MovLIO())

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
        if self.command_type != other.command_type:
            return False
        if self.mov_j_params != other.mov_j_params:
            return False
        if self.mov_l_params != other.mov_l_params:
            return False
        if self.joint_mov_j_params != other.joint_mov_j_params:
            return False
        if self.mov_jio_params != other.mov_jio_params:
            return False
        if self.mov_lio_params != other.mov_lio_params:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def command_type(self):
        """Message field 'command_type'."""
        return self._command_type

    @command_type.setter
    def command_type(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'command_type' field must be of type 'int'"
            assert value >= 0 and value < 65536, \
                "The 'command_type' field must be an unsigned integer in [0, 65535]"
        self._command_type = value

    @builtins.property
    def mov_j_params(self):
        """Message field 'mov_j_params'."""
        return self._mov_j_params

    @mov_j_params.setter
    def mov_j_params(self, value):
        if __debug__:
            from mg400_msgs.msg import MovJ
            assert \
                isinstance(value, MovJ), \
                "The 'mov_j_params' field must be a sub message of type 'MovJ'"
        self._mov_j_params = value

    @builtins.property
    def mov_l_params(self):
        """Message field 'mov_l_params'."""
        return self._mov_l_params

    @mov_l_params.setter
    def mov_l_params(self, value):
        if __debug__:
            from mg400_msgs.msg import MovL
            assert \
                isinstance(value, MovL), \
                "The 'mov_l_params' field must be a sub message of type 'MovL'"
        self._mov_l_params = value

    @builtins.property
    def joint_mov_j_params(self):
        """Message field 'joint_mov_j_params'."""
        return self._joint_mov_j_params

    @joint_mov_j_params.setter
    def joint_mov_j_params(self, value):
        if __debug__:
            from mg400_msgs.msg import JointMovJ
            assert \
                isinstance(value, JointMovJ), \
                "The 'joint_mov_j_params' field must be a sub message of type 'JointMovJ'"
        self._joint_mov_j_params = value

    @builtins.property
    def mov_jio_params(self):
        """Message field 'mov_jio_params'."""
        return self._mov_jio_params

    @mov_jio_params.setter
    def mov_jio_params(self, value):
        if __debug__:
            from mg400_msgs.msg import MovJIO
            assert \
                isinstance(value, MovJIO), \
                "The 'mov_jio_params' field must be a sub message of type 'MovJIO'"
        self._mov_jio_params = value

    @builtins.property
    def mov_lio_params(self):
        """Message field 'mov_lio_params'."""
        return self._mov_lio_params

    @mov_lio_params.setter
    def mov_lio_params(self, value):
        if __debug__:
            from mg400_msgs.msg import MovLIO
            assert \
                isinstance(value, MovLIO), \
                "The 'mov_lio_params' field must be a sub message of type 'MovLIO'"
        self._mov_lio_params = value
