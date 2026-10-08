// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:msg/IDArray.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__MSG__DETAIL__ID_ARRAY__STRUCT_HPP_
#define MG400_MSGS__MSG__DETAIL__ID_ARRAY__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__msg__IDArray __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__msg__IDArray __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct IDArray_
{
  using Type = IDArray_<ContainerAllocator>;

  explicit IDArray_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
  }

  explicit IDArray_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_init;
    (void)_alloc;
  }

  // field types and members
  using _ids_type =
    std::vector<int32_t, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<int32_t>>;
  _ids_type ids;

  // setters for named parameter idiom
  Type & set__ids(
    const std::vector<int32_t, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<int32_t>> & _arg)
  {
    this->ids = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::msg::IDArray_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::msg::IDArray_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::IDArray_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::msg::IDArray_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__msg__IDArray
    std::shared_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__msg__IDArray
    std::shared_ptr<mg400_msgs::msg::IDArray_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const IDArray_ & other) const
  {
    if (this->ids != other.ids) {
      return false;
    }
    return true;
  }
  bool operator!=(const IDArray_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct IDArray_

// alias to use template instance with default allocator
using IDArray =
  mg400_msgs::msg::IDArray_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace mg400_msgs

#endif  // MG400_MSGS__MSG__DETAIL__ID_ARRAY__STRUCT_HPP_
