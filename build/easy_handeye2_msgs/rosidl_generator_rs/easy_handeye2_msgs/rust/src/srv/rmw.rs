#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ListAlgorithms_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__ListAlgorithms_Request__init(msg: *mut ListAlgorithms_Request) -> bool;
    fn easy_handeye2_msgs__srv__ListAlgorithms_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListAlgorithms_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__ListAlgorithms_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListAlgorithms_Request>);
    fn easy_handeye2_msgs__srv__ListAlgorithms_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListAlgorithms_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ListAlgorithms_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__ListAlgorithms_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAlgorithms_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListAlgorithms_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__ListAlgorithms_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__ListAlgorithms_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListAlgorithms_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ListAlgorithms_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ListAlgorithms_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ListAlgorithms_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListAlgorithms_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListAlgorithms_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/ListAlgorithms_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ListAlgorithms_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ListAlgorithms_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__ListAlgorithms_Response__init(msg: *mut ListAlgorithms_Response) -> bool;
    fn easy_handeye2_msgs__srv__ListAlgorithms_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ListAlgorithms_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__ListAlgorithms_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ListAlgorithms_Response>);
    fn easy_handeye2_msgs__srv__ListAlgorithms_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ListAlgorithms_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ListAlgorithms_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__ListAlgorithms_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAlgorithms_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub algorithms: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_algorithm: rosidl_runtime_rs::String,

}



impl Default for ListAlgorithms_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__ListAlgorithms_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__ListAlgorithms_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ListAlgorithms_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ListAlgorithms_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ListAlgorithms_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ListAlgorithms_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ListAlgorithms_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ListAlgorithms_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/ListAlgorithms_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ListAlgorithms_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SetAlgorithm_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SetAlgorithm_Request__init(msg: *mut SetAlgorithm_Request) -> bool;
    fn easy_handeye2_msgs__srv__SetAlgorithm_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetAlgorithm_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SetAlgorithm_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetAlgorithm_Request>);
    fn easy_handeye2_msgs__srv__SetAlgorithm_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetAlgorithm_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetAlgorithm_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SetAlgorithm_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAlgorithm_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub new_algorithm: rosidl_runtime_rs::String,

}



impl Default for SetAlgorithm_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SetAlgorithm_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SetAlgorithm_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetAlgorithm_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SetAlgorithm_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SetAlgorithm_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SetAlgorithm_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetAlgorithm_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetAlgorithm_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SetAlgorithm_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SetAlgorithm_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SetAlgorithm_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SetAlgorithm_Response__init(msg: *mut SetAlgorithm_Response) -> bool;
    fn easy_handeye2_msgs__srv__SetAlgorithm_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetAlgorithm_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SetAlgorithm_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetAlgorithm_Response>);
    fn easy_handeye2_msgs__srv__SetAlgorithm_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetAlgorithm_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetAlgorithm_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SetAlgorithm_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAlgorithm_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SetAlgorithm_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SetAlgorithm_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SetAlgorithm_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetAlgorithm_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SetAlgorithm_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SetAlgorithm_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SetAlgorithm_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetAlgorithm_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetAlgorithm_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SetAlgorithm_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SetAlgorithm_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ComputeCalibration_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__ComputeCalibration_Request__init(msg: *mut ComputeCalibration_Request) -> bool;
    fn easy_handeye2_msgs__srv__ComputeCalibration_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCalibration_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__ComputeCalibration_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCalibration_Request>);
    fn easy_handeye2_msgs__srv__ComputeCalibration_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCalibration_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCalibration_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__ComputeCalibration_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCalibration_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ComputeCalibration_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__ComputeCalibration_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__ComputeCalibration_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCalibration_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ComputeCalibration_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ComputeCalibration_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ComputeCalibration_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCalibration_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCalibration_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/ComputeCalibration_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ComputeCalibration_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ComputeCalibration_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__ComputeCalibration_Response__init(msg: *mut ComputeCalibration_Response) -> bool;
    fn easy_handeye2_msgs__srv__ComputeCalibration_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ComputeCalibration_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__ComputeCalibration_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ComputeCalibration_Response>);
    fn easy_handeye2_msgs__srv__ComputeCalibration_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ComputeCalibration_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ComputeCalibration_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__ComputeCalibration_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCalibration_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calibration: super::super::msg::rmw::HandeyeCalibration,

}



