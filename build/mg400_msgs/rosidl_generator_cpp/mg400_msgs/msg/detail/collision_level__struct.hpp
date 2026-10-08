// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/CollisionLevel.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__CollisionLevel __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__CollisionLevel __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct CollisionLevel_
{
  using Type = CollisionLevel_<ContainerAllocator>;

  explicit CollisionLevel_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->level = 0;
    }
  }

  explicit CollisionLevel_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->level = 0;
    }
  }

  // field types and members
  using _level_type =
    uint8_t;
  _level_type level;

  // setters for named parameter idiom
  Type & set__level(
    const uint8_t & _arg)
  {
    this->level = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t OFF =
    0u;
  static constexpr uint8_t LEVEL1 =
    1u;
  static constexpr uint8_t LEVEL2 =
    2u;
  static constexpr uint8_t LEVEL3 =
    3u;
  static constexpr uint8_t LEVEL4 =
    4u;
  static constexpr uint8_t LEVEL5 =
    5u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::CollisionLevel_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::CollisionLevel_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::CollisionLevel_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::CollisionLevel_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__CollisionLevel
    std::shared_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__CollisionLevel
    std::shared_ptr<mg400_msgs::msg::CollisionLevel_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CollisionLevel_ & other) const
  {
    if (this->level != other.level) {
      return false;
    }
    return true;
  }
  bool operator!=(const CollisionLevel_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CollisionLevel_

// alias to use template instance with default allocator
using CollisionLevel =
  mg400_msgs::msg::CollisionLevel_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CollisionLevel_<ContainerAllocator>::OFF;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CollisionLevel_<ContainerAllocator>::LEVEL1;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CollisionLevel_<ContainerAllocator>::LEVEL2;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CollisionLevel_<ContainerAllocator>::LEVEL3;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CollisionLevel_<ContainerAllocator>::LEVEL4;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CollisionLevel_<ContainerAllocator>::LEVEL5;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__COLLISION_LEVEL__STRUCT_HPP_
