// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:srv/GetPose.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__GET_POSE__STRUCT_HPP_
#define MG400_MSGS__SRV__DETAIL__GET_POSE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__GetPose_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__GetPose_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GetPose_Request_
{
  using Type = GetPose_Request_<ContainerAllocator>;

  explicit GetPose_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  explicit GetPose_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::srv::GetPose_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::GetPose_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::GetPose_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::GetPose_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__GetPose_Request
    std::shared_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__GetPose_Request
    std::shared_ptr<mg400_msgs::srv::GetPose_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GetPose_Request_ & other) const
  {
    if (this->structure_needs_at_least_one_member != other.structure_needs_at_least_one_member) {
      return false;
    }
    return true;
  }
  bool operator!=(const GetPose_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GetPose_Request_

// alias to use template instance with default allocator
using GetPose_Request =
  mg400_msgs::srv::GetPose_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__GetPose_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__GetPose_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct GetPose_Response_
{
  using Type = GetPose_Response_<ContainerAllocator>;

  explicit GetPose_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->error_id = 0l;
      this->pose1 = 0.0;
      this->pose2 = 0.0;
      this->pose3 = 0.0;
      this->pose4 = 0.0;
      this->pose5 = 0.0;
      this->pose6 = 0.0;
    }
  }

  explicit GetPose_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->error_id = 0l;
      this->pose1 = 0.0;
      this->pose2 = 0.0;
      this->pose3 = 0.0;
      this->pose4 = 0.0;
      this->pose5 = 0.0;
      this->pose6 = 0.0;
    }
  }

  // field types and members
  using _error_id_type =
    int32_t;
  _error_id_type error_id;
  using _pose1_type =
    double;
  _pose1_type pose1;
  using _pose2_type =
    double;
  _pose2_type pose2;
  using _pose3_type =
    double;
  _pose3_type pose3;
  using _pose4_type =
    double;
  _pose4_type pose4;
  using _pose5_type =
    double;
  _pose5_type pose5;
  using _pose6_type =
    double;
  _pose6_type pose6;

  // setters for named parameter idiom
  Type & set__error_id(
    const int32_t & _arg)
  {
    this->error_id = _arg;
    return *this;
  }
  Type & set__pose1(
    const double & _arg)
  {
    this->pose1 = _arg;
    return *this;
  }
  Type & set__pose2(
    const double & _arg)
  {
    this->pose2 = _arg;
    return *this;
  }
  Type & set__pose3(
    const double & _arg)
  {
    this->pose3 = _arg;
    return *this;
  }
  Type & set__pose4(
    const double & _arg)
  {
    this->pose4 = _arg;
    return *this;
  }
  Type & set__pose5(
    const double & _arg)
  {
    this->pose5 = _arg;
    return *this;
  }
  Type & set__pose6(
    const double & _arg)
  {
    this->pose6 = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::srv::GetPose_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::GetPose_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::GetPose_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::GetPose_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__GetPose_Response
    std::shared_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__GetPose_Response
    std::shared_ptr<mg400_msgs::srv::GetPose_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const GetPose_Response_ & other) const
  {
    if (this->error_id != other.error_id) {
      return false;
    }
    if (this->pose1 != other.pose1) {
      return false;
    }
    if (this->pose2 != other.pose2) {
      return false;
    }
    if (this->pose3 != other.pose3) {
      return false;
    }
    if (this->pose4 != other.pose4) {
      return false;
    }
    if (this->pose5 != other.pose5) {
      return false;
    }
    if (this->pose6 != other.pose6) {
      return false;
    }
    return true;
  }
  bool operator!=(const GetPose_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct GetPose_Response_

// alias to use template instance with default allocator
using GetPose_Response =
  mg400_msgs::srv::GetPose_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace srv
{

struct GetPose
{
  using Request = mg400_msgs::srv::GetPose_Request;
  using Response = mg400_msgs::srv::GetPose_Response;
};

}  // namespace srv

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__GET_POSE__STRUCT_HPP_
