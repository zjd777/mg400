// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/ErrorID.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ERROR_ID__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__ERROR_ID__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'controller'
// Member 'servo'
#include "mg400_msgs/msg/detail/id_array__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__ErrorID __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__ErrorID __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ErrorID_
{
  using Type = ErrorID_<ContainerAllocator>;

  explicit ErrorID_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : controller(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->servo.fill(mg400_msgs::msg::IDArray_<ContainerAllocator>{_init});
    }
  }

  explicit ErrorID_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : controller(_alloc, _init),
    servo(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->servo.fill(mg400_msgs::msg::IDArray_<ContainerAllocator>{_alloc, _init});
    }
  }

  // field types and members
  using _controller_type =
    mg400_msgs::msg::IDArray_<ContainerAllocator>;
  _controller_type controller;
  using _servo_type =
    std::array<mg400_msgs::msg::IDArray_<ContainerAllocator>, 5>;
  _servo_type servo;

  // setters for named parameter idiom
  Type & set__controller(
    const mg400_msgs::msg::IDArray_<ContainerAllocator> & _arg)
  {
    this->controller = _arg;
    return *this;
  }
  Type & set__servo(
    const std::array<mg400_msgs::msg::IDArray_<ContainerAllocator>, 5> & _arg)
  {
    this->servo = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::msg::ErrorID_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::ErrorID_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::ErrorID_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::ErrorID_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__ErrorID
    std::shared_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__ErrorID
    std::shared_ptr<mg400_msgs::msg::ErrorID_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ErrorID_ & other) const
  {
    if (this->controller != other.controller) {
      return false;
    }
    if (this->servo != other.servo) {
      return false;
    }
    return true;
  }
  bool operator!=(const ErrorID_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ErrorID_

// alias to use template instance with default allocator
using ErrorID =
  mg400_msgs::msg::ErrorID_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ERROR_ID__STRUCT_HPP_
