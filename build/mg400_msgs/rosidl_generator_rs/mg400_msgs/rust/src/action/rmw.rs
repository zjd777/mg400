
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_Goal() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_Goal__init(msg: *mut CommandQueue_Goal) -> bool;
    fn mg400_msgs__action__CommandQueue_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Goal>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Goal>);
    fn mg400_msgs__action__CommandQueue_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Goal>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub commands: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Command>,

}



impl Default for CommandQueue_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_Goal__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_Goal() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_Result() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_Result__init(msg: *mut CommandQueue_Result) -> bool;
    fn mg400_msgs__action__CommandQueue_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Result>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Result>);
    fn mg400_msgs__action__CommandQueue_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Result>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::super::msg::rmw::ErrorID,

}



impl Default for CommandQueue_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_Result__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_Result where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_Result() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_Feedback__init(msg: *mut CommandQueue_Feedback) -> bool;
    fn mg400_msgs__action__CommandQueue_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Feedback>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Feedback>);
    fn mg400_msgs__action__CommandQueue_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_Feedback>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// Current joint angles in radian
    pub current_angles: [f64; 4],

}



impl Default for CommandQueue_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_Feedback__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_Feedback() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_FeedbackMessage__init(msg: *mut CommandQueue_FeedbackMessage) -> bool;
    fn mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_FeedbackMessage>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_FeedbackMessage>);
    fn mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_FeedbackMessage>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::CommandQueue_Feedback,

}



impl Default for CommandQueue_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_FeedbackMessage() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_Goal() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_Goal__init(msg: *mut JointMovJ_Goal) -> bool;
    fn mg400_msgs__action__JointMovJ_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Goal>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Goal>);
    fn mg400_msgs__action__JointMovJ_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Goal>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Goal {
    /// Joint angles in radian
    pub joint_angles: [f64; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_speed_j: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_j: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_acc_j: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc_j: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for JointMovJ_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_Goal__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_Goal() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_Result() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_Result__init(msg: *mut JointMovJ_Result) -> bool;
    fn mg400_msgs__action__JointMovJ_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Result>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Result>);
    fn mg400_msgs__action__JointMovJ_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Result>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::super::msg::rmw::ErrorID,

}



impl Default for JointMovJ_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_Result__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_Result where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_Result() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_Feedback__init(msg: *mut JointMovJ_Feedback) -> bool;
    fn mg400_msgs__action__JointMovJ_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Feedback>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Feedback>);
    fn mg400_msgs__action__JointMovJ_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Feedback>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_angles: [f64; 4],

}



impl Default for JointMovJ_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_Feedback__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_Feedback() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_FeedbackMessage__init(msg: *mut JointMovJ_FeedbackMessage) -> bool;
    fn mg400_msgs__action__JointMovJ_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_FeedbackMessage>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_FeedbackMessage>);
    fn mg400_msgs__action__JointMovJ_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_FeedbackMessage>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::JointMovJ_Feedback,

}



impl Default for JointMovJ_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_FeedbackMessage() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_Goal() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_Goal__init(msg: *mut MovJ_Goal) -> bool;
    fn mg400_msgs__action__MovJ_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Goal>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Goal>);
    fn mg400_msgs__action__MovJ_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_Goal>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_speed_j: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_j: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_acc_j: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc_j: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for MovJ_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_Goal__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_Goal() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_Result() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_Result__init(msg: *mut MovJ_Result) -> bool;
    fn mg400_msgs__action__MovJ_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Result>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Result>);
    fn mg400_msgs__action__MovJ_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_Result>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::super::msg::rmw::ErrorID,

}



impl Default for MovJ_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_Result__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_Result where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_Result() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_Feedback__init(msg: *mut MovJ_Feedback) -> bool;
    fn mg400_msgs__action__MovJ_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Feedback>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Feedback>);
    fn mg400_msgs__action__MovJ_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_Feedback>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for MovJ_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_Feedback__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_Feedback() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_FeedbackMessage__init(msg: *mut MovJ_FeedbackMessage) -> bool;
    fn mg400_msgs__action__MovJ_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_FeedbackMessage>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_FeedbackMessage>);
    fn mg400_msgs__action__MovJ_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_FeedbackMessage>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MovJ_Feedback,

}



impl Default for MovJ_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_FeedbackMessage() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_Goal() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_Goal__init(msg: *mut MovJIO_Goal) -> bool;
    fn mg400_msgs__action__MovJIO_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Goal>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Goal>);
    fn mg400_msgs__action__MovJIO_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Goal>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: super::super::msg::rmw::DistanceMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::DOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::DOStatus,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_speed_j: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_j: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_acc_j: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc_j: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for MovJIO_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_Goal__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_Goal() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_Result() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_Result__init(msg: *mut MovJIO_Result) -> bool;
    fn mg400_msgs__action__MovJIO_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Result>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Result>);
    fn mg400_msgs__action__MovJIO_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Result>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::super::msg::rmw::ErrorID,

}



