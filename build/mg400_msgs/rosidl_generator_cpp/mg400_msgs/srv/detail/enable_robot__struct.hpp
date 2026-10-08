// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from mg400_msgs:srv/EnableRobot.idl
// generated code does not contain a copyright notice

#ifndef MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__STRUCT_HPP_
#define MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__EnableRobot_Request __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__EnableRobot_Request __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct EnableRobot_Request_
{
  using Type = EnableRobot_Request_<ContainerAllocator>;

  explicit EnableRobot_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->num_of_params = 0;
      this->load = 0.0;
      this->center_x = 0.0;
      this->center_y = 0.0;
      this->center_z = 0.0;
    }
  }

  explicit EnableRobot_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->num_of_params = 0;
      this->load = 0.0;
      this->center_x = 0.0;
      this->center_y = 0.0;
      this->center_z = 0.0;
    }
  }

  // field types and members
  using _num_of_params_type =
    int8_t;
  _num_of_params_type num_of_params;
  using _load_type =
    double;
  _load_type load;
  using _center_x_type =
    double;
  _center_x_type center_x;
  using _center_y_type =
    double;
  _center_y_type center_y;
  using _center_z_type =
    double;
  _center_z_type center_z;

  // setters for named parameter idiom
  Type & set__num_of_params(
    const int8_t & _arg)
  {
    this->num_of_params = _arg;
    return *this;
  }
  Type & set__load(
    const double & _arg)
  {
    this->load = _arg;
    return *this;
  }
  Type & set__center_x(
    const double & _arg)
  {
    this->center_x = _arg;
    return *this;
  }
  Type & set__center_y(
    const double & _arg)
  {
    this->center_y = _arg;
    return *this;
  }
  Type & set__center_z(
    const double & _arg)
  {
    this->center_z = _arg;
    return *this;
  }

  // constant declarations
  static constexpr int8_t NO_PARAM =
    0;
  static constexpr int8_t ONE_PARAM =
    1;
  static constexpr int8_t FOUR_PARAM =
    4;

  // pointer types
  using RawPtr =
    mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__EnableRobot_Request
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__EnableRobot_Request
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const EnableRobot_Request_ & other) const
  {
    if (this->num_of_params != other.num_of_params) {
      return false;
    }
    if (this->load != other.load) {
      return false;
    }
    if (this->center_x != other.center_x) {
      return false;
    }
    if (this->center_y != other.center_y) {
      return false;
    }
    if (this->center_z != other.center_z) {
      return false;
    }
    return true;
  }
  bool operator!=(const EnableRobot_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct EnableRobot_Request_

// alias to use template instance with default allocator
using EnableRobot_Request =
  mg400_msgs::srv::EnableRobot_Request_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t EnableRobot_Request_<ContainerAllocator>::NO_PARAM;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t EnableRobot_Request_<ContainerAllocator>::ONE_PARAM;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr int8_t EnableRobot_Request_<ContainerAllocator>::FOUR_PARAM;
#endif  // __cplusplus < 201703L

}  // namespace srv

}  // namespace mg400_msgs


#ifndef _WIN32
# define DEPRECATED__mg400_msgs__srv__EnableRobot_Response __attribute__((deprecated))
#else
# define DEPRECATED__mg400_msgs__srv__EnableRobot_Response __declspec(deprecated)
#endif

namespace mg400_msgs
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct EnableRobot_Response_
{
  using Type = EnableRobot_Response_<ContainerAllocator>;

  explicit EnableRobot_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->result = false;
      this->error_id = 0l;
    }
  }

  explicit EnableRobot_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__mg400_msgs__srv__EnableRobot_Response
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__mg400_msgs__srv__EnableRobot_Response
    std::shared_ptr<mg400_msgs::srv::EnableRobot_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const EnableRobot_Response_ & other) const
  {
    if (this->result != other.result) {
      return false;
    }
    if (this->error_id != other.error_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const EnableRobot_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct EnableRobot_Response_

// alias to use template instance with default allocator
using EnableRobot_Response =
  mg400_msgs::srv::EnableRobot_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace mg400_msgs

namespace mg400_msgs
{

namespace srv
{

struct EnableRobot
{
  using Request = mg400_msgs::srv::EnableRobot_Request;
  using Response = mg400_msgs::srv::EnableRobot_Response;
};

}  // namespace srv

}  // namespace mg400_msgs

#endif  // MG400_MSGS__SRV__DETAIL__ENABLE_ROBOT__STRUCT_HPP_
