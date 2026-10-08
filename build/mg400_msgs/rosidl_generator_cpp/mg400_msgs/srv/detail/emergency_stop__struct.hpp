// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:srv/EmergencyStop.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__EMERGENCY_STOP__STRUCT_HPP_
#define MG400_MSGS__SRV__DETAIL__EMERGENCY_STOP__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__EmergencyStop_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__EmergencyStop_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct EmergencyStop_Request_
{
  using Type = EmergencyStop_Request_<ContainerAllocator>;

  explicit EmergencyStop_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  explicit EmergencyStop_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  // field types and members
  using _structure_needs_at_least_one_member_type =
    uint8_t;
  _structure_needs_at_least_one_member_type structure_needs_at_least_one_member;


  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__EmergencyStop_Request
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__EmergencyStop_Request
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const EmergencyStop_Request_ & other) const
  {
    if (this->structure_needs_at_least_one_member != other.structure_needs_at_least_one_member) {
      return false;
    }
    return true;
  }
  bool operator!=(const EmergencyStop_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct EmergencyStop_Request_

// alias to use template instance with default allocator
using EmergencyStop_Request =
  mg400_msgs::srv::EmergencyStop_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__EmergencyStop_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__EmergencyStop_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct EmergencyStop_Response_
{
  using Type = EmergencyStop_Response_<ContainerAllocator>;

  explicit EmergencyStop_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
      this->error_id = 0l;
    }
  }

  explicit EmergencyStop_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__EmergencyStop_Response
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__EmergencyStop_Response
    std::shared_ptr<mg400_msgs::srv::EmergencyStop_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const EmergencyStop_Response_ & other) const
  {
    if (this->result != other.result) {
      return false;
    }
    if (this->error_id != other.error_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const EmergencyStop_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct EmergencyStop_Response_

// alias to use template instance with default allocator
using EmergencyStop_Response =
  mg400_msgs::srv::EmergencyStop_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace srv
{

struct EmergencyStop
{
  using Request = mg400_msgs::srv::EmergencyStop_Request;
  using Response = mg400_msgs::srv::EmergencyStop_Response;
};

}  // namespace srv

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__EMERGENCY_STOP__STRUCT_HPP_
