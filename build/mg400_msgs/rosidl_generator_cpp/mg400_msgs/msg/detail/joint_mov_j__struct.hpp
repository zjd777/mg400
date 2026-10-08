// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/JointMovJ.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__JointMovJ __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__JointMovJ __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct JointMovJ_
{
  using Type = JointMovJ_<ContainerAllocator>;

  explicit JointMovJ_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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

  explicit JointMovJ_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::msg::JointMovJ_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::JointMovJ_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::JointMovJ_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::JointMovJ_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__JointMovJ
    std::shared_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__JointMovJ
    std::shared_ptr<mg400_msgs::msg::JointMovJ_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const JointMovJ_ & other) const
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
  bool operator!=(const JointMovJ_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct JointMovJ_

// alias to use template instance with default allocator
using JointMovJ =
  mg400_msgs::msg::JointMovJ_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__JOINT_MOV_J__STRUCT_HPP_
