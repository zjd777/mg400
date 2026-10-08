// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/MovL.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_L__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_L__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose_stamped__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__MovL __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__MovL __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct MovL_
{
  using Type = MovL_<ContainerAllocator>;

  explicit MovL_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pose(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->set_speed_l = false;
      this->speed_l = 0;
      this->set_acc_l = false;
      this->acc_l = 0;
      this->set_cp = false;
      this->cp = 0;
    }
  }

  explicit MovL_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pose(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->set_speed_l = false;
      this->speed_l = 0;
      this->set_acc_l = false;
      this->acc_l = 0;
      this->set_cp = false;
      this->cp = 0;
    }
  }

  // field types and members
  using _pose_type =
    geometry_msgs::msg::PoseStamped_<ContainerAllocator>;
  _pose_type pose;
  using _set_speed_l_type =
    bool;
  _set_speed_l_type set_speed_l;
  using _speed_l_type =
    uint8_t;
  _speed_l_type speed_l;
  using _set_acc_l_type =
    bool;
  _set_acc_l_type set_acc_l;
  using _acc_l_type =
    uint8_t;
  _acc_l_type acc_l;
  using _set_cp_type =
    bool;
  _set_cp_type set_cp;
  using _cp_type =
    uint8_t;
  _cp_type cp;

  // setters for named parameter idiom
  Type & set__pose(
    const geometry_msgs::msg::PoseStamped_<ContainerAllocator> & _arg)
  {
    this->pose = _arg;
    return *this;
  }
  Type & set__set_speed_l(
    const bool & _arg)
  {
    this->set_speed_l = _arg;
    return *this;
  }
  Type & set__speed_l(
    const uint8_t & _arg)
  {
    this->speed_l = _arg;
    return *this;
  }
  Type & set__set_acc_l(
    const bool & _arg)
  {
    this->set_acc_l = _arg;
    return *this;
  }
  Type & set__acc_l(
    const uint8_t & _arg)
  {
    this->acc_l = _arg;
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
    mg400_msgs::msg::MovL_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::MovL_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::MovL_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::MovL_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::MovL_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::MovL_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::MovL_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::MovL_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::MovL_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::MovL_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__MovL
    std::shared_ptr<mg400_msgs::msg::MovL_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__MovL
    std::shared_ptr<mg400_msgs::msg::MovL_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MovL_ & other) const
  {
    if (this->pose != other.pose) {
      return false;
    }
    if (this->set_speed_l != other.set_speed_l) {
      return false;
    }
    if (this->speed_l != other.speed_l) {
      return false;
    }
    if (this->set_acc_l != other.set_acc_l) {
      return false;
    }
    if (this->acc_l != other.acc_l) {
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
  bool operator!=(const MovL_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MovL_

// alias to use template instance with default allocator
using MovL =
  mg400_msgs::msg::MovL_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__MOV_L__STRUCT_HPP_