impl Default for ComputeCalibration_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__ComputeCalibration_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__ComputeCalibration_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ComputeCalibration_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ComputeCalibration_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ComputeCalibration_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ComputeCalibration_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ComputeCalibration_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ComputeCalibration_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/ComputeCalibration_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ComputeCalibration_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveCalibration_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SaveCalibration_Request__init(msg: *mut SaveCalibration_Request) -> bool;
    fn easy_handeye2_msgs__srv__SaveCalibration_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveCalibration_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SaveCalibration_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveCalibration_Request>);
    fn easy_handeye2_msgs__srv__SaveCalibration_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveCalibration_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveCalibration_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SaveCalibration_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveCalibration_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SaveCalibration_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SaveCalibration_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SaveCalibration_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveCalibration_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveCalibration_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveCalibration_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveCalibration_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveCalibration_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveCalibration_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SaveCalibration_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveCalibration_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveCalibration_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SaveCalibration_Response__init(msg: *mut SaveCalibration_Response) -> bool;
    fn easy_handeye2_msgs__srv__SaveCalibration_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveCalibration_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SaveCalibration_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveCalibration_Response>);
    fn easy_handeye2_msgs__srv__SaveCalibration_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveCalibration_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveCalibration_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SaveCalibration_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveCalibration_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filepath: std_msgs::msg::rmw::String,

}



impl Default for SaveCalibration_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SaveCalibration_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SaveCalibration_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveCalibration_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveCalibration_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveCalibration_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveCalibration_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveCalibration_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveCalibration_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SaveCalibration_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveCalibration_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__RemoveSample_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__RemoveSample_Request__init(msg: *mut RemoveSample_Request) -> bool;
    fn easy_handeye2_msgs__srv__RemoveSample_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RemoveSample_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__RemoveSample_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RemoveSample_Request>);
    fn easy_handeye2_msgs__srv__RemoveSample_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RemoveSample_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RemoveSample_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__RemoveSample_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveSample_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sample_index: i8,

}



impl Default for RemoveSample_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__RemoveSample_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__RemoveSample_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RemoveSample_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__RemoveSample_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__RemoveSample_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__RemoveSample_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RemoveSample_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RemoveSample_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/RemoveSample_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__RemoveSample_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__RemoveSample_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__RemoveSample_Response__init(msg: *mut RemoveSample_Response) -> bool;
    fn easy_handeye2_msgs__srv__RemoveSample_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RemoveSample_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__RemoveSample_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RemoveSample_Response>);
    fn easy_handeye2_msgs__srv__RemoveSample_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RemoveSample_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RemoveSample_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__RemoveSample_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveSample_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: super::super::msg::rmw::SampleList,

}



impl Default for RemoveSample_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__RemoveSample_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__RemoveSample_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RemoveSample_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__RemoveSample_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__RemoveSample_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__RemoveSample_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RemoveSample_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RemoveSample_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/RemoveSample_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__RemoveSample_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__TakeSample_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__TakeSample_Request__init(msg: *mut TakeSample_Request) -> bool;
    fn easy_handeye2_msgs__srv__TakeSample_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TakeSample_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__TakeSample_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TakeSample_Request>);
    fn easy_handeye2_msgs__srv__TakeSample_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TakeSample_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TakeSample_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__TakeSample_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TakeSample_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for TakeSample_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__TakeSample_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__TakeSample_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TakeSample_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__TakeSample_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__TakeSample_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__TakeSample_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TakeSample_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TakeSample_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/TakeSample_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__TakeSample_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__TakeSample_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__TakeSample_Response__init(msg: *mut TakeSample_Response) -> bool;
    fn easy_handeye2_msgs__srv__TakeSample_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TakeSample_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__TakeSample_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TakeSample_Response>);
    fn easy_handeye2_msgs__srv__TakeSample_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TakeSample_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TakeSample_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__TakeSample_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TakeSample_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: super::super::msg::rmw::SampleList,

}