impl Default for MovJIO_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_Result__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_Result where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_Result() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_Feedback__init(msg: *mut MovJIO_Feedback) -> bool;
    fn mg400_msgs__action__MovJIO_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Feedback>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Feedback>);
    fn mg400_msgs__action__MovJIO_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Feedback>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for MovJIO_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_Feedback__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_Feedback() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_FeedbackMessage__init(msg: *mut MovJIO_FeedbackMessage) -> bool;
    fn mg400_msgs__action__MovJIO_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_FeedbackMessage>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_FeedbackMessage>);
    fn mg400_msgs__action__MovJIO_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_FeedbackMessage>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MovJIO_Feedback,

}



impl Default for MovJIO_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_FeedbackMessage() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_Goal() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_Goal__init(msg: *mut MovL_Goal) -> bool;
    fn mg400_msgs__action__MovL_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_Goal>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_Goal>);
    fn mg400_msgs__action__MovL_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_Goal>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_speed_l: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_acc_l: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for MovL_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_Goal__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_Goal() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_Result() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_Result__init(msg: *mut MovL_Result) -> bool;
    fn mg400_msgs__action__MovL_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_Result>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_Result>);
    fn mg400_msgs__action__MovL_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_Result>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::super::msg::rmw::ErrorID,

}



impl Default for MovL_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_Result__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_Result where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_Result() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_Feedback__init(msg: *mut MovL_Feedback) -> bool;
    fn mg400_msgs__action__MovL_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_Feedback>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_Feedback>);
    fn mg400_msgs__action__MovL_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_Feedback>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for MovL_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_Feedback__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_Feedback() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_FeedbackMessage__init(msg: *mut MovL_FeedbackMessage) -> bool;
    fn mg400_msgs__action__MovL_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_FeedbackMessage>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_FeedbackMessage>);
    fn mg400_msgs__action__MovL_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_FeedbackMessage>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MovL_Feedback,

}



impl Default for MovL_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_FeedbackMessage() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_Goal() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_Goal__init(msg: *mut MovLIO_Goal) -> bool;
    fn mg400_msgs__action__MovLIO_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Goal>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Goal>);
    fn mg400_msgs__action__MovLIO_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Goal>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: super::super::msg::rmw::DistanceMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::DOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::DOStatus,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_speed_l: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_acc_l: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc_l: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for MovLIO_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_Goal__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_Goal() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_Result() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_Result__init(msg: *mut MovLIO_Result) -> bool;
    fn mg400_msgs__action__MovLIO_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Result>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Result>);
    fn mg400_msgs__action__MovLIO_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Result>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::super::msg::rmw::ErrorID,

}



impl Default for MovLIO_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_Result__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_Result where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_Result() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_Feedback__init(msg: *mut MovLIO_Feedback) -> bool;
    fn mg400_msgs__action__MovLIO_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Feedback>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Feedback>);
    fn mg400_msgs__action__MovLIO_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Feedback>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for MovLIO_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_Feedback__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_Feedback() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_FeedbackMessage__init(msg: *mut MovLIO_FeedbackMessage) -> bool;
    fn mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_FeedbackMessage>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_FeedbackMessage>);
    fn mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_FeedbackMessage>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MovLIO_Feedback,

}



impl Default for MovLIO_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_FeedbackMessage() }
  }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_SendGoal_Request__init(msg: *mut CommandQueue_SendGoal_Request) -> bool;
    fn mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Request>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Request>);
    fn mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::CommandQueue_Goal,

}



impl Default for CommandQueue_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_SendGoal_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_SendGoal_Response__init(msg: *mut CommandQueue_SendGoal_Response) -> bool;
    fn mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Response>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Response>);
    fn mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_SendGoal_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for CommandQueue_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_SendGoal_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_GetResult_Request__init(msg: *mut CommandQueue_GetResult_Request) -> bool;
    fn mg400_msgs__action__CommandQueue_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Request>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Request>);
    fn mg400_msgs__action__CommandQueue_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for CommandQueue_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_GetResult_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__CommandQueue_GetResult_Response__init(msg: *mut CommandQueue_GetResult_Response) -> bool;
    fn mg400_msgs__action__CommandQueue_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Response>, size: usize) -> bool;
    fn mg400_msgs__action__CommandQueue_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Response>);
    fn mg400_msgs__action__CommandQueue_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CommandQueue_GetResult_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__CommandQueue_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::CommandQueue_Result,

}



impl Default for CommandQueue_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__CommandQueue_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__CommandQueue_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CommandQueue_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__CommandQueue_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CommandQueue_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/CommandQueue_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__CommandQueue_GetResult_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_SendGoal_Request__init(msg: *mut JointMovJ_SendGoal_Request) -> bool;
    fn mg400_msgs__action__JointMovJ_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Request>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Request>);
    fn mg400_msgs__action__JointMovJ_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::JointMovJ_Goal,

}



