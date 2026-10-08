// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/Tool.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__TOOL__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__TOOL__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__Tool __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__Tool __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Tool_
{
  using Type = Tool_<ContainerAllocator>;

  explicit Tool_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->tool = 0;
    }
  }

  explicit Tool_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->tool = 0;
    }
  }

  // field types and members
  using _tool_type =
    uint8_t;
  _tool_type tool;

  // setters for named parameter idiom
  Type & set__tool(
    const uint8_t & _arg)
  {
    this->tool = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t TOOL0 =
    0u;
  static constexpr uint8_t TOOL1 =
    1u;
  static constexpr uint8_t TOOL2 =
    2u;
  static constexpr uint8_t TOOL3 =
    3u;
  static constexpr uint8_t TOOL4 =
    4u;
  static constexpr uint8_t TOOL5 =
    5u;
  static constexpr uint8_t TOOL6 =
    6u;
  static constexpr uint8_t TOOL7 =
    7u;
  static constexpr uint8_t TOOL8 =
    8u;
  static constexpr uint8_t TOOL9 =
    9u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::Tool_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::Tool_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::Tool_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::Tool_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::Tool_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::Tool_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::Tool_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::Tool_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::Tool_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::Tool_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__Tool
    std::shared_ptr<mg400_msgs::msg::Tool_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__Tool
    std::shared_ptr<mg400_msgs::msg::Tool_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Tool_ & other) const
  {
    if (this->tool != other.tool) {
      return false;
    }
    return true;
  }
  bool operator!=(const Tool_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Tool_

// alias to use template instance with default allocator
using Tool =
  mg400_msgs::msg::Tool_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL0;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL1;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL2;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL3;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL4;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL5;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL6;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL7;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL8;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t Tool_<ContainerAllocator>::TOOL9;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__TOOL__STRUCT_HPP_
