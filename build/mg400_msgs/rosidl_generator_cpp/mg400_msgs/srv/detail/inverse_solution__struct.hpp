// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:srv/InverseSolution.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__INVERSE_SOLUTION__STRUCT_HPP_
#define MG400_MSGS__SRV__DETAIL__INVERSE_SOLUTION__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__InverseSolution_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__InverseSolution_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct InverseSolution_Request_
{
  using Type = InverseSolution_Request_<ContainerAllocator>;

  explicit InverseSolution_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->x = 0.0;
      this->y = 0.0;
      this->z = 0.0;
      this->r = 0.0;
      this->user = 0;
      this->tool = 0;
    }
  }

  explicit InverseSolution_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->x = 0.0;
      this->y = 0.0;
      this->z = 0.0;
      this->r = 0.0;
      this->user = 0;
      this->tool = 0;
    }
  }

  // field types and members
  using _x_type =
    double;
  _x_type x;
  using _y_type =
    double;
  _y_type y;
  using _z_type =
    double;
  _z_type z;
  using _r_type =
    double;
  _r_type r;
  using _user_type =
    uint8_t;
  _user_type user;
  using _tool_type =
    uint8_t;
  _tool_type tool;

  // setters for named parameter idiom
  Type & set__x(
    const double & _arg)
  {
    this->x = _arg;
    return *this;
  }
  Type & set__y(
    const double & _arg)
  {
    this->y = _arg;
    return *this;
  }
  Type & set__z(
    const double & _arg)
  {
    this->z = _arg;
    return *this;
  }
  Type & set__r(
    const double & _arg)
  {
    this->r = _arg;
    return *this;
  }
  Type & set__user(
    const uint8_t & _arg)
  {
    this->user = _arg;
    return *this;
  }
  Type & set__tool(
    const uint8_t & _arg)
  {
    this->tool = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__InverseSolution_Request
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__InverseSolution_Request
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const InverseSolution_Request_ & other) const
  {
    if (this->x != other.x) {
      return false;
    }
    if (this->y != other.y) {
      return false;
    }
    if (this->z != other.z) {
      return false;
    }
    if (this->r != other.r) {
      return false;
    }
    if (this->user != other.user) {
      return false;
    }
    if (this->tool != other.tool) {
      return false;
    }
    return true;
  }
  bool operator!=(const InverseSolution_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct InverseSolution_Request_

// alias to use template instance with default allocator
using InverseSolution_Request =
  mg400_msgs::srv::InverseSolution_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__InverseSolution_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__InverseSolution_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct InverseSolution_Response_
{
  using Type = InverseSolution_Response_<ContainerAllocator>;

  explicit InverseSolution_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->error_id = 0l;
      this->j1 = 0.0;
      this->j2 = 0.0;
      this->j3 = 0.0;
      this->j4 = 0.0;
    }
  }

  explicit InverseSolution_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->error_id = 0l;
      this->j1 = 0.0;
      this->j2 = 0.0;
      this->j3 = 0.0;
      this->j4 = 0.0;
    }
  }

  // field types and members
  using _error_id_type =
    int32_t;
  _error_id_type error_id;
  using _j1_type =
    double;
  _j1_type j1;
  using _j2_type =
    double;
  _j2_type j2;
  using _j3_type =
    double;
  _j3_type j3;
  using _j4_type =
    double;
  _j4_type j4;

  // setters for named parameter idiom
  Type & set__error_id(
    const int32_t & _arg)
  {
    this->error_id = _arg;
    return *this;
  }
  Type & set__j1(
    const double & _arg)
  {
    this->j1 = _arg;
    return *this;
  }
  Type & set__j2(
    const double & _arg)
  {
    this->j2 = _arg;
    return *this;
  }
  Type & set__j3(
    const double & _arg)
  {
    this->j3 = _arg;
    return *this;
  }
  Type & set__j4(
    const double & _arg)
  {
    this->j4 = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__InverseSolution_Response
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__InverseSolution_Response
    std::shared_ptr<mg400_msgs::srv::InverseSolution_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const InverseSolution_Response_ & other) const
  {
    if (this->error_id != other.error_id) {
      return false;
    }
    if (this->j1 != other.j1) {
      return false;
    }
    if (this->j2 != other.j2) {
      return false;
    }
    if (this->j3 != other.j3) {
      return false;
    }
    if (this->j4 != other.j4) {
      return false;
    }
    return true;
  }
  bool operator!=(const InverseSolution_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct InverseSolution_Response_

// alias to use template instance with default allocator
using InverseSolution_Response =
  mg400_msgs::srv::InverseSolution_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace srv
{

struct InverseSolution
{
  using Request = mg400_msgs::srv::InverseSolution_Request;
  using Response = mg400_msgs::srv::InverseSolution_Response;
};

}  // namespace srv

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__INVERSE_SOLUTION__STRUCT_HPP_