impl Default for TakeSample_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__TakeSample_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__TakeSample_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TakeSample_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__TakeSample_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__TakeSample_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__TakeSample_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TakeSample_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TakeSample_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/TakeSample_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__TakeSample_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveSamples_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SaveSamples_Request__init(msg: *mut SaveSamples_Request) -> bool;
    fn easy_handeye2_msgs__srv__SaveSamples_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveSamples_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SaveSamples_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveSamples_Request>);
    fn easy_handeye2_msgs__srv__SaveSamples_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveSamples_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveSamples_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SaveSamples_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveSamples_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SaveSamples_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SaveSamples_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SaveSamples_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveSamples_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveSamples_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveSamples_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveSamples_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveSamples_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveSamples_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SaveSamples_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveSamples_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveSamples_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SaveSamples_Response__init(msg: *mut SaveSamples_Response) -> bool;
    fn easy_handeye2_msgs__srv__SaveSamples_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveSamples_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SaveSamples_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveSamples_Response>);
    fn easy_handeye2_msgs__srv__SaveSamples_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveSamples_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveSamples_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SaveSamples_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveSamples_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveSamples_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SaveSamples_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SaveSamples_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveSamples_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveSamples_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveSamples_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SaveSamples_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveSamples_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveSamples_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SaveSamples_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SaveSamples_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__LoadSamples_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__LoadSamples_Request__init(msg: *mut LoadSamples_Request) -> bool;
    fn easy_handeye2_msgs__srv__LoadSamples_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadSamples_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__LoadSamples_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadSamples_Request>);
    fn easy_handeye2_msgs__srv__LoadSamples_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadSamples_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadSamples_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__LoadSamples_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSamples_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for LoadSamples_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__LoadSamples_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__LoadSamples_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadSamples_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__LoadSamples_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__LoadSamples_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__LoadSamples_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadSamples_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadSamples_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/LoadSamples_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__LoadSamples_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__LoadSamples_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__LoadSamples_Response__init(msg: *mut LoadSamples_Response) -> bool;
    fn easy_handeye2_msgs__srv__LoadSamples_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoadSamples_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__LoadSamples_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoadSamples_Response>);
    fn easy_handeye2_msgs__srv__LoadSamples_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoadSamples_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoadSamples_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__LoadSamples_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSamples_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: super::super::msg::rmw::SampleList,

}



impl Default for LoadSamples_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__LoadSamples_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__LoadSamples_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoadSamples_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__LoadSamples_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__LoadSamples_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__LoadSamples_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoadSamples_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoadSamples_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/LoadSamples_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__LoadSamples_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__CheckStartingPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__CheckStartingPose_Request__init(msg: *mut CheckStartingPose_Request) -> bool;
    fn easy_handeye2_msgs__srv__CheckStartingPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckStartingPose_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__CheckStartingPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckStartingPose_Request>);
    fn easy_handeye2_msgs__srv__CheckStartingPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckStartingPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckStartingPose_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__CheckStartingPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckStartingPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CheckStartingPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__CheckStartingPose_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__CheckStartingPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckStartingPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__CheckStartingPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__CheckStartingPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__CheckStartingPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckStartingPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckStartingPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/CheckStartingPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__CheckStartingPose_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__CheckStartingPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__CheckStartingPose_Response__init(msg: *mut CheckStartingPose_Response) -> bool;
    fn easy_handeye2_msgs__srv__CheckStartingPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckStartingPose_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__CheckStartingPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckStartingPose_Response>);
    fn easy_handeye2_msgs__srv__CheckStartingPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckStartingPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckStartingPose_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__CheckStartingPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckStartingPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub can_calibrate: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: super::super::msg::rmw::TargetPoseList,

}



impl Default for CheckStartingPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__CheckStartingPose_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__CheckStartingPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckStartingPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__CheckStartingPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__CheckStartingPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__CheckStartingPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckStartingPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckStartingPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/CheckStartingPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__CheckStartingPose_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__EnumerateTargetPoses_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__init(msg: *mut EnumerateTargetPoses_Request) -> bool;
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Request>);
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__EnumerateTargetPoses_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnumerateTargetPoses_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EnumerateTargetPoses_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnumerateTargetPoses_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__EnumerateTargetPoses_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnumerateTargetPoses_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnumerateTargetPoses_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/EnumerateTargetPoses_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__EnumerateTargetPoses_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__EnumerateTargetPoses_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__init(msg: *mut EnumerateTargetPoses_Response) -> bool;
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Response>);
    fn easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EnumerateTargetPoses_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__EnumerateTargetPoses_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnumerateTargetPoses_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: super::super::msg::rmw::TargetPoseList,

}