impl Default for JointMovJ_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_SendGoal_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_SendGoal_Response__init(msg: *mut JointMovJ_SendGoal_Response) -> bool;
    fn mg400_msgs__action__JointMovJ_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Response>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Response>);
    fn mg400_msgs__action__JointMovJ_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_SendGoal_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for JointMovJ_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_SendGoal_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_GetResult_Request__init(msg: *mut JointMovJ_GetResult_Request) -> bool;
    fn mg400_msgs__action__JointMovJ_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Request>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Request>);
    fn mg400_msgs__action__JointMovJ_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for JointMovJ_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_GetResult_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__JointMovJ_GetResult_Response__init(msg: *mut JointMovJ_GetResult_Response) -> bool;
    fn mg400_msgs__action__JointMovJ_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Response>, size: usize) -> bool;
    fn mg400_msgs__action__JointMovJ_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Response>);
    fn mg400_msgs__action__JointMovJ_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_GetResult_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__JointMovJ_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::JointMovJ_Result,

}



impl Default for JointMovJ_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__JointMovJ_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__JointMovJ_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__JointMovJ_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/JointMovJ_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__JointMovJ_GetResult_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_SendGoal_Request__init(msg: *mut MovJ_SendGoal_Request) -> bool;
    fn mg400_msgs__action__MovJ_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_SendGoal_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_SendGoal_Request>);
    fn mg400_msgs__action__MovJ_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_SendGoal_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MovJ_Goal,

}



impl Default for MovJ_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_SendGoal_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_SendGoal_Response__init(msg: *mut MovJ_SendGoal_Response) -> bool;
    fn mg400_msgs__action__MovJ_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_SendGoal_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_SendGoal_Response>);
    fn mg400_msgs__action__MovJ_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_SendGoal_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MovJ_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_SendGoal_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_GetResult_Request__init(msg: *mut MovJ_GetResult_Request) -> bool;
    fn mg400_msgs__action__MovJ_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_GetResult_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_GetResult_Request>);
    fn mg400_msgs__action__MovJ_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_GetResult_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MovJ_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_GetResult_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJ_GetResult_Response__init(msg: *mut MovJ_GetResult_Response) -> bool;
    fn mg400_msgs__action__MovJ_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_GetResult_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovJ_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_GetResult_Response>);
    fn mg400_msgs__action__MovJ_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_GetResult_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJ_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MovJ_Result,

}



impl Default for MovJ_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJ_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJ_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJ_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJ_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJ_GetResult_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_SendGoal_Request__init(msg: *mut MovJIO_SendGoal_Request) -> bool;
    fn mg400_msgs__action__MovJIO_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Request>);
    fn mg400_msgs__action__MovJIO_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MovJIO_Goal,

}



impl Default for MovJIO_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_SendGoal_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_SendGoal_Response__init(msg: *mut MovJIO_SendGoal_Response) -> bool;
    fn mg400_msgs__action__MovJIO_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Response>);
    fn mg400_msgs__action__MovJIO_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_SendGoal_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MovJIO_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_SendGoal_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_GetResult_Request__init(msg: *mut MovJIO_GetResult_Request) -> bool;
    fn mg400_msgs__action__MovJIO_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_GetResult_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_GetResult_Request>);
    fn mg400_msgs__action__MovJIO_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_GetResult_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MovJIO_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_GetResult_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovJIO_GetResult_Response__init(msg: *mut MovJIO_GetResult_Response) -> bool;
    fn mg400_msgs__action__MovJIO_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_GetResult_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovJIO_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_GetResult_Response>);
    fn mg400_msgs__action__MovJIO_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_GetResult_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovJIO_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MovJIO_Result,

}



impl Default for MovJIO_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovJIO_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovJIO_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovJIO_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovJIO_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovJIO_GetResult_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_SendGoal_Request__init(msg: *mut MovL_SendGoal_Request) -> bool;
    fn mg400_msgs__action__MovL_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_SendGoal_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_SendGoal_Request>);
    fn mg400_msgs__action__MovL_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_SendGoal_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MovL_Goal,

}



impl Default for MovL_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_SendGoal_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_SendGoal_Response__init(msg: *mut MovL_SendGoal_Response) -> bool;
    fn mg400_msgs__action__MovL_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_SendGoal_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_SendGoal_Response>);
    fn mg400_msgs__action__MovL_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_SendGoal_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MovL_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_SendGoal_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_GetResult_Request__init(msg: *mut MovL_GetResult_Request) -> bool;
    fn mg400_msgs__action__MovL_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_GetResult_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_GetResult_Request>);
    fn mg400_msgs__action__MovL_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_GetResult_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MovL_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_GetResult_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovL_GetResult_Response__init(msg: *mut MovL_GetResult_Response) -> bool;
    fn mg400_msgs__action__MovL_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_GetResult_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovL_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_GetResult_Response>);
    fn mg400_msgs__action__MovL_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_GetResult_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovL_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MovL_Result,

}



