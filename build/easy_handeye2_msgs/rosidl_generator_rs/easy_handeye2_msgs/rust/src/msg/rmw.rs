#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__HandeyeCalibration() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__msg__HandeyeCalibration__init(msg: *mut HandeyeCalibration) -> bool;
    fn easy_handeye2_msgs__msg__HandeyeCalibration__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandeyeCalibration>, size: usize) -> bool;
    fn easy_handeye2_msgs__msg__HandeyeCalibration__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandeyeCalibration>);
    fn easy_handeye2_msgs__msg__HandeyeCalibration__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandeyeCalibration>, out_seq: *mut rosidl_runtime_rs::Sequence<HandeyeCalibration>) -> bool;
}

// Corresponds to easy_handeye2_msgs__msg__HandeyeCalibration
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandeyeCalibration {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::super::msg::rmw::HandeyeCalibrationParameters,


    // This member is not documented.
    #[allow(missing_docs)]
    pub transform: geometry_msgs::msg::rmw::Transform,

}



impl Default for HandeyeCalibration {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__msg__HandeyeCalibration__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__msg__HandeyeCalibration__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandeyeCalibration {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__HandeyeCalibration__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__HandeyeCalibration__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__HandeyeCalibration__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandeyeCalibration {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandeyeCalibration where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/msg/HandeyeCalibration";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__HandeyeCalibration() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__HandeyeCalibrationParameters() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__msg__HandeyeCalibrationParameters__init(msg: *mut HandeyeCalibrationParameters) -> bool;
    fn easy_handeye2_msgs__msg__HandeyeCalibrationParameters__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HandeyeCalibrationParameters>, size: usize) -> bool;
    fn easy_handeye2_msgs__msg__HandeyeCalibrationParameters__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HandeyeCalibrationParameters>);
    fn easy_handeye2_msgs__msg__HandeyeCalibrationParameters__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HandeyeCalibrationParameters>, out_seq: *mut rosidl_runtime_rs::Sequence<HandeyeCalibrationParameters>) -> bool;
}

// Corresponds to easy_handeye2_msgs__msg__HandeyeCalibrationParameters
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandeyeCalibrationParameters {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calibration_type: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_base_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_effector_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking_base_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking_marker_frame: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub freehand_robot_movement: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_group_namespace: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_group: rosidl_runtime_rs::String,

}



impl Default for HandeyeCalibrationParameters {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__msg__HandeyeCalibrationParameters__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__msg__HandeyeCalibrationParameters__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HandeyeCalibrationParameters {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__HandeyeCalibrationParameters__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__HandeyeCalibrationParameters__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__HandeyeCalibrationParameters__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HandeyeCalibrationParameters {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HandeyeCalibrationParameters where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/msg/HandeyeCalibrationParameters";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__HandeyeCalibrationParameters() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__Sample() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__msg__Sample__init(msg: *mut Sample) -> bool;
    fn easy_handeye2_msgs__msg__Sample__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sample>, size: usize) -> bool;
    fn easy_handeye2_msgs__msg__Sample__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sample>);
    fn easy_handeye2_msgs__msg__Sample__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sample>, out_seq: *mut rosidl_runtime_rs::Sequence<Sample>) -> bool;
}

// Corresponds to easy_handeye2_msgs__msg__Sample
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sample {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: geometry_msgs::msg::rmw::Transform,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking: geometry_msgs::msg::rmw::Transform,

}



impl Default for Sample {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__msg__Sample__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__msg__Sample__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sample {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__Sample__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__Sample__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__Sample__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sample {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sample where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/msg/Sample";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__Sample() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__SampleList() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__msg__SampleList__init(msg: *mut SampleList) -> bool;
    fn easy_handeye2_msgs__msg__SampleList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SampleList>, size: usize) -> bool;
    fn easy_handeye2_msgs__msg__SampleList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SampleList>);
    fn easy_handeye2_msgs__msg__SampleList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SampleList>, out_seq: *mut rosidl_runtime_rs::Sequence<SampleList>) -> bool;
}

// Corresponds to easy_handeye2_msgs__msg__SampleList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SampleList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::super::msg::rmw::HandeyeCalibrationParameters,


    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Sample>,

}



impl Default for SampleList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__msg__SampleList__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__msg__SampleList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SampleList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__SampleList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__SampleList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__SampleList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SampleList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SampleList where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/msg/SampleList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__SampleList() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__TargetPoseList() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__msg__TargetPoseList__init(msg: *mut TargetPoseList) -> bool;
    fn easy_handeye2_msgs__msg__TargetPoseList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TargetPoseList>, size: usize) -> bool;
    fn easy_handeye2_msgs__msg__TargetPoseList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TargetPoseList>);
    fn easy_handeye2_msgs__msg__TargetPoseList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TargetPoseList>, out_seq: *mut rosidl_runtime_rs::Sequence<TargetPoseList>) -> bool;
}

// Corresponds to easy_handeye2_msgs__msg__TargetPoseList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TargetPoseList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::super::msg::rmw::HandeyeCalibrationParameters,


    // This member is not documented.
    #[allow(missing_docs)]
    pub home_pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::PoseStamped>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_target_pose_index: i64,

}



impl Default for TargetPoseList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__msg__TargetPoseList__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__msg__TargetPoseList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TargetPoseList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__TargetPoseList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__TargetPoseList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__msg__TargetPoseList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TargetPoseList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TargetPoseList where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/msg/TargetPoseList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__msg__TargetPoseList() }
  }
}


