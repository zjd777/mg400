// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:action/CommandQueue.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__STRUCT_HPP_
#define MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'commands'
#include "mg400_msgs/msg/detail/command__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_Goal __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_Goal __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_Goal_
{
  using Type = CommandQueue_Goal_<ContainerAllocator>;

  explicit CommandQueue_Goal_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
  }

  explicit CommandQueue_Goal_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
    (void)_alloc;
  }

  // field types and members
  using _commands_type =
    std::vector<mg400_msgs::msg::Command_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<mg400_msgs::msg::Command_<ContainerAllocator>>>;
  _commands_type commands;

  // setters for named parameter idiom
  Type & set__commands(
    const std::vector<mg400_msgs::msg::Command_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<mg400_msgs::msg::Command_<ContainerAllocator>>> & _arg)
  {
    this->commands = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_Goal
    std::shared_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_Goal
    std::shared_ptr<mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_Goal_ & other) const
  {
    if (this->commands != other.commands) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_Goal_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_Goal_

// alias to use template instance with default allocator
using CommandQueue_Goal =
  mg400_msgs::action::CommandQueue_Goal_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'error_id'
#include "mg400_msgs/msg/detail/error_id__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_Result __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_Result __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_Result_
{
  using Type = CommandQueue_Result_<ContainerAllocator>;

  explicit CommandQueue_Result_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : error_id(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
    }
  }

  explicit CommandQueue_Result_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : error_id(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
    }
  }

  // field types and members
  using _result_type =
    bool;
  _result_type result;
  using _error_id_type =
    mg400_msgs::msg::ErrorID_<ContainerAllocator>;
  _error_id_type error_id;

  // setters for named parameter idiom
  Type & set__result(
    const bool & _arg)
  {
    this->result = _arg;
    return *this;
  }
  Type & set__error_id(
    const mg400_msgs::msg::ErrorID_<ContainerAllocator> & _arg)
  {
    this->error_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_Result
    std::shared_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_Result
    std::shared_ptr<mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_Result_ & other) const
  {
    if (this->result != other.result) {
      return false;
    }
    if (this->error_id != other.error_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_Result_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_Result_

// alias to use template instance with default allocator
using CommandQueue_Result =
  mg400_msgs::action::CommandQueue_Result_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'current_pose'
#include "geometry_msgs/msg/detail/pose_stamped__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_Feedback __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_Feedback __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_Feedback_
{
  using Type = CommandQueue_Feedback_<ContainerAllocator>;

  explicit CommandQueue_Feedback_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : current_pose(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      std::fill<typename std::array<double, 4>::iterator, double>(this->current_angles.begin(), this->current_angles.end(), 0.0);
    }
  }

  explicit CommandQueue_Feedback_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : current_pose(_alloc, _init),
    current_angles(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      std::fill<typename std::array<double, 4>::iterator, double>(this->current_angles.begin(), this->current_angles.end(), 0.0);
    }
  }

  // field types and members
  using _current_pose_type =
    geometry_msgs::msg::PoseStamped_<ContainerAllocator>;
  _current_pose_type current_pose;
  using _current_angles_type =
    std::array<double, 4>;
  _current_angles_type current_angles;

  // setters for named parameter idiom
  Type & set__current_pose(
    const geometry_msgs::msg::PoseStamped_<ContainerAllocator> & _arg)
  {
    this->current_pose = _arg;
    return *this;
  }
  Type & set__current_angles(
    const std::array<double, 4> & _arg)
  {
    this->current_angles = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_Feedback
    std::shared_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_Feedback
    std::shared_ptr<mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_Feedback_ & other) const
  {
    if (this->current_pose != other.current_pose) {
      return false;
    }
    if (this->current_angles != other.current_angles) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_Feedback_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_Feedback_

// alias to use template instance with default allocator
using CommandQueue_Feedback =
  mg400_msgs::action::CommandQueue_Feedback_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"
// Member 'goal'
#include "mg400_msgs/action/detail/command_queue__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_SendGoal_Request_
{
  using Type = CommandQueue_SendGoal_Request_<ContainerAllocator>;

  explicit CommandQueue_SendGoal_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init),
    goal(_init)
  {
    (void)_init;
  }

  explicit CommandQueue_SendGoal_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_alloc, _init),
    goal(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _goal_id_type =
    unique_identifier_msgs::msg::UUID_<ContainerAllocator>;
  _goal_id_type goal_id;
  using _goal_type =
    mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator>;
  _goal_type goal;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }
  Type & set__goal(
    const mg400_msgs::action::CommandQueue_Goal_<ContainerAllocator> & _arg)
  {
    this->goal = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Request
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Request
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_SendGoal_Request_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    if (this->goal != other.goal) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_SendGoal_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_SendGoal_Request_

// alias to use template instance with default allocator
using CommandQueue_SendGoal_Request =
  mg400_msgs::action::CommandQueue_SendGoal_Request_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_SendGoal_Response_
{
  using Type = CommandQueue_SendGoal_Response_<ContainerAllocator>;

  explicit CommandQueue_SendGoal_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stamp(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->accepted = false;
    }
  }

  explicit CommandQueue_SendGoal_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stamp(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->accepted = false;
    }
  }

  // field types and members
  using _accepted_type =
    bool;
  _accepted_type accepted;
  using _stamp_type =
    builtin_interfaces::msg::Time_<ContainerAllocator>;
  _stamp_type stamp;

  // setters for named parameter idiom
  Type & set__accepted(
    const bool & _arg)
  {
    this->accepted = _arg;
    return *this;
  }
  Type & set__stamp(
    const builtin_interfaces::msg::Time_<ContainerAllocator> & _arg)
  {
    this->stamp = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Response
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_SendGoal_Response
    std::shared_ptr<mg400_msgs::action::CommandQueue_SendGoal_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_SendGoal_Response_ & other) const
  {
    if (this->accepted != other.accepted) {
      return false;
    }
    if (this->stamp != other.stamp) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_SendGoal_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_SendGoal_Response_

// alias to use template instance with default allocator
using CommandQueue_SendGoal_Response =
  mg400_msgs::action::CommandQueue_SendGoal_Response_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace action
{

struct CommandQueue_SendGoal
{
  using Request = mg400_msgs::action::CommandQueue_SendGoal_Request;
  using Response = mg400_msgs::action::CommandQueue_SendGoal_Response;
};

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_GetResult_Request_
{
  using Type = CommandQueue_GetResult_Request_<ContainerAllocator>;

  explicit CommandQueue_GetResult_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init)
  {
    (void)_init;
  }

  explicit CommandQueue_GetResult_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _goal_id_type =
    unique_identifier_msgs::msg::UUID_<ContainerAllocator>;
  _goal_id_type goal_id;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Request
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Request
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_GetResult_Request_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_GetResult_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_GetResult_Request_

// alias to use template instance with default allocator
using CommandQueue_GetResult_Request =
  mg400_msgs::action::CommandQueue_GetResult_Request_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'result'
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_GetResult_Response_
{
  using Type = CommandQueue_GetResult_Response_<ContainerAllocator>;

  explicit CommandQueue_GetResult_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : result(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  explicit CommandQueue_GetResult_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : result(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  // field types and members
  using _status_type =
    int8_t;
  _status_type status;
  using _result_type =
    mg400_msgs::action::CommandQueue_Result_<ContainerAllocator>;
  _result_type result;

  // setters for named parameter idiom
  Type & set__status(
    const int8_t & _arg)
  {
    this->status = _arg;
    return *this;
  }
  Type & set__result(
    const mg400_msgs::action::CommandQueue_Result_<ContainerAllocator> & _arg)
  {
    this->result = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Response
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_GetResult_Response
    std::shared_ptr<mg400_msgs::action::CommandQueue_GetResult_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_GetResult_Response_ & other) const
  {
    if (this->status != other.status) {
      return false;
    }
    if (this->result != other.result) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_GetResult_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_GetResult_Response_

// alias to use template instance with default allocator
using CommandQueue_GetResult_Response =
  mg400_msgs::action::CommandQueue_GetResult_Response_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace action
{

struct CommandQueue_GetResult
{
  using Request = mg400_msgs::action::CommandQueue_GetResult_Request;
  using Response = mg400_msgs::action::CommandQueue_GetResult_Response;
};

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"
// Member 'feedback'
// already included above
// #include "mg400_msgs/action/detail/command_queue__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__CommandQueue_FeedbackMessage __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__CommandQueue_FeedbackMessage __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct CommandQueue_FeedbackMessage_
{
  using Type = CommandQueue_FeedbackMessage_<ContainerAllocator>;

  explicit CommandQueue_FeedbackMessage_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init),
    feedback(_init)
  {
    (void)_init;
  }

  explicit CommandQueue_FeedbackMessage_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_alloc, _init),
    feedback(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _goal_id_type =
    unique_identifier_msgs::msg::UUID_<ContainerAllocator>;
  _goal_id_type goal_id;
  using _feedback_type =
    mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator>;
  _feedback_type feedback;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }
  Type & set__feedback(
    const mg400_msgs::action::CommandQueue_Feedback_<ContainerAllocator> & _arg)
  {
    this->feedback = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_FeedbackMessage
    std::shared_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__CommandQueue_FeedbackMessage
    std::shared_ptr<mg400_msgs::action::CommandQueue_FeedbackMessage_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CommandQueue_FeedbackMessage_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    if (this->feedback != other.feedback) {
      return false;
    }
    return true;
  }
  bool operator!=(const CommandQueue_FeedbackMessage_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CommandQueue_FeedbackMessage_

// alias to use template instance with default allocator
using CommandQueue_FeedbackMessage =
  mg400_msgs::action::CommandQueue_FeedbackMessage_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs

#include "action_msgs/srv/cancel_goal.hpp"
#include "action_msgs/msg/goal_info.hpp"
#include "action_msgs/msg/goal_status_array.hpp"

namespace mg400_msgs
{

namespace action
{

struct CommandQueue
{
  /// The goal message defined in the action definition.
  using Goal = mg400_msgs::action::CommandQueue_Goal;
  /// The result message defined in the action definition.
  using Result = mg400_msgs::action::CommandQueue_Result;
  /// The feedback message defined in the action definition.
  using Feedback = mg400_msgs::action::CommandQueue_Feedback;

  struct Impl
  {
    /// The send_goal service using a wrapped version of the goal message as a request.
    using SendGoalService = mg400_msgs::action::CommandQueue_SendGoal;
    /// The get_result service using a wrapped version of the result message as a response.
    using GetResultService = mg400_msgs::action::CommandQueue_GetResult;
    /// The feedback message with generic fields which wraps the feedback message.
    using FeedbackMessage = mg400_msgs::action::CommandQueue_FeedbackMessage;

    /// The generic service to cancel a goal.
    using CancelGoalService = action_msgs::srv::CancelGoal;
    /// The generic message for the status of a goal.
    using GoalStatusMessage = action_msgs::msg::GoalStatusArray;
  };
};

typedef struct CommandQueue CommandQueue;

}  // namespace action

}  // namespace mg400_msgs

#endif  // MG400_MSGS__ACTION__DETAIL__COMMAND_QUEUE__STRUCT_HPP_
