// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/User.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__USER__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__USER__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__User __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__User __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct User_
{
  using Type = User_<ContainerAllocator>;

  explicit User_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->user = 0;
    }
  }

  explicit User_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->user = 0;
    }
  }

  // field types and members
  using _user_type =
    uint8_t;
  _user_type user;

  // setters for named parameter idiom
  Type & set__user(
    const uint8_t & _arg)
  {
    this->user = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t USER0 =
    0u;
  static constexpr uint8_t USER1 =
    1u;
  static constexpr uint8_t USER2 =
    2u;
  static constexpr uint8_t USER3 =
    3u;
  static constexpr uint8_t USER4 =
    4u;
  static constexpr uint8_t USER5 =
    5u;
  static constexpr uint8_t USER6 =
    6u;
  static constexpr uint8_t USER7 =
    7u;
  static constexpr uint8_t USER8 =
    8u;
  static constexpr uint8_t USER9 =
    9u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::User_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::User_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::User_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::User_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::User_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::User_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::User_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::User_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::User_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::User_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__User
    std::shared_ptr<mg400_msgs::msg::User_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__User
    std::shared_ptr<mg400_msgs::msg::User_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const User_ & other) const
  {
    if (this->user != other.user) {
      return false;
    }
    return true;
  }
  bool operator!=(const User_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct User_

// alias to use template instance with default allocator
using User =
  mg400_msgs::msg::User_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER0;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER1;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER2;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER3;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER4;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER5;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER6;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER7;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER8;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t User_<ContainerAllocator>::USER9;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__USER__STRUCT_HPP_
