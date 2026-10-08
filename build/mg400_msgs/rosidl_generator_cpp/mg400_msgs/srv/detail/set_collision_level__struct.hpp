// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:srv/SetCollisionLevel.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__SET_COLLISION_LEVEL__STRUCT_HPP_
#define MG400_MSGS__SRV__DETAIL__SET_COLLISION_LEVEL__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'level'
#include "mg400_msgs/msg/detail/collision_level__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetCollisionLevel_Request_
{
  using Type = SetCollisionLevel_Request_<ContainerAllocator>;

  explicit SetCollisionLevel_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : level(_init)
  {
    (void)_init;
  }

  explicit SetCollisionLevel_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : level(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _level_type =
    mg400_msgs::msg::CollisionLevel_<ContainerAllocator>;
  _level_type level;

  // setters for named parameter idiom
  Type & set__level(
    const mg400_msgs::msg::CollisionLevel_<ContainerAllocator> & _arg)
  {
    this->level = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Request
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Request
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetCollisionLevel_Request_ & other) const
  {
    if (this->level != other.level) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetCollisionLevel_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetCollisionLevel_Request_

// alias to use template instance with default allocator
using SetCollisionLevel_Request =
  mg400_msgs::srv::SetCollisionLevel_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetCollisionLevel_Response_
{
  using Type = SetCollisionLevel_Response_<ContainerAllocator>;

  explicit SetCollisionLevel_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
      this->error_id = 0l;
    }
  }

  explicit SetCollisionLevel_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
      this->error_id = 0l;
    }
  }

  // field types and members
  using _result_type =
    bool;
  _result_type result;
  using _error_id_type =
    int32_t;
  _error_id_type error_id;

  // setters for named parameter idiom
  Type & set__result(
    const bool & _arg)
  {
    this->result = _arg;
    return *this;
  }
  Type & set__error_id(
    const int32_t & _arg)
  {
    this->error_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Response
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__SetCollisionLevel_Response
    std::shared_ptr<mg400_msgs::srv::SetCollisionLevel_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetCollisionLevel_Response_ & other) const
  {
    if (this->result != other.result) {
      return false;
    }
    if (this->error_id != other.error_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetCollisionLevel_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetCollisionLevel_Response_

// alias to use template instance with default allocator
using SetCollisionLevel_Response =
  mg400_msgs::srv::SetCollisionLevel_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace srv
{

struct SetCollisionLevel
{
  using Request = mg400_msgs::srv::SetCollisionLevel_Request;
  using Response = mg400_msgs::srv::SetCollisionLevel_Response;
};

}  // namespace srv

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__SET_COLLISION_LEVEL__STRUCT_HPP_
