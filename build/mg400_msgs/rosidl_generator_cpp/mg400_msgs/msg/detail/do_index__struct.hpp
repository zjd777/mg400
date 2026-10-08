// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/DOIndex.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__DO_INDEX__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__DO_INDEX__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__DOIndex __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__DOIndex __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DOIndex_
{
  using Type = DOIndex_<ContainerAllocator>;

  explicit DOIndex_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->index = 0ul;
    }
  }

  explicit DOIndex_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->index = 0ul;
    }
  }

  // field types and members
  using _index_type =
    uint32_t;
  _index_type index;

  // setters for named parameter idiom
  Type & set__index(
    const uint32_t & _arg)
  {
    this->index = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint32_t D1 =
    1u;
  static constexpr uint32_t D2 =
    2u;
  static constexpr uint32_t D3 =
    3u;
  static constexpr uint32_t D4 =
    4u;
  static constexpr uint32_t D5 =
    5u;
  static constexpr uint32_t D6 =
    6u;
  static constexpr uint32_t D7 =
    7u;
  static constexpr uint32_t D8 =
    8u;
  static constexpr uint32_t D9 =
    9u;
  static constexpr uint32_t D10 =
    10u;
  static constexpr uint32_t D11 =
    11u;
  static constexpr uint32_t D12 =
    12u;
  static constexpr uint32_t D13 =
    13u;
  static constexpr uint32_t D14 =
    14u;
  static constexpr uint32_t D15 =
    15u;
  static constexpr uint32_t D16 =
    16u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::DOIndex_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::DOIndex_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::DOIndex_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::DOIndex_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__DOIndex
    std::shared_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__DOIndex
    std::shared_ptr<mg400_msgs::msg::DOIndex_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DOIndex_ & other) const
  {
    if (this->index != other.index) {
      return false;
    }
    return true;
  }
  bool operator!=(const DOIndex_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DOIndex_

// alias to use template instance with default allocator
using DOIndex =
  mg400_msgs::msg::DOIndex_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D1;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D2;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D3;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D4;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D5;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D6;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D7;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D8;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D9;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D10;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D11;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D12;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D13;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D14;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D15;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint32_t DOIndex_<ContainerAllocator>::D16;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__DO_INDEX__STRUCT_HPP_
