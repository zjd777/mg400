# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:msg/MovJIO.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_MovJIO(type):
    """Metaclass of message 'MovJIO'."""

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
                'mg400_msgs.msg.MovJIO')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__mov_jio
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__mov_jio
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__mov_jio
            cls._TYPE_SUPPORT = module.type_support_msg__msg__mov_jio
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__mov_jio

            from geometry_msgs.msg import PoseStamped
            if PoseStamped.__class__._TYPE_SUPPORT is None:
                PoseStamped.__class__.__import_type_support__()

            from mg400_msgs.msg import DOIndex
            if DOIndex.__class__._TYPE_SUPPORT is None:
                DOIndex.__class__.__import_type_support__()

            from mg400_msgs.msg import DOStatus
            if DOStatus.__class__._TYPE_SUPPORT is None:
                DOStatus.__class__.__import_type_support__()

            from mg400_msgs.msg import DistanceMode
            if DistanceMode.__class__._TYPE_SUPPORT is None:
                DistanceMode.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO(metaclass=Metaclass_MovJIO):
    """Message class 'MovJIO'."""

    __slots__ = [
        '_pose',
        '_mode',
        '_distance',
        '_index',
        '_status',
        '_set_speed_j',
        '_speed_j',
        '_set_acc_j',
        '_acc_j',
    ]

    _fields_and_field_types = {
        'pose': 'geometry_msgs/PoseStamped',
        'mode': 'mg400_msgs/DistanceMode',
        'distance': 'int32',
        'index': 'mg400_msgs/DOIndex',
        'status': 'mg400_msgs/DOStatus',
        'set_speed_j': 'boolean',
        'speed_j': 'uint8',
        'set_acc_j': 'boolean',
        'acc_j': 'uint8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'PoseStamped'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'DistanceMode'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'DOIndex'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'DOStatus'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from geometry_msgs.msg import PoseStamped
        self.pose = kwargs.get('pose', PoseStamped())
        from mg400_msgs.msg import DistanceMode
        self.mode = kwargs.get('mode', DistanceMode())
        self.distance = kwargs.get('distance', int())
        from mg400_msgs.msg import DOIndex
        self.index = kwargs.get('index', DOIndex())
        from mg400_msgs.msg import DOStatus
        self.status = kwargs.get('status', DOStatus())
        self.set_speed_j = kwargs.get('set_speed_j', bool())
        self.speed_j = kwargs.get('speed_j', int())
        self.set_acc_j = kwargs.get('set_acc_j', bool())
        self.acc_j = kwargs.get('acc_j', int())

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
        if self.pose != other.pose:
            return False
        if self.mode != other.mode:
            return False
        if self.distance != other.distance:
            return False
        if self.index != other.index:
            return False
        if self.status != other.status:
            return False
        if self.set_speed_j != other.set_speed_j:
            return False
        if self.speed_j != other.speed_j:
            return False
        if self.set_acc_j != other.set_acc_j:
            return False
        if self.acc_j != other.acc_j:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def pose(self):
        """Message field 'pose'."""
        return self._pose

    @pose.setter
    def pose(self, value):
        if __debug__:
            from geometry_msgs.msg import PoseStamped
            assert \
                isinstance(value, PoseStamped), \
                "The 'pose' field must be a sub message of type 'PoseStamped'"
        self._pose = value

    @builtins.property
    def mode(self):
        """Message field 'mode'."""
        return self._mode

    @mode.setter
    def mode(self, value):
        if __debug__:
            from mg400_msgs.msg import DistanceMode
            assert \
                isinstance(value, DistanceMode), \
                "The 'mode' field must be a sub message of type 'DistanceMode'"
        self._mode = value

    @builtins.property
    def distance(self):
        """Message field 'distance'."""
        return self._distance

    @distance.setter
    def distance(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'distance' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'distance' field must be an integer in [-2147483648, 2147483647]"
        self._distance = value

    @builtins.property
    def index(self):
        """Message field 'index'."""
        return self._index

    @index.setter
    def index(self, value):
        if __debug__:
            from mg400_msgs.msg import DOIndex
            assert \
                isinstance(value, DOIndex), \
                "The 'index' field must be a sub message of type 'DOIndex'"
        self._index = value

    @builtins.property
    def status(self):
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value):
        if __debug__:
            from mg400_msgs.msg import DOStatus
            assert \
                isinstance(value, DOStatus), \
                "The 'status' field must be a sub message of type 'DOStatus'"
        self._status = value

    @builtins.property
    def set_speed_j(self):
        """Message field 'set_speed_j'."""
        return self._set_speed_j

    @set_speed_j.setter
    def set_speed_j(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'set_speed_j' field must be of type 'bool'"
        self._set_speed_j = value

    @builtins.property
    def speed_j(self):
        """Message field 'speed_j'."""
        return self._speed_j

    @speed_j.setter
    def speed_j(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'speed_j' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'speed_j' field must be an unsigned integer in [0, 255]"
        self._speed_j = value

    @builtins.property
    def set_acc_j(self):
        """Message field 'set_acc_j'."""
        return self._set_acc_j

    @set_acc_j.setter
    def set_acc_j(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'set_acc_j' field must be of type 'bool'"
        self._set_acc_j = value

    @builtins.property
    def acc_j(self):
        """Message field 'acc_j'."""
        return self._acc_j

    @acc_j.setter
    def acc_j(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'acc_j' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'acc_j' field must be an unsigned integer in [0, 255]"
        self._acc_j = value
