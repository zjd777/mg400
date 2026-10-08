// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/Command.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__COMMAND__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__COMMAND__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'mov_j_params'
#include "mg400_msgs/msg/detail/mov_j__struct.hpp"
// Member 'mov_l_params'
#include "mg400_msgs/msg/detail/mov_l__struct.hpp"
// Member 'joint_mov_j_params'
#include "mg400_msgs/msg/detail/joint_mov_j__struct.hpp"
// Member 'mov_jio_params'
#include "mg400_msgs/msg/detail/mov_jio__struct.hpp"
// Member 'mov_lio_params'
#include "mg400_msgs/msg/detail/mov_lio__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__Command __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__Command __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Command_
{
  using Type = Command_<ContainerAllocator>;

  explicit Command_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : mov_j_params(_init),
    mov_l_params(_init),
    joint_mov_j_params(_init),
    mov_jio_params(_init),
    mov_lio_params(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->command_type = 0;
    }
  }

  explicit Command_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : mov_j_params(_alloc, _init),
    mov_l_params(_alloc, _init),
    joint_mov_j_params(_alloc, _init),
    mov_jio_params(_alloc, _init),
    mov_lio_params(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->command_type = 0;
    }
  }

  // field types and members
  using _command_type_type =
    uint16_t;
  _command_type_type command_type;
  using _mov_j_params_type =
    mg400_msgs::msg::MovJ_<ContainerAllocator>;
  _mov_j_params_type mov_j_params;
  using _mov_l_params_type =
    mg400_msgs::msg::MovL_<ContainerAllocator>;
  _mov_l_params_type mov_l_params;
  using _joint_mov_j_params_type =
    mg400_msgs::msg::JointMovJ_<ContainerAllocator>;
  _joint_mov_j_params_type joint_mov_j_params;
  using _mov_jio_params_type =
    mg400_msgs::msg::MovJIO_<ContainerAllocator>;
  _mov_jio_params_type mov_jio_params;
  using _mov_lio_params_type =
    mg400_msgs::msg::MovLIO_<ContainerAllocator>;
  _mov_lio_params_type mov_lio_params;

  // setters for named parameter idiom
  Type & set__command_type(
    const uint16_t & _arg)
  {
    this->command_type = _arg;
    return *this;
  }
  Type & set__mov_j_params(
    const mg400_msgs::msg::MovJ_<ContainerAllocator> & _arg)
  {
    this->mov_j_params = _arg;
    return *this;
  }
  Type & set__mov_l_params(
    const mg400_msgs::msg::MovL_<ContainerAllocator> & _arg)
  {
    this->mov_l_params = _arg;
    return *this;
  }
  Type & set__joint_mov_j_params(
    const mg400_msgs::msg::JointMovJ_<ContainerAllocator> & _arg)
  {
    this->joint_mov_j_params = _arg;
    return *this;
  }
  Type & set__mov_jio_params(
    const mg400_msgs::msg::MovJIO_<ContainerAllocator> & _arg)
  {
    this->mov_jio_params = _arg;
    return *this;
  }
  Type & set__mov_lio_params(
    const mg400_msgs::msg::MovLIO_<ContainerAllocator> & _arg)
  {
    this->mov_lio_params = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint16_t CT_MOV_J =
    1u;
  static constexpr uint16_t CT_MOV_L =
    2u;
  static constexpr uint16_t CT_JOINT_MOV_J =
    3u;
  static constexpr uint16_t CT_MOV_JIO =
    4u;
  static constexpr uint16_t CT_MOV_LIO =
    5u;
  static constexpr uint16_t CT_TOOL_DO =
    25u;

  // pointer types
  using RawPtr =
    mg400_msgs::msg::Command_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::Command_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::Command_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::Command_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::Command_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::Command_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::Command_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::Command_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::Command_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::Command_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__Command
    std::shared_ptr<mg400_msgs::msg::Command_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__Command
    std::shared_ptr<mg400_msgs::msg::Command_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Command_ & other) const
  {
    if (this->command_type != other.command_type) {
      return false;
    }
    if (this->mov_j_params != other.mov_j_params) {
      return false;
    }
    if (this->mov_l_params != other.mov_l_params) {
      return false;
    }
    if (this->joint_mov_j_params != other.joint_mov_j_params) {
      return false;
    }
    if (this->mov_jio_params != other.mov_jio_params) {
      return false;
    }
    if (this->mov_lio_params != other.mov_lio_params) {
      return false;
    }
    return true;
  }
  bool operator!=(const Command_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Command_

// alias to use template instance with default allocator
using Command =
  mg400_msgs::msg::Command_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint16_t Command_<ContainerAllocator>::CT_MOV_J;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint16_t Command_<ContainerAllocator>::CT_MOV_L;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint16_t Command_<ContainerAllocator>::CT_JOINT_MOV_J;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint16_t Command_<ContainerAllocator>::CT_MOV_JIO;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint16_t Command_<ContainerAllocator>::CT_MOV_LIO;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint16_t Command_<ContainerAllocator>::CT_TOOL_DO;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__COMMAND__STRUCT_HPP_
