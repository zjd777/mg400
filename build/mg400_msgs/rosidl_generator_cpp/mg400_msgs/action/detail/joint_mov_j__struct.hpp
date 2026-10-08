// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:action/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__ACTION__DETAIL__JOINT_MOV_J__STRUCT_HPP_
#define MG400_MSGS__ACTION__DETAIL__JOINT_MOV_J__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_Goal __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_Goal __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_Goal_
{
  using Type = JointMovJ_Goal_<ContainerAllocator>;

  explicit JointMovJ_Goal_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      std::fill<typename std::array<double, 4>::iterator, double>(this->joint_angles.begin(), this->joint_angles.end(), 0.0);
      this->set_speed_j = false;
      this->speed_j = 0;
      this->set_acc_j = false;
      this->acc_j = 0;
      this->set_cp = false;
      this->cp = 0;
    }
  }

  explicit JointMovJ_Goal_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : joint_angles(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      std::fill<typename std::array<double, 4>::iterator, double>(this->joint_angles.begin(), this->joint_angles.end(), 0.0);
      this->set_speed_j = false;
      this->speed_j = 0;
      this->set_acc_j = false;
      this->acc_j = 0;
      this->set_cp = false;
      this->cp = 0;
    }
  }

  // field types and members
  using _joint_angles_type =
    std::array<double, 4>;
  _joint_angles_type joint_angles;
  using _set_speed_j_type =
    bool;
  _set_speed_j_type set_speed_j;
  using _speed_j_type =
    uint8_t;
  _speed_j_type speed_j;
  using _set_acc_j_type =
    bool;
  _set_acc_j_type set_acc_j;
  using _acc_j_type =
    uint8_t;
  _acc_j_type acc_j;
  using _set_cp_type =
    bool;
  _set_cp_type set_cp;
  using _cp_type =
    uint8_t;
  _cp_type cp;

  // setters for named parameter idiom
  Type & set__joint_angles(
    const std::array<double, 4> & _arg)
  {
    this->joint_angles = _arg;
    return *this;
  }
  Type & set__set_speed_j(
    const bool & _arg)
  {
    this->set_speed_j = _arg;
    return *this;
  }
  Type & set__speed_j(
    const uint8_t & _arg)
  {
    this->speed_j = _arg;
    return *this;
  }
  Type & set__set_acc_j(
    const bool & _arg)
  {
    this->set_acc_j = _arg;
    return *this;
  }
  Type & set__acc_j(
    const uint8_t & _arg)
  {
    this->acc_j = _arg;
    return *this;
  }
  Type & set__set_cp(
    const bool & _arg)
  {
    this->set_cp = _arg;
    return *this;
  }
  Type & set__cp(
    const uint8_t & _arg)
  {
    this->cp = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_Goal
    std::shared_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_Goal
    std::shared_ptr<mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_Goal_ & other) const
  {
    if (this->joint_angles != other.joint_angles) {
      return false;
    }
    if (this->set_speed_j != other.set_speed_j) {
      return false;
    }
    if (this->speed_j != other.speed_j) {
      return false;
    }
    if (this->set_acc_j != other.set_acc_j) {
      return false;
    }
    if (this->acc_j != other.acc_j) {
      return false;
    }
    if (this->set_cp != other.set_cp) {
      return false;
    }
    if (this->cp != other.cp) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_Goal_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_Goal_

// alias to use template instance with default allocator
using JointMovJ_Goal =
  mg400_msgs::action::JointMovJ_Goal_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'error_id'
#include "mg400_msgs/msg/detail/error_id__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_Result __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_Result __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_Result_
{
  using Type = JointMovJ_Result_<ContainerAllocator>;

  explicit JointMovJ_Result_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : error_id(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
    }
  }

  explicit JointMovJ_Result_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_Result
    std::shared_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_Result
    std::shared_ptr<mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_Result_ & other) const
  {
    if (this->result != other.result) {
      return false;
    }
    if (this->error_id != other.error_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_Result_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_Result_

// alias to use template instance with default allocator
using JointMovJ_Result =
  mg400_msgs::action::JointMovJ_Result_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_Feedback __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_Feedback __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_Feedback_
{
  using Type = JointMovJ_Feedback_<ContainerAllocator>;

  explicit JointMovJ_Feedback_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      std::fill<typename std::array<double, 4>::iterator, double>(this->current_angles.begin(), this->current_angles.end(), 0.0);
    }
  }

  explicit JointMovJ_Feedback_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : current_angles(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      std::fill<typename std::array<double, 4>::iterator, double>(this->current_angles.begin(), this->current_angles.end(), 0.0);
    }
  }

  // field types and members
  using _current_angles_type =
    std::array<double, 4>;
  _current_angles_type current_angles;

  // setters for named parameter idiom
  Type & set__current_angles(
    const std::array<double, 4> & _arg)
  {
    this->current_angles = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_Feedback
    std::shared_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_Feedback
    std::shared_ptr<mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_Feedback_ & other) const
  {
    if (this->current_angles != other.current_angles) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_Feedback_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_Feedback_

// alias to use template instance with default allocator
using JointMovJ_Feedback =
  mg400_msgs::action::JointMovJ_Feedback_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"
// Member 'goal'
#include "mg400_msgs/action/detail/joint_mov_j__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_SendGoal_Request_
{
  using Type = JointMovJ_SendGoal_Request_<ContainerAllocator>;

  explicit JointMovJ_SendGoal_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init),
    goal(_init)
  {
    (void)_init;
  }

  explicit JointMovJ_SendGoal_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator>;
  _goal_type goal;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }
  Type & set__goal(
    const mg400_msgs::action::JointMovJ_Goal_<ContainerAllocator> & _arg)
  {
    this->goal = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Request
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Request
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_SendGoal_Request_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    if (this->goal != other.goal) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_SendGoal_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_SendGoal_Request_

// alias to use template instance with default allocator
using JointMovJ_SendGoal_Request =
  mg400_msgs::action::JointMovJ_SendGoal_Request_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_SendGoal_Response_
{
  using Type = JointMovJ_SendGoal_Response_<ContainerAllocator>;

  explicit JointMovJ_SendGoal_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stamp(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->accepted = false;
    }
  }

  explicit JointMovJ_SendGoal_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Response
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_SendGoal_Response
    std::shared_ptr<mg400_msgs::action::JointMovJ_SendGoal_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_SendGoal_Response_ & other) const
  {
    if (this->accepted != other.accepted) {
      return false;
    }
    if (this->stamp != other.stamp) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_SendGoal_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_SendGoal_Response_

// alias to use template instance with default allocator
using JointMovJ_SendGoal_Response =
  mg400_msgs::action::JointMovJ_SendGoal_Response_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace action
{

struct JointMovJ_SendGoal
{
  using Request = mg400_msgs::action::JointMovJ_SendGoal_Request;
  using Response = mg400_msgs::action::JointMovJ_SendGoal_Response;
};

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_GetResult_Request_
{
  using Type = JointMovJ_GetResult_Request_<ContainerAllocator>;

  explicit JointMovJ_GetResult_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init)
  {
    (void)_init;
  }

  explicit JointMovJ_GetResult_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Request
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Request
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_GetResult_Request_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_GetResult_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_GetResult_Request_

// alias to use template instance with default allocator
using JointMovJ_GetResult_Request =
  mg400_msgs::action::JointMovJ_GetResult_Request_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'result'
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_GetResult_Response_
{
  using Type = JointMovJ_GetResult_Response_<ContainerAllocator>;

  explicit JointMovJ_GetResult_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : result(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  explicit JointMovJ_GetResult_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::action::JointMovJ_Result_<ContainerAllocator>;
  _result_type result;

  // setters for named parameter idiom
  Type & set__status(
    const int8_t & _arg)
  {
    this->status = _arg;
    return *this;
  }
  Type & set__result(
    const mg400_msgs::action::JointMovJ_Result_<ContainerAllocator> & _arg)
  {
    this->result = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Response
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_GetResult_Response
    std::shared_ptr<mg400_msgs::action::JointMovJ_GetResult_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_GetResult_Response_ & other) const
  {
    if (this->status != other.status) {
      return false;
    }
    if (this->result != other.result) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_GetResult_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_GetResult_Response_

// alias to use template instance with default allocator
using JointMovJ_GetResult_Response =
  mg400_msgs::action::JointMovJ_GetResult_Response_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace action
{

struct JointMovJ_GetResult
{
  using Request = mg400_msgs::action::JointMovJ_GetResult_Request;
  using Response = mg400_msgs::action::JointMovJ_GetResult_Response;
};

}  // namespace action

}  // namespace mg400_msgs


// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"
// Member 'feedback'
// already included above
// #include "mg400_msgs/action/detail/joint_mov_j__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__action__JointMovJ_FeedbackMessage __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__action__JointMovJ_FeedbackMessage __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_FeedbackMessage_
{
  using Type = JointMovJ_FeedbackMessage_<ContainerAllocator>;

  explicit JointMovJ_FeedbackMessage_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init),
    feedback(_init)
  {
    (void)_init;
  }

  explicit JointMovJ_FeedbackMessage_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator>;
  _feedback_type feedback;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }
  Type & set__feedback(
    const mg400_msgs::action::JointMovJ_Feedback_<ContainerAllocator> & _arg)
  {
    this->feedback = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_FeedbackMessage
    std::shared_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__action__JointMovJ_FeedbackMessage
    std::shared_ptr<mg400_msgs::action::JointMovJ_FeedbackMessage_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_FeedbackMessage_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    if (this->feedback != other.feedback) {
      return false;
    }
    return true;
  }
  bool operator!=(const JointMovJ_FeedbackMessage_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_FeedbackMessage_

// alias to use template instance with default allocator
using JointMovJ_FeedbackMessage =
  mg400_msgs::action::JointMovJ_FeedbackMessage_<std::allocator<void>>;

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

struct JointMovJ
{
  /// The goal message defined in the action definition.
  using Goal = mg400_msgs::action::JointMovJ_Goal;
  /// The result message defined in the action definition.
  using Result = mg400_msgs::action::JointMovJ_Result;
  /// The feedback message defined in the action definition.
  using Feedback = mg400_msgs::action::JointMovJ_Feedback;

  struct Impl
  {
    /// The send_goal service using a wrapped version of the goal message as a request.
    using SendGoalService = mg400_msgs::action::JointMovJ_SendGoal;
    /// The get_result service using a wrapped version of the result message as a response.
    using GetResultService = mg400_msgs::action::JointMovJ_GetResult;
    /// The feedback message with generic fields which wraps the feedback message.
    using FeedbackMessage = mg400_msgs::action::JointMovJ_FeedbackMessage;

    /// The generic service to cancel a goal.
    using CancelGoalService = action_msgs::srv::CancelGoal;
    /// The generic message for the status of a goal.
    using GoalStatusMessage = action_msgs::msg::GoalStatusArray;
  };
};

typedef struct JointMovJ JointMovJ;

}  // namespace action

}  // namespace mg400_msgs

#endif  // MG400_MSGS__ACTION__DETAIL__JOINT_MOV_J__STRUCT_HPP_