impl Default for MovL_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovL_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovL_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovL_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovL_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovL_GetResult_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_SendGoal_Request__init(msg: *mut MovLIO_SendGoal_Request) -> bool;
    fn mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Request>);
    fn mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MovLIO_Goal,

}



impl Default for MovLIO_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_SendGoal_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_SendGoal_Response__init(msg: *mut MovLIO_SendGoal_Response) -> bool;
    fn mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Response>);
    fn mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_SendGoal_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MovLIO_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_SendGoal_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_GetResult_Request__init(msg: *mut MovLIO_GetResult_Request) -> bool;
    fn mg400_msgs__action__MovLIO_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_GetResult_Request>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_GetResult_Request>);
    fn mg400_msgs__action__MovLIO_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_GetResult_Request>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MovLIO_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_GetResult_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__action__MovLIO_GetResult_Response__init(msg: *mut MovLIO_GetResult_Response) -> bool;
    fn mg400_msgs__action__MovLIO_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_GetResult_Response>, size: usize) -> bool;
    fn mg400_msgs__action__MovLIO_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_GetResult_Response>);
    fn mg400_msgs__action__MovLIO_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_GetResult_Response>) -> bool;
}

// Corresponds to mg400_msgs__action__MovLIO_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MovLIO_Result,

}



impl Default for MovLIO_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__action__MovLIO_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__action__MovLIO_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__action__MovLIO_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/action/MovLIO_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__action__MovLIO_GetResult_Response() }
  }
}






#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__CommandQueue_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__CommandQueue_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct CommandQueue_SendGoal;

impl rosidl_runtime_rs::Service for CommandQueue_SendGoal {
    type Request = CommandQueue_SendGoal_Request;
    type Response = CommandQueue_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__CommandQueue_SendGoal() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__CommandQueue_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__CommandQueue_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct CommandQueue_GetResult;

impl rosidl_runtime_rs::Service for CommandQueue_GetResult {
    type Request = CommandQueue_GetResult_Request;
    type Response = CommandQueue_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__CommandQueue_GetResult() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__JointMovJ_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__JointMovJ_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct JointMovJ_SendGoal;

impl rosidl_runtime_rs::Service for JointMovJ_SendGoal {
    type Request = JointMovJ_SendGoal_Request;
    type Response = JointMovJ_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__JointMovJ_SendGoal() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__JointMovJ_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__JointMovJ_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct JointMovJ_GetResult;

impl rosidl_runtime_rs::Service for JointMovJ_GetResult {
    type Request = JointMovJ_GetResult_Request;
    type Response = JointMovJ_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__JointMovJ_GetResult() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJ_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovJ_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJ_SendGoal;

impl rosidl_runtime_rs::Service for MovJ_SendGoal {
    type Request = MovJ_SendGoal_Request;
    type Response = MovJ_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJ_SendGoal() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJ_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovJ_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJ_GetResult;

impl rosidl_runtime_rs::Service for MovJ_GetResult {
    type Request = MovJ_GetResult_Request;
    type Response = MovJ_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJ_GetResult() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJIO_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovJIO_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJIO_SendGoal;

impl rosidl_runtime_rs::Service for MovJIO_SendGoal {
    type Request = MovJIO_SendGoal_Request;
    type Response = MovJIO_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJIO_SendGoal() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJIO_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovJIO_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJIO_GetResult;

impl rosidl_runtime_rs::Service for MovJIO_GetResult {
    type Request = MovJIO_GetResult_Request;
    type Response = MovJIO_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovJIO_GetResult() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovL_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovL_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MovL_SendGoal;

impl rosidl_runtime_rs::Service for MovL_SendGoal {
    type Request = MovL_SendGoal_Request;
    type Response = MovL_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovL_SendGoal() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovL_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovL_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MovL_GetResult;

impl rosidl_runtime_rs::Service for MovL_GetResult {
    type Request = MovL_GetResult_Request;
    type Response = MovL_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovL_GetResult() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovLIO_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovLIO_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MovLIO_SendGoal;

impl rosidl_runtime_rs::Service for MovLIO_SendGoal {
    type Request = MovLIO_SendGoal_Request;
    type Response = MovLIO_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovLIO_SendGoal() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovLIO_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovLIO_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MovLIO_GetResult;

impl rosidl_runtime_rs::Service for MovLIO_GetResult {
    type Request = MovLIO_GetResult_Request;
    type Response = MovLIO_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__action__MovLIO_GetResult() }
    }
}