impl Default for EnumerateTargetPoses_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnumerateTargetPoses_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__EnumerateTargetPoses_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnumerateTargetPoses_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnumerateTargetPoses_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/EnumerateTargetPoses_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__EnumerateTargetPoses_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SelectTargetPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SelectTargetPose_Request__init(msg: *mut SelectTargetPose_Request) -> bool;
    fn easy_handeye2_msgs__srv__SelectTargetPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SelectTargetPose_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SelectTargetPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SelectTargetPose_Request>);
    fn easy_handeye2_msgs__srv__SelectTargetPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SelectTargetPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SelectTargetPose_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SelectTargetPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectTargetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_pose_index: i64,

}



impl Default for SelectTargetPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SelectTargetPose_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SelectTargetPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SelectTargetPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SelectTargetPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SelectTargetPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SelectTargetPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SelectTargetPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SelectTargetPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SelectTargetPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SelectTargetPose_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SelectTargetPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__SelectTargetPose_Response__init(msg: *mut SelectTargetPose_Response) -> bool;
    fn easy_handeye2_msgs__srv__SelectTargetPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SelectTargetPose_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__SelectTargetPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SelectTargetPose_Response>);
    fn easy_handeye2_msgs__srv__SelectTargetPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SelectTargetPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SelectTargetPose_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__SelectTargetPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectTargetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: super::super::msg::rmw::TargetPoseList,

}



impl Default for SelectTargetPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__SelectTargetPose_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__SelectTargetPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SelectTargetPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SelectTargetPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SelectTargetPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__SelectTargetPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SelectTargetPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SelectTargetPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/SelectTargetPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__SelectTargetPose_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__init(msg: *mut PlanToSelectedTargetPose_Request) -> bool;
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Request>);
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanToSelectedTargetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PlanToSelectedTargetPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanToSelectedTargetPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanToSelectedTargetPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanToSelectedTargetPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/PlanToSelectedTargetPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__init(msg: *mut PlanToSelectedTargetPose_Response) -> bool;
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Response>);
    fn easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PlanToSelectedTargetPose_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanToSelectedTargetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for PlanToSelectedTargetPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlanToSelectedTargetPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlanToSelectedTargetPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlanToSelectedTargetPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/PlanToSelectedTargetPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ExecutePlan_Request() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__ExecutePlan_Request__init(msg: *mut ExecutePlan_Request) -> bool;
    fn easy_handeye2_msgs__srv__ExecutePlan_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecutePlan_Request>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__ExecutePlan_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecutePlan_Request>);
    fn easy_handeye2_msgs__srv__ExecutePlan_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecutePlan_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecutePlan_Request>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__ExecutePlan_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecutePlan_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ExecutePlan_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__ExecutePlan_Request__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__ExecutePlan_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecutePlan_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ExecutePlan_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ExecutePlan_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ExecutePlan_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecutePlan_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecutePlan_Request where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/ExecutePlan_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ExecutePlan_Request() }
  }
}


#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ExecutePlan_Response() -> *const std::ffi::c_void;
}

#[link(name = "easy_handeye2_msgs__rosidl_generator_c")]
extern "C" {
    fn easy_handeye2_msgs__srv__ExecutePlan_Response__init(msg: *mut ExecutePlan_Response) -> bool;
    fn easy_handeye2_msgs__srv__ExecutePlan_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecutePlan_Response>, size: usize) -> bool;
    fn easy_handeye2_msgs__srv__ExecutePlan_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecutePlan_Response>);
    fn easy_handeye2_msgs__srv__ExecutePlan_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecutePlan_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecutePlan_Response>) -> bool;
}

// Corresponds to easy_handeye2_msgs__srv__ExecutePlan_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecutePlan_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ExecutePlan_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !easy_handeye2_msgs__srv__ExecutePlan_Response__init(&mut msg as *mut _) {
        panic!("Call to easy_handeye2_msgs__srv__ExecutePlan_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecutePlan_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ExecutePlan_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ExecutePlan_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { easy_handeye2_msgs__srv__ExecutePlan_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecutePlan_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecutePlan_Response where Self: Sized {
  const TYPE_NAME: &'static str = "easy_handeye2_msgs/srv/ExecutePlan_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__easy_handeye2_msgs__srv__ExecutePlan_Response() }
  }
}






