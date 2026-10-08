// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/MovJIO.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__MOV_JIO__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__MOV_JIO__STRUCT_HPP_

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
// Member 'mode'
#include "mg400_msgs/msg/detail/distance_mode__struct.hpp"
// Member 'index'
#include "mg400_msgs/msg/detail/do_index__struct.hpp"
// Member 'status'
#include "mg400_msgs/msg/detail/do_status__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__MovJIO __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__MovJIO __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct MovJIO_
{
  using Type = MovJIO_<ContainerAllocator>;

  explicit MovJIO_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pose(_init),
    mode(_init),
    index(_init),
    status(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->distance = 0l;
      this->set_speed_j = false;
      this->speed_j = 0;
      this->set_acc_j = false;
      this->acc_j = 0;
    }
  }

  explicit MovJIO_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pose(_alloc, _init),
    mode(_alloc, _init),
    index(_alloc, _init),
    status(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->distance = 0l;
      this->set_speed_j = false;
      this->speed_j = 0;
      this->set_acc_j = false;
      this->acc_j = 0;
    }
  }

  // field types and members
  using _pose_type =
    geometry_msgs::msg::PoseStamped_<ContainerAllocator>;
  _pose_type pose;
  using _mode_type =
    mg400_msgs::msg::DistanceMode_<ContainerAllocator>;
  _mode_type mode;
  using _distance_type =
    int32_t;
  _distance_type distance;
  using _index_type =
    mg400_msgs::msg::DOIndex_<ContainerAllocator>;
  _index_type index;
  using _status_type =
    mg400_msgs::msg::DOStatus_<ContainerAllocator>;
  _status_type status;
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

  // setters for named parameter idiom
  Type & set__pose(
    const geometry_msgs::msg::PoseStamped_<ContainerAllocator> & _arg)
  {
    this->pose = _arg;
    return *this;
  }
  Type & set__mode(
    const mg400_msgs::msg::DistanceMode_<ContainerAllocator> & _arg)
  {
    this->mode = _arg;
    return *this;
  }
  Type & set__distance(
    const int32_t & _arg)
  {
    this->distance = _arg;
    return *this;
  }
  Type & set__index(
    const mg400_msgs::msg::DOIndex_<ContainerAllocator> & _arg)
  {
    this->index = _arg;
    return *this;
  }
  Type & set__status(
    const mg400_msgs::msg::DOStatus_<ContainerAllocator> & _arg)
  {
    this->status = _arg;
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

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::msg::MovJIO_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::MovJIO_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::MovJIO_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::MovJIO_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__MovJIO
    std::shared_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__MovJIO
    std::shared_ptr<mg400_msgs::msg::MovJIO_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MovJIO_ & other) const
  {
    if (this->pose != other.pose) {
      return false;
    }
    if (this->mode != other.mode) {
      return false;
    }
    if (this->distance != other.distance) {
      return false;
    }
    if (this->index != other.index) {
      return false;
    }
    if (this->status != other.status) {
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
    return true;
  }
  bool operator!=(const MovJIO_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MovJIO_

// alias to use template instance with default allocator
using MovJIO =
  mg400_msgs::msg::MovJIO_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__MOV_JIO__STRUCT_HPP_
