// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/DOStatus.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DO_STATUS__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__DO_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__DOStatus __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__DOStatus __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DOStatus_
{
  using Type = DOStatus_<ContainerAllocator>;

  explicit DOStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  explicit DOStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  // field types and members
  using _status_type =
    uint8_t;
  _status_type status;

  // setters for named parameter idiom
  Type & set__status(
    const uint8_t & _arg)
  {
    this->status = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t LOW =
    0u;
  static constexpr uint8_t HIGH =
    1u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::DOStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::DOStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::DOStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::DOStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__DOStatus
    std::shared_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__DOStatus
    std::shared_ptr<mg400_msgs::msg::DOStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DOStatus_ & other) const
  {
    if (this->status != other.status) {
      return false;
    }
    return true;
  }
  bool operator!=(const DOStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DOStatus_

// alias to use template instance with default allocator
using DOStatus =
  mg400_msgs::msg::DOStatus_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t DOStatus_<ContainerAllocator>::LOW;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t DOStatus_<ContainerAllocator>::HIGH;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__DO_STATUS__STRUCT_HPP_