#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__ListAlgorithms() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__ListAlgorithms
#[allow(missing_docs, non_camel_case_types)]
pub struct ListAlgorithms;

impl rosidl_runtime_rs::Service for ListAlgorithms {
    type Request = ListAlgorithms_Request;
    type Response = ListAlgorithms_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__ListAlgorithms() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SetAlgorithm() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__SetAlgorithm
#[allow(missing_docs, non_camel_case_types)]
pub struct SetAlgorithm;

impl rosidl_runtime_rs::Service for SetAlgorithm {
    type Request = SetAlgorithm_Request;
    type Response = SetAlgorithm_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SetAlgorithm() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__ComputeCalibration() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__ComputeCalibration
#[allow(missing_docs, non_camel_case_types)]
pub struct ComputeCalibration;

impl rosidl_runtime_rs::Service for ComputeCalibration {
    type Request = ComputeCalibration_Request;
    type Response = ComputeCalibration_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__ComputeCalibration() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SaveCalibration() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__SaveCalibration
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveCalibration;

impl rosidl_runtime_rs::Service for SaveCalibration {
    type Request = SaveCalibration_Request;
    type Response = SaveCalibration_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SaveCalibration() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__RemoveSample() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__RemoveSample
#[allow(missing_docs, non_camel_case_types)]
pub struct RemoveSample;

impl rosidl_runtime_rs::Service for RemoveSample {
    type Request = RemoveSample_Request;
    type Response = RemoveSample_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__RemoveSample() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__TakeSample() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__TakeSample
#[allow(missing_docs, non_camel_case_types)]
pub struct TakeSample;

impl rosidl_runtime_rs::Service for TakeSample {
    type Request = TakeSample_Request;
    type Response = TakeSample_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__TakeSample() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SaveSamples() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__SaveSamples
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveSamples;

impl rosidl_runtime_rs::Service for SaveSamples {
    type Request = SaveSamples_Request;
    type Response = SaveSamples_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SaveSamples() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__LoadSamples() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__LoadSamples
#[allow(missing_docs, non_camel_case_types)]
pub struct LoadSamples;

impl rosidl_runtime_rs::Service for LoadSamples {
    type Request = LoadSamples_Request;
    type Response = LoadSamples_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__LoadSamples() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__CheckStartingPose() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__CheckStartingPose
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckStartingPose;

impl rosidl_runtime_rs::Service for CheckStartingPose {
    type Request = CheckStartingPose_Request;
    type Response = CheckStartingPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__CheckStartingPose() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__EnumerateTargetPoses() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__EnumerateTargetPoses
#[allow(missing_docs, non_camel_case_types)]
pub struct EnumerateTargetPoses;

impl rosidl_runtime_rs::Service for EnumerateTargetPoses {
    type Request = EnumerateTargetPoses_Request;
    type Response = EnumerateTargetPoses_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__EnumerateTargetPoses() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SelectTargetPose() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__SelectTargetPose
#[allow(missing_docs, non_camel_case_types)]
pub struct SelectTargetPose;

impl rosidl_runtime_rs::Service for SelectTargetPose {
    type Request = SelectTargetPose_Request;
    type Response = SelectTargetPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__SelectTargetPose() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__PlanToSelectedTargetPose() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__PlanToSelectedTargetPose
#[allow(missing_docs, non_camel_case_types)]
pub struct PlanToSelectedTargetPose;

impl rosidl_runtime_rs::Service for PlanToSelectedTargetPose {
    type Request = PlanToSelectedTargetPose_Request;
    type Response = PlanToSelectedTargetPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__PlanToSelectedTargetPose() }
    }
}




#[link(name = "easy_handeye2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__ExecutePlan() -> *const std::ffi::c_void;
}

// Corresponds to easy_handeye2_msgs__srv__ExecutePlan
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecutePlan;

impl rosidl_runtime_rs::Service for ExecutePlan {
    type Request = ExecutePlan_Request;
    type Response = ExecutePlan_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__easy_handeye2_msgs__srv__ExecutePlan() }
    }
}


