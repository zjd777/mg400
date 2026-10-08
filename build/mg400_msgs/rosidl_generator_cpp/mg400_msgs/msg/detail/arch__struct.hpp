// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/Arch.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ARCH__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__ARCH__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__Arch __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__Arch __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Arch_
{
  using Type = Arch_<ContainerAllocator>;

  explicit Arch_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->index = 0;
    }
  }

  explicit Arch_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->index = 0;
    }
  }

  // field types and members
  using _index_type =
    uint8_t;
  _index_type index;

  // setters for named parameter idiom
  Type & set__index(
    const uint8_t & _arg)
  {
    this->index = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t INDEX0 =
    0u;
  static constexpr uint8_t INDEX1 =
    1u;
  static constexpr uint8_t INDEX2 =
    2u;
  static constexpr uint8_t INDEX3 =
    3u;
  static constexpr uint8_t INDEX4 =
    4u;
  static constexpr uint8_t INDEX5 =
    5u;
  static constexpr uint8_t INDEX6 =
    6u;
  static constexpr uint8_t INDEX7 =
    7u;
  static constexpr uint8_t INDEX8 =
    8u;
  static constexpr uint8_t INDEX9 =
    9u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::Arch_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::Arch_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::Arch_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::Arch_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::Arch_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::Arch_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::Arch_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::Arch_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::Arch_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::Arch_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__Arch
    std::shared_ptr<mg400_msgs::msg::Arch_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__Arch
    std::shared_ptr<mg400_msgs::msg::Arch_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Arch_ & other) const
  {
    if (this->index != other.index) {
      return false;
    }
    return true;
  }
  bool operator!=(const Arch_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Arch_

// alias to use template instance with default allocator
using Arch =
  mg400_msgs::msg::Arch_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX0;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX1;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX2;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX3;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX4;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX5;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX6;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX7;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX8;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Arch_<ContainerAllocator>::INDEX9;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ARCH__STRUCT_HPP_
