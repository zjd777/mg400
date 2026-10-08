# generated from rosidl_generator_py/resource/_idl.py.em
# with input from mg400_msgs:action/MovJIO.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_MovJIO_Goal(type):
    """Metaclass of message 'MovJIO_Goal'."""

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
                'mg400_msgs.action.MovJIO_Goal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__goal
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__goal
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__goal
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__goal
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__goal

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


class MovJIO_Goal(metaclass=Metaclass_MovJIO_Goal):
    """Message class 'MovJIO_Goal'."""

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
        '_set_cp',
        '_cp',
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
        'set_cp': 'boolean',
        'cp': 'uint8',
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
        self.set_cp = kwargs.get('set_cp', bool())
        self.cp = kwargs.get('cp', int())

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
        if self.set_cp != other.set_cp:
            return False
        if self.cp != other.cp:
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

    @builtins.property
    def set_cp(self):
        """Message field 'set_cp'."""
        return self._set_cp

    @set_cp.setter
    def set_cp(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'set_cp' field must be of type 'bool'"
        self._set_cp = value

    @builtins.property
    def cp(self):
        """Message field 'cp'."""
        return self._cp

    @cp.setter
    def cp(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'cp' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'cp' field must be an unsigned integer in [0, 255]"
        self._cp = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_Result(type):
    """Metaclass of message 'MovJIO_Result'."""

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
                'mg400_msgs.action.MovJIO_Result')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__result
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__result
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__result
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__result
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__result

            from mg400_msgs.msg import ErrorID
            if ErrorID.__class__._TYPE_SUPPORT is None:
                ErrorID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_Result(metaclass=Metaclass_MovJIO_Result):
    """Message class 'MovJIO_Result'."""

    __slots__ = [
        '_result',
        '_error_id',
    ]

    _fields_and_field_types = {
        'result': 'boolean',
        'error_id': 'mg400_msgs/ErrorID',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'msg'], 'ErrorID'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.result = kwargs.get('result', bool())
        from mg400_msgs.msg import ErrorID
        self.error_id = kwargs.get('error_id', ErrorID())

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
            from mg400_msgs.msg import ErrorID
            assert \
                isinstance(value, ErrorID), \
                "The 'error_id' field must be a sub message of type 'ErrorID'"
        self._error_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_Feedback(type):
    """Metaclass of message 'MovJIO_Feedback'."""

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
                'mg400_msgs.action.MovJIO_Feedback')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__feedback
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__feedback
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__feedback
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__feedback
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__feedback

            from geometry_msgs.msg import PoseStamped
            if PoseStamped.__class__._TYPE_SUPPORT is None:
                PoseStamped.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_Feedback(metaclass=Metaclass_MovJIO_Feedback):
    """Message class 'MovJIO_Feedback'."""

    __slots__ = [
        '_current_pose',
    ]

    _fields_and_field_types = {
        'current_pose': 'geometry_msgs/PoseStamped',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'PoseStamped'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from geometry_msgs.msg import PoseStamped
        self.current_pose = kwargs.get('current_pose', PoseStamped())

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
        if self.current_pose != other.current_pose:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def current_pose(self):
        """Message field 'current_pose'."""
        return self._current_pose

    @current_pose.setter
    def current_pose(self, value):
        if __debug__:
            from geometry_msgs.msg import PoseStamped
            assert \
                isinstance(value, PoseStamped), \
                "The 'current_pose' field must be a sub message of type 'PoseStamped'"
        self._current_pose = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_SendGoal_Request(type):
    """Metaclass of message 'MovJIO_SendGoal_Request'."""

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
                'mg400_msgs.action.MovJIO_SendGoal_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__send_goal__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__send_goal__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__send_goal__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__send_goal__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__send_goal__request

            from mg400_msgs.action import MovJIO
            if MovJIO.Goal.__class__._TYPE_SUPPORT is None:
                MovJIO.Goal.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_SendGoal_Request(metaclass=Metaclass_MovJIO_SendGoal_Request):
    """Message class 'MovJIO_SendGoal_Request'."""

    __slots__ = [
        '_goal_id',
        '_goal',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'goal': 'mg400_msgs/MovJIO_Goal',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'action'], 'MovJIO_Goal'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from mg400_msgs.action._mov_jio import MovJIO_Goal
        self.goal = kwargs.get('goal', MovJIO_Goal())

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
        if self.goal_id != other.goal_id:
            return False
        if self.goal != other.goal:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def goal(self):
        """Message field 'goal'."""
        return self._goal

    @goal.setter
    def goal(self, value):
        if __debug__:
            from mg400_msgs.action._mov_jio import MovJIO_Goal
            assert \
                isinstance(value, MovJIO_Goal), \
                "The 'goal' field must be a sub message of type 'MovJIO_Goal'"
        self._goal = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_SendGoal_Response(type):
    """Metaclass of message 'MovJIO_SendGoal_Response'."""

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
                'mg400_msgs.action.MovJIO_SendGoal_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__send_goal__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__send_goal__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__send_goal__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__send_goal__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__send_goal__response

            from builtin_interfaces.msg import Time
            if Time.__class__._TYPE_SUPPORT is None:
                Time.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_SendGoal_Response(metaclass=Metaclass_MovJIO_SendGoal_Response):
    """Message class 'MovJIO_SendGoal_Response'."""

    __slots__ = [
        '_accepted',
        '_stamp',
    ]

    _fields_and_field_types = {
        'accepted': 'boolean',
        'stamp': 'builtin_interfaces/Time',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.accepted = kwargs.get('accepted', bool())
        from builtin_interfaces.msg import Time
        self.stamp = kwargs.get('stamp', Time())

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
        if self.accepted != other.accepted:
            return False
        if self.stamp != other.stamp:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def accepted(self):
        """Message field 'accepted'."""
        return self._accepted

    @accepted.setter
    def accepted(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'accepted' field must be of type 'bool'"
        self._accepted = value

    @builtins.property
    def stamp(self):
        """Message field 'stamp'."""
        return self._stamp

    @stamp.setter
    def stamp(self, value):
        if __debug__:
            from builtin_interfaces.msg import Time
            assert \
                isinstance(value, Time), \
                "The 'stamp' field must be a sub message of type 'Time'"
        self._stamp = value


class Metaclass_MovJIO_SendGoal(type):
    """Metaclass of service 'MovJIO_SendGoal'."""

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
                'mg400_msgs.action.MovJIO_SendGoal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__mov_jio__send_goal

            from mg400_msgs.action import _mov_jio
            if _mov_jio.Metaclass_MovJIO_SendGoal_Request._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_SendGoal_Request.__import_type_support__()
            if _mov_jio.Metaclass_MovJIO_SendGoal_Response._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_SendGoal_Response.__import_type_support__()


class MovJIO_SendGoal(metaclass=Metaclass_MovJIO_SendGoal):
    from mg400_msgs.action._mov_jio import MovJIO_SendGoal_Request as Request
    from mg400_msgs.action._mov_jio import MovJIO_SendGoal_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_GetResult_Request(type):
    """Metaclass of message 'MovJIO_GetResult_Request'."""

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
                'mg400_msgs.action.MovJIO_GetResult_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__get_result__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__get_result__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__get_result__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__get_result__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__get_result__request

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_GetResult_Request(metaclass=Metaclass_MovJIO_GetResult_Request):
    """Message class 'MovJIO_GetResult_Request'."""

    __slots__ = [
        '_goal_id',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())

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
        if self.goal_id != other.goal_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_GetResult_Response(type):
    """Metaclass of message 'MovJIO_GetResult_Response'."""

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
                'mg400_msgs.action.MovJIO_GetResult_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__get_result__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__get_result__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__get_result__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__get_result__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__get_result__response

            from mg400_msgs.action import MovJIO
            if MovJIO.Result.__class__._TYPE_SUPPORT is None:
                MovJIO.Result.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_GetResult_Response(metaclass=Metaclass_MovJIO_GetResult_Response):
    """Message class 'MovJIO_GetResult_Response'."""

    __slots__ = [
        '_status',
        '_result',
    ]

    _fields_and_field_types = {
        'status': 'int8',
        'result': 'mg400_msgs/MovJIO_Result',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'action'], 'MovJIO_Result'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.status = kwargs.get('status', int())
        from mg400_msgs.action._mov_jio import MovJIO_Result
        self.result = kwargs.get('result', MovJIO_Result())

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
        if self.status != other.status:
            return False
        if self.result != other.result:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def status(self):
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'status' field must be of type 'int'"
            assert value >= -128 and value < 128, \
                "The 'status' field must be an integer in [-128, 127]"
        self._status = value

    @builtins.property
    def result(self):
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value):
        if __debug__:
            from mg400_msgs.action._mov_jio import MovJIO_Result
            assert \
                isinstance(value, MovJIO_Result), \
                "The 'result' field must be a sub message of type 'MovJIO_Result'"
        self._result = value


class Metaclass_MovJIO_GetResult(type):
    """Metaclass of service 'MovJIO_GetResult'."""

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
                'mg400_msgs.action.MovJIO_GetResult')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__mov_jio__get_result

            from mg400_msgs.action import _mov_jio
            if _mov_jio.Metaclass_MovJIO_GetResult_Request._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_GetResult_Request.__import_type_support__()
            if _mov_jio.Metaclass_MovJIO_GetResult_Response._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_GetResult_Response.__import_type_support__()


class MovJIO_GetResult(metaclass=Metaclass_MovJIO_GetResult):
    from mg400_msgs.action._mov_jio import MovJIO_GetResult_Request as Request
    from mg400_msgs.action._mov_jio import MovJIO_GetResult_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_MovJIO_FeedbackMessage(type):
    """Metaclass of message 'MovJIO_FeedbackMessage'."""

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
                'mg400_msgs.action.MovJIO_FeedbackMessage')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__mov_jio__feedback_message
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__mov_jio__feedback_message
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__mov_jio__feedback_message
            cls._TYPE_SUPPORT = module.type_support_msg__action__mov_jio__feedback_message
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__mov_jio__feedback_message

            from mg400_msgs.action import MovJIO
            if MovJIO.Feedback.__class__._TYPE_SUPPORT is None:
                MovJIO.Feedback.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class MovJIO_FeedbackMessage(metaclass=Metaclass_MovJIO_FeedbackMessage):
    """Message class 'MovJIO_FeedbackMessage'."""

    __slots__ = [
        '_goal_id',
        '_feedback',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'feedback': 'mg400_msgs/MovJIO_Feedback',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['mg400_msgs', 'action'], 'MovJIO_Feedback'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from mg400_msgs.action._mov_jio import MovJIO_Feedback
        self.feedback = kwargs.get('feedback', MovJIO_Feedback())

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
        if self.goal_id != other.goal_id:
            return False
        if self.feedback != other.feedback:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def feedback(self):
        """Message field 'feedback'."""
        return self._feedback

    @feedback.setter
    def feedback(self, value):
        if __debug__:
            from mg400_msgs.action._mov_jio import MovJIO_Feedback
            assert \
                isinstance(value, MovJIO_Feedback), \
                "The 'feedback' field must be a sub message of type 'MovJIO_Feedback'"
        self._feedback = value


class Metaclass_MovJIO(type):
    """Metaclass of action 'MovJIO'."""

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
                'mg400_msgs.action.MovJIO')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_action__action__mov_jio

            from action_msgs.msg import _goal_status_array
            if _goal_status_array.Metaclass_GoalStatusArray._TYPE_SUPPORT is None:
                _goal_status_array.Metaclass_GoalStatusArray.__import_type_support__()
            from action_msgs.srv import _cancel_goal
            if _cancel_goal.Metaclass_CancelGoal._TYPE_SUPPORT is None:
                _cancel_goal.Metaclass_CancelGoal.__import_type_support__()

            from mg400_msgs.action import _mov_jio
            if _mov_jio.Metaclass_MovJIO_SendGoal._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_SendGoal.__import_type_support__()
            if _mov_jio.Metaclass_MovJIO_GetResult._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_GetResult.__import_type_support__()
            if _mov_jio.Metaclass_MovJIO_FeedbackMessage._TYPE_SUPPORT is None:
                _mov_jio.Metaclass_MovJIO_FeedbackMessage.__import_type_support__()


class MovJIO(metaclass=Metaclass_MovJIO):

    # The goal message defined in the action definition.
    from mg400_msgs.action._mov_jio import MovJIO_Goal as Goal
    # The result message defined in the action definition.
    from mg400_msgs.action._mov_jio import MovJIO_Result as Result
    # The feedback message defined in the action definition.
    from mg400_msgs.action._mov_jio import MovJIO_Feedback as Feedback

    class Impl:

        # The send_goal service using a wrapped version of the goal message as a request.
        from mg400_msgs.action._mov_jio import MovJIO_SendGoal as SendGoalService
        # The get_result service using a wrapped version of the result message as a response.
        from mg400_msgs.action._mov_jio import MovJIO_GetResult as GetResultService
        # The feedback message with generic fields which wraps the feedback message.
        from mg400_msgs.action._mov_jio import MovJIO_FeedbackMessage as FeedbackMessage

        # The generic service to cancel a goal.
        from action_msgs.srv._cancel_goal import CancelGoal as CancelGoalService
        # The generic message for get the status of a goal.
        from action_msgs.msg._goal_status_array import GoalStatusArray as GoalStatusMessage

    def __init__(self):
        raise NotImplementedError('Action classes can not be instantiated')
