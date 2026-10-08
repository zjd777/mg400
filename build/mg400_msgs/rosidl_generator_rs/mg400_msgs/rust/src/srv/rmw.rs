#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__AccJ_Request__init(msg: *mut AccJ_Request) -> bool;
    fn mg400_msgs__srv__AccJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__AccJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Request>);
    fn mg400_msgs__srv__AccJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AccJ_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__AccJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for AccJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__AccJ_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__AccJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/AccJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccJ_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__AccJ_Response__init(msg: *mut AccJ_Response) -> bool;
    fn mg400_msgs__srv__AccJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__AccJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Response>);
    fn mg400_msgs__srv__AccJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AccJ_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__AccJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for AccJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__AccJ_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__AccJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/AccJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccJ_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccL_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__AccL_Request__init(msg: *mut AccL_Request) -> bool;
    fn mg400_msgs__srv__AccL_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccL_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__AccL_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccL_Request>);
    fn mg400_msgs__srv__AccL_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccL_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AccL_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__AccL_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for AccL_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__AccL_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__AccL_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccL_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccL_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccL_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccL_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccL_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccL_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/AccL_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccL_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccL_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__AccL_Response__init(msg: *mut AccL_Response) -> bool;
    fn mg400_msgs__srv__AccL_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccL_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__AccL_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccL_Response>);
    fn mg400_msgs__srv__AccL_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccL_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AccL_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__AccL_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for AccL_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__AccL_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__AccL_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccL_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccL_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccL_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__AccL_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccL_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccL_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/AccL_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__AccL_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Arch_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__Arch_Request__init(msg: *mut Arch_Request) -> bool;
    fn mg400_msgs__srv__Arch_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arch_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__Arch_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arch_Request>);
    fn mg400_msgs__srv__Arch_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arch_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Arch_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__Arch_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arch_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::Arch,

}



impl Default for Arch_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__Arch_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__Arch_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arch_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Arch_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Arch_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Arch_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arch_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arch_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/Arch_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Arch_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Arch_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__Arch_Response__init(msg: *mut Arch_Response) -> bool;
    fn mg400_msgs__srv__Arch_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arch_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__Arch_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arch_Response>);
    fn mg400_msgs__srv__Arch_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arch_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Arch_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__Arch_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arch_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for Arch_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__Arch_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__Arch_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arch_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Arch_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Arch_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Arch_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arch_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arch_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/Arch_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Arch_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__CP_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__CP_Request__init(msg: *mut CP_Request) -> bool;
    fn mg400_msgs__srv__CP_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CP_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__CP_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CP_Request>);
    fn mg400_msgs__srv__CP_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CP_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CP_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__CP_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u16,

}



impl Default for CP_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__CP_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__CP_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CP_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__CP_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__CP_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__CP_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CP_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CP_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/CP_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__CP_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__CP_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__CP_Response__init(msg: *mut CP_Response) -> bool;
    fn mg400_msgs__srv__CP_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CP_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__CP_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CP_Response>);
    fn mg400_msgs__srv__CP_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CP_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CP_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__CP_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for CP_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__CP_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__CP_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CP_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__CP_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__CP_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__CP_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CP_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CP_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/CP_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__CP_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ClearError_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ClearError_Request__init(msg: *mut ClearError_Request) -> bool;
    fn mg400_msgs__srv__ClearError_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__ClearError_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Request>);
    fn mg400_msgs__srv__ClearError_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearError_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearError_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__ClearError_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ClearError_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ClearError_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ClearError_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearError_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ClearError_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ClearError_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ClearError_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearError_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearError_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ClearError_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ClearError_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ClearError_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ClearError_Response__init(msg: *mut ClearError_Response) -> bool;
    fn mg400_msgs__srv__ClearError_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__ClearError_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Response>);
    fn mg400_msgs__srv__ClearError_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearError_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearError_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__ClearError_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for ClearError_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ClearError_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ClearError_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearError_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ClearError_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ClearError_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ClearError_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearError_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearError_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ClearError_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ClearError_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DI_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__DI_Request__init(msg: *mut DI_Request) -> bool;
    fn mg400_msgs__srv__DI_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DI_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__DI_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DI_Request>);
    fn mg400_msgs__srv__DI_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DI_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DI_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__DI_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::DIIndex,

}



impl Default for DI_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__DI_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__DI_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DI_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DI_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DI_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DI_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DI_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DI_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/DI_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DI_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DI_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__DI_Response__init(msg: *mut DI_Response) -> bool;
    fn mg400_msgs__srv__DI_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DI_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__DI_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DI_Response>);
    fn mg400_msgs__srv__DI_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DI_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DI_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__DI_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for DI_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__DI_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__DI_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DI_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DI_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DI_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DI_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DI_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DI_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/DI_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DI_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DO_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__DO_Request__init(msg: *mut DO_Request) -> bool;
    fn mg400_msgs__srv__DO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DO_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__DO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DO_Request>);
    fn mg400_msgs__srv__DO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DO_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__DO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::DOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::DOStatus,

}



impl Default for DO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__DO_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__DO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/DO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DO_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DO_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__DO_Response__init(msg: *mut DO_Response) -> bool;
    fn mg400_msgs__srv__DO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DO_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__DO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DO_Response>);
    fn mg400_msgs__srv__DO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DO_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__DO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for DO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__DO_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__DO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/DO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DO_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DisableRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__DisableRobot_Request__init(msg: *mut DisableRobot_Request) -> bool;
    fn mg400_msgs__srv__DisableRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__DisableRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Request>);
    fn mg400_msgs__srv__DisableRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DisableRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__DisableRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DisableRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__DisableRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__DisableRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DisableRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DisableRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DisableRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DisableRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DisableRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/DisableRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DisableRobot_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DisableRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__DisableRobot_Response__init(msg: *mut DisableRobot_Response) -> bool;
    fn mg400_msgs__srv__DisableRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__DisableRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Response>);
    fn mg400_msgs__srv__DisableRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DisableRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__DisableRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for DisableRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__DisableRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__DisableRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DisableRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DisableRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DisableRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__DisableRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DisableRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/DisableRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__DisableRobot_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EmergencyStop_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__EmergencyStop_Request__init(msg: *mut EmergencyStop_Request) -> bool;
    fn mg400_msgs__srv__EmergencyStop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__EmergencyStop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Request>);
    fn mg400_msgs__srv__EmergencyStop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EmergencyStop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__EmergencyStop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EmergencyStop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__EmergencyStop_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__EmergencyStop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EmergencyStop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EmergencyStop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EmergencyStop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EmergencyStop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EmergencyStop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/EmergencyStop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EmergencyStop_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EmergencyStop_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__EmergencyStop_Response__init(msg: *mut EmergencyStop_Response) -> bool;
    fn mg400_msgs__srv__EmergencyStop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__EmergencyStop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Response>);
    fn mg400_msgs__srv__EmergencyStop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EmergencyStop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__EmergencyStop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for EmergencyStop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__EmergencyStop_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__EmergencyStop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EmergencyStop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EmergencyStop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EmergencyStop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EmergencyStop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EmergencyStop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/EmergencyStop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EmergencyStop_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EnableRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__EnableRobot_Request__init(msg: *mut EnableRobot_Request) -> bool;
    fn mg400_msgs__srv__EnableRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__EnableRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Request>);
    fn mg400_msgs__srv__EnableRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__EnableRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub num_of_params: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub load: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub center_x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub center_y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub center_z: f64,

}

impl EnableRobot_Request {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NO_PARAM: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ONE_PARAM: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FOUR_PARAM: i8 = 4;

}


impl Default for EnableRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__EnableRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__EnableRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EnableRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EnableRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EnableRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/EnableRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EnableRobot_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EnableRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__EnableRobot_Response__init(msg: *mut EnableRobot_Response) -> bool;
    fn mg400_msgs__srv__EnableRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__EnableRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Response>);
    fn mg400_msgs__srv__EnableRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__EnableRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for EnableRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__EnableRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__EnableRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EnableRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EnableRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__EnableRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/EnableRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__EnableRobot_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetAngle_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__GetAngle_Request__init(msg: *mut GetAngle_Request) -> bool;
    fn mg400_msgs__srv__GetAngle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__GetAngle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Request>);
    fn mg400_msgs__srv__GetAngle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAngle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__GetAngle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetAngle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__GetAngle_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__GetAngle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAngle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetAngle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetAngle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetAngle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAngle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/GetAngle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetAngle_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetAngle_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__GetAngle_Response__init(msg: *mut GetAngle_Response) -> bool;
    fn mg400_msgs__srv__GetAngle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__GetAngle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Response>);
    fn mg400_msgs__srv__GetAngle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAngle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__GetAngle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint6: f64,

}



impl Default for GetAngle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__GetAngle_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__GetAngle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAngle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetAngle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetAngle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetAngle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAngle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/GetAngle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetAngle_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetErrorID_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__GetErrorID_Request__init(msg: *mut GetErrorID_Request) -> bool;
    fn mg400_msgs__srv__GetErrorID_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__GetErrorID_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Request>);
    fn mg400_msgs__srv__GetErrorID_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetErrorID_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__GetErrorID_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetErrorID_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__GetErrorID_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__GetErrorID_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetErrorID_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetErrorID_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetErrorID_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetErrorID_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetErrorID_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/GetErrorID_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetErrorID_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetErrorID_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__GetErrorID_Response__init(msg: *mut GetErrorID_Response) -> bool;
    fn mg400_msgs__srv__GetErrorID_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__GetErrorID_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Response>);
    fn mg400_msgs__srv__GetErrorID_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetErrorID_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__GetErrorID_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_ids: super::super::msg::rmw::ErrorID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for GetErrorID_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__GetErrorID_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__GetErrorID_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetErrorID_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetErrorID_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetErrorID_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetErrorID_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetErrorID_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/GetErrorID_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetErrorID_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__GetPose_Request__init(msg: *mut GetPose_Request) -> bool;
    fn mg400_msgs__srv__GetPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__GetPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Request>);
    fn mg400_msgs__srv__GetPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPose_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__GetPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__GetPose_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__GetPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/GetPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetPose_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__GetPose_Response__init(msg: *mut GetPose_Response) -> bool;
    fn mg400_msgs__srv__GetPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__GetPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Response>);
    fn mg400_msgs__srv__GetPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPose_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__GetPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose6: f64,

}



impl Default for GetPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__GetPose_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__GetPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__GetPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/GetPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__GetPose_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__InverseSolution_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__InverseSolution_Request__init(msg: *mut InverseSolution_Request) -> bool;
    fn mg400_msgs__srv__InverseSolution_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__InverseSolution_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Request>);
    fn mg400_msgs__srv__InverseSolution_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InverseSolution_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__InverseSolution_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseSolution_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub r: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: u8,

}



impl Default for InverseSolution_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__InverseSolution_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__InverseSolution_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InverseSolution_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__InverseSolution_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__InverseSolution_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__InverseSolution_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InverseSolution_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/InverseSolution_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__InverseSolution_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__InverseSolution_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__InverseSolution_Response__init(msg: *mut InverseSolution_Response) -> bool;
    fn mg400_msgs__srv__InverseSolution_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__InverseSolution_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Response>);
    fn mg400_msgs__srv__InverseSolution_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InverseSolution_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__InverseSolution_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseSolution_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j4: f64,

}



impl Default for InverseSolution_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__InverseSolution_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__InverseSolution_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InverseSolution_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__InverseSolution_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__InverseSolution_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__InverseSolution_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InverseSolution_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/InverseSolution_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__InverseSolution_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__JointMovJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__JointMovJ_Request__init(msg: *mut JointMovJ_Request) -> bool;
    fn mg400_msgs__srv__JointMovJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__JointMovJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Request>);
    fn mg400_msgs__srv__JointMovJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__JointMovJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j6: f64,

}



impl Default for JointMovJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__JointMovJ_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__JointMovJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__JointMovJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__JointMovJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__JointMovJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/JointMovJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__JointMovJ_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__JointMovJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__JointMovJ_Response__init(msg: *mut JointMovJ_Response) -> bool;
    fn mg400_msgs__srv__JointMovJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__JointMovJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Response>);
    fn mg400_msgs__srv__JointMovJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__JointMovJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for JointMovJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__JointMovJ_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__JointMovJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__JointMovJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__JointMovJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__JointMovJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/JointMovJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__JointMovJ_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__MoveJog_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__MoveJog_Request__init(msg: *mut MoveJog_Request) -> bool;
    fn mg400_msgs__srv__MoveJog_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__MoveJog_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Request>);
    fn mg400_msgs__srv__MoveJog_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveJog_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__MoveJog_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub jog: super::super::msg::rmw::MoveJog,

}



impl Default for MoveJog_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__MoveJog_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__MoveJog_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveJog_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__MoveJog_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__MoveJog_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__MoveJog_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveJog_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/MoveJog_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__MoveJog_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__MoveJog_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__MoveJog_Response__init(msg: *mut MoveJog_Response) -> bool;
    fn mg400_msgs__srv__MoveJog_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__MoveJog_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Response>);
    fn mg400_msgs__srv__MoveJog_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveJog_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__MoveJog_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for MoveJog_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__MoveJog_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__MoveJog_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveJog_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__MoveJog_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__MoveJog_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__MoveJog_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveJog_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/MoveJog_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__MoveJog_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PayLoad_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__PayLoad_Request__init(msg: *mut PayLoad_Request) -> bool;
    fn mg400_msgs__srv__PayLoad_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PayLoad_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__PayLoad_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PayLoad_Request>);
    fn mg400_msgs__srv__PayLoad_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PayLoad_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PayLoad_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__PayLoad_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PayLoad_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub weight: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub inertia: f64,

}



impl Default for PayLoad_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__PayLoad_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__PayLoad_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PayLoad_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PayLoad_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PayLoad_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PayLoad_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PayLoad_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PayLoad_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/PayLoad_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PayLoad_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PayLoad_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__PayLoad_Response__init(msg: *mut PayLoad_Response) -> bool;
    fn mg400_msgs__srv__PayLoad_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PayLoad_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__PayLoad_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PayLoad_Response>);
    fn mg400_msgs__srv__PayLoad_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PayLoad_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PayLoad_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__PayLoad_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PayLoad_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for PayLoad_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__PayLoad_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__PayLoad_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PayLoad_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PayLoad_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PayLoad_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PayLoad_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PayLoad_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PayLoad_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/PayLoad_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PayLoad_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PositiveSolution_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__PositiveSolution_Request__init(msg: *mut PositiveSolution_Request) -> bool;
    fn mg400_msgs__srv__PositiveSolution_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PositiveSolution_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__PositiveSolution_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PositiveSolution_Request>);
    fn mg400_msgs__srv__PositiveSolution_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PositiveSolution_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PositiveSolution_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__PositiveSolution_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositiveSolution_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: u8,

}



impl Default for PositiveSolution_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__PositiveSolution_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__PositiveSolution_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PositiveSolution_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PositiveSolution_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PositiveSolution_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PositiveSolution_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PositiveSolution_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PositiveSolution_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/PositiveSolution_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PositiveSolution_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PositiveSolution_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__PositiveSolution_Response__init(msg: *mut PositiveSolution_Response) -> bool;
    fn mg400_msgs__srv__PositiveSolution_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PositiveSolution_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__PositiveSolution_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PositiveSolution_Response>);
    fn mg400_msgs__srv__PositiveSolution_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PositiveSolution_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PositiveSolution_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__PositiveSolution_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositiveSolution_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub r: f64,

}



impl Default for PositiveSolution_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__PositiveSolution_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__PositiveSolution_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PositiveSolution_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PositiveSolution_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PositiveSolution_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__PositiveSolution_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PositiveSolution_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PositiveSolution_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/PositiveSolution_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__PositiveSolution_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ResetRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ResetRobot_Request__init(msg: *mut ResetRobot_Request) -> bool;
    fn mg400_msgs__srv__ResetRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__ResetRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Request>);
    fn mg400_msgs__srv__ResetRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__ResetRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ResetRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ResetRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ResetRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ResetRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ResetRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ResetRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ResetRobot_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ResetRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ResetRobot_Response__init(msg: *mut ResetRobot_Response) -> bool;
    fn mg400_msgs__srv__ResetRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__ResetRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Response>);
    fn mg400_msgs__srv__ResetRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__ResetRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for ResetRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ResetRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ResetRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ResetRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ResetRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ResetRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ResetRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ResetRobot_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__RobotMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__RobotMode_Request__init(msg: *mut RobotMode_Request) -> bool;
    fn mg400_msgs__srv__RobotMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__RobotMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Request>);
    fn mg400_msgs__srv__RobotMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__RobotMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RobotMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__RobotMode_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__RobotMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__RobotMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__RobotMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__RobotMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/RobotMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__RobotMode_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__RobotMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__RobotMode_Response__init(msg: *mut RobotMode_Response) -> bool;
    fn mg400_msgs__srv__RobotMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__RobotMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Response>);
    fn mg400_msgs__srv__RobotMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__RobotMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_mode: super::super::msg::rmw::RobotMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for RobotMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__RobotMode_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__RobotMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__RobotMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__RobotMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__RobotMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/RobotMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__RobotMode_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SetCollisionLevel_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SetCollisionLevel_Request__init(msg: *mut SetCollisionLevel_Request) -> bool;
    fn mg400_msgs__srv__SetCollisionLevel_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__SetCollisionLevel_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>);
    fn mg400_msgs__srv__SetCollisionLevel_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__SetCollisionLevel_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub level: super::super::msg::rmw::CollisionLevel,

}



impl Default for SetCollisionLevel_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SetCollisionLevel_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SetCollisionLevel_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCollisionLevel_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SetCollisionLevel_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SetCollisionLevel_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SetCollisionLevel_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCollisionLevel_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SetCollisionLevel_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SetCollisionLevel_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SetCollisionLevel_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SetCollisionLevel_Response__init(msg: *mut SetCollisionLevel_Response) -> bool;
    fn mg400_msgs__srv__SetCollisionLevel_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__SetCollisionLevel_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>);
    fn mg400_msgs__srv__SetCollisionLevel_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__SetCollisionLevel_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for SetCollisionLevel_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SetCollisionLevel_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SetCollisionLevel_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCollisionLevel_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SetCollisionLevel_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SetCollisionLevel_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SetCollisionLevel_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCollisionLevel_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SetCollisionLevel_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SetCollisionLevel_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedFactor_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SpeedFactor_Request__init(msg: *mut SpeedFactor_Request) -> bool;
    fn mg400_msgs__srv__SpeedFactor_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__SpeedFactor_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Request>);
    fn mg400_msgs__srv__SpeedFactor_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedFactor_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__SpeedFactor_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ratio: u8,

}



impl Default for SpeedFactor_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SpeedFactor_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SpeedFactor_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedFactor_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedFactor_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedFactor_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedFactor_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedFactor_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SpeedFactor_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedFactor_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedFactor_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SpeedFactor_Response__init(msg: *mut SpeedFactor_Response) -> bool;
    fn mg400_msgs__srv__SpeedFactor_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__SpeedFactor_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Response>);
    fn mg400_msgs__srv__SpeedFactor_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedFactor_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__SpeedFactor_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for SpeedFactor_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SpeedFactor_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SpeedFactor_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedFactor_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedFactor_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedFactor_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedFactor_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedFactor_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SpeedFactor_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedFactor_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SpeedJ_Request__init(msg: *mut SpeedJ_Request) -> bool;
    fn mg400_msgs__srv__SpeedJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedJ_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__SpeedJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedJ_Request>);
    fn mg400_msgs__srv__SpeedJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedJ_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__SpeedJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for SpeedJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SpeedJ_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SpeedJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SpeedJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedJ_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SpeedJ_Response__init(msg: *mut SpeedJ_Response) -> bool;
    fn mg400_msgs__srv__SpeedJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedJ_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__SpeedJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedJ_Response>);
    fn mg400_msgs__srv__SpeedJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedJ_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__SpeedJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for SpeedJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SpeedJ_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SpeedJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SpeedJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedJ_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedL_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SpeedL_Request__init(msg: *mut SpeedL_Request) -> bool;
    fn mg400_msgs__srv__SpeedL_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedL_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__SpeedL_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedL_Request>);
    fn mg400_msgs__srv__SpeedL_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedL_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedL_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__SpeedL_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for SpeedL_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SpeedL_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SpeedL_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedL_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedL_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedL_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedL_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedL_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedL_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SpeedL_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedL_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedL_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__SpeedL_Response__init(msg: *mut SpeedL_Response) -> bool;
    fn mg400_msgs__srv__SpeedL_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedL_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__SpeedL_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedL_Response>);
    fn mg400_msgs__srv__SpeedL_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedL_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedL_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__SpeedL_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for SpeedL_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__SpeedL_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__SpeedL_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedL_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedL_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedL_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__SpeedL_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedL_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedL_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/SpeedL_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__SpeedL_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Tool_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__Tool_Request__init(msg: *mut Tool_Request) -> bool;
    fn mg400_msgs__srv__Tool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Tool_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__Tool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Tool_Request>);
    fn mg400_msgs__srv__Tool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Tool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Tool_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__Tool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: super::super::msg::rmw::Tool,

}



impl Default for Tool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__Tool_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__Tool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Tool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Tool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Tool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Tool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Tool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Tool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/Tool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Tool_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Tool_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__Tool_Response__init(msg: *mut Tool_Response) -> bool;
    fn mg400_msgs__srv__Tool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Tool_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__Tool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Tool_Response>);
    fn mg400_msgs__srv__Tool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Tool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Tool_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__Tool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for Tool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__Tool_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__Tool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Tool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Tool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Tool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__Tool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Tool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Tool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/Tool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__Tool_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDI_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ToolDI_Request__init(msg: *mut ToolDI_Request) -> bool;
    fn mg400_msgs__srv__ToolDI_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__ToolDI_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Request>);
    fn mg400_msgs__srv__ToolDI_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDI_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__ToolDI_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::ToolDIIndex,

}



impl Default for ToolDI_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ToolDI_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ToolDI_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDI_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDI_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDI_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDI_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDI_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ToolDI_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDI_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDI_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ToolDI_Response__init(msg: *mut ToolDI_Response) -> bool;
    fn mg400_msgs__srv__ToolDI_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__ToolDI_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Response>);
    fn mg400_msgs__srv__ToolDI_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDI_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__ToolDI_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for ToolDI_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ToolDI_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ToolDI_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDI_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDI_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDI_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDI_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDI_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ToolDI_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDI_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDOExecute_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ToolDOExecute_Request__init(msg: *mut ToolDOExecute_Request) -> bool;
    fn mg400_msgs__srv__ToolDOExecute_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDOExecute_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__ToolDOExecute_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDOExecute_Request>);
    fn mg400_msgs__srv__ToolDOExecute_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDOExecute_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDOExecute_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__ToolDOExecute_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOExecute_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::super::msg::rmw::ToolDOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::DOStatus,

}



impl Default for ToolDOExecute_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ToolDOExecute_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ToolDOExecute_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDOExecute_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDOExecute_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDOExecute_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDOExecute_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDOExecute_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDOExecute_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ToolDOExecute_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDOExecute_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDOExecute_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__ToolDOExecute_Response__init(msg: *mut ToolDOExecute_Response) -> bool;
    fn mg400_msgs__srv__ToolDOExecute_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDOExecute_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__ToolDOExecute_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDOExecute_Response>);
    fn mg400_msgs__srv__ToolDOExecute_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDOExecute_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDOExecute_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__ToolDOExecute_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOExecute_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for ToolDOExecute_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__ToolDOExecute_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__ToolDOExecute_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDOExecute_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDOExecute_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDOExecute_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__ToolDOExecute_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDOExecute_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDOExecute_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/ToolDOExecute_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__ToolDOExecute_Response() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__User_Request() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__User_Request__init(msg: *mut User_Request) -> bool;
    fn mg400_msgs__srv__User_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<User_Request>, size: usize) -> bool;
    fn mg400_msgs__srv__User_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<User_Request>);
    fn mg400_msgs__srv__User_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<User_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<User_Request>) -> bool;
}

// Corresponds to mg400_msgs__srv__User_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user: super::super::msg::rmw::User,

}



impl Default for User_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__User_Request__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__User_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for User_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__User_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__User_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__User_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for User_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for User_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/User_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__User_Request() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__User_Response() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__srv__User_Response__init(msg: *mut User_Response) -> bool;
    fn mg400_msgs__srv__User_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<User_Response>, size: usize) -> bool;
    fn mg400_msgs__srv__User_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<User_Response>);
    fn mg400_msgs__srv__User_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<User_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<User_Response>) -> bool;
}

// Corresponds to mg400_msgs__srv__User_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for User_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__srv__User_Response__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__srv__User_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for User_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__User_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__User_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__srv__User_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for User_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for User_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/srv/User_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__srv__User_Response() }
  }
}






#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__AccJ() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__AccJ
#[allow(missing_docs, non_camel_case_types)]
pub struct AccJ;

impl rosidl_runtime_rs::Service for AccJ {
    type Request = AccJ_Request;
    type Response = AccJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__AccJ() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__AccL() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__AccL
#[allow(missing_docs, non_camel_case_types)]
pub struct AccL;

impl rosidl_runtime_rs::Service for AccL {
    type Request = AccL_Request;
    type Response = AccL_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__AccL() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__Arch() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__Arch
#[allow(missing_docs, non_camel_case_types)]
pub struct Arch;

impl rosidl_runtime_rs::Service for Arch {
    type Request = Arch_Request;
    type Response = Arch_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__Arch() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__CP() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__CP
#[allow(missing_docs, non_camel_case_types)]
pub struct CP;

impl rosidl_runtime_rs::Service for CP {
    type Request = CP_Request;
    type Response = CP_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__CP() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ClearError() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__ClearError
#[allow(missing_docs, non_camel_case_types)]
pub struct ClearError;

impl rosidl_runtime_rs::Service for ClearError {
    type Request = ClearError_Request;
    type Response = ClearError_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ClearError() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__DI() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__DI
#[allow(missing_docs, non_camel_case_types)]
pub struct DI;

impl rosidl_runtime_rs::Service for DI {
    type Request = DI_Request;
    type Response = DI_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__DI() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__DO() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__DO
#[allow(missing_docs, non_camel_case_types)]
pub struct DO;

impl rosidl_runtime_rs::Service for DO {
    type Request = DO_Request;
    type Response = DO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__DO() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__DisableRobot() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__DisableRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct DisableRobot;

impl rosidl_runtime_rs::Service for DisableRobot {
    type Request = DisableRobot_Request;
    type Response = DisableRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__DisableRobot() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__EmergencyStop() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__EmergencyStop
#[allow(missing_docs, non_camel_case_types)]
pub struct EmergencyStop;

impl rosidl_runtime_rs::Service for EmergencyStop {
    type Request = EmergencyStop_Request;
    type Response = EmergencyStop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__EmergencyStop() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__EnableRobot() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__EnableRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct EnableRobot;

impl rosidl_runtime_rs::Service for EnableRobot {
    type Request = EnableRobot_Request;
    type Response = EnableRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__EnableRobot() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__GetAngle() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__GetAngle
#[allow(missing_docs, non_camel_case_types)]
pub struct GetAngle;

impl rosidl_runtime_rs::Service for GetAngle {
    type Request = GetAngle_Request;
    type Response = GetAngle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__GetAngle() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__GetErrorID() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__GetErrorID
#[allow(missing_docs, non_camel_case_types)]
pub struct GetErrorID;

impl rosidl_runtime_rs::Service for GetErrorID {
    type Request = GetErrorID_Request;
    type Response = GetErrorID_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__GetErrorID() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__GetPose() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__GetPose
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPose;

impl rosidl_runtime_rs::Service for GetPose {
    type Request = GetPose_Request;
    type Response = GetPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__GetPose() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__InverseSolution() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__InverseSolution
#[allow(missing_docs, non_camel_case_types)]
pub struct InverseSolution;

impl rosidl_runtime_rs::Service for InverseSolution {
    type Request = InverseSolution_Request;
    type Response = InverseSolution_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__InverseSolution() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__JointMovJ() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__JointMovJ
#[allow(missing_docs, non_camel_case_types)]
pub struct JointMovJ;

impl rosidl_runtime_rs::Service for JointMovJ {
    type Request = JointMovJ_Request;
    type Response = JointMovJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__JointMovJ() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__MoveJog() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__MoveJog
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveJog;

impl rosidl_runtime_rs::Service for MoveJog {
    type Request = MoveJog_Request;
    type Response = MoveJog_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__MoveJog() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__PayLoad() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__PayLoad
#[allow(missing_docs, non_camel_case_types)]
pub struct PayLoad;

impl rosidl_runtime_rs::Service for PayLoad {
    type Request = PayLoad_Request;
    type Response = PayLoad_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__PayLoad() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__PositiveSolution() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__PositiveSolution
#[allow(missing_docs, non_camel_case_types)]
pub struct PositiveSolution;

impl rosidl_runtime_rs::Service for PositiveSolution {
    type Request = PositiveSolution_Request;
    type Response = PositiveSolution_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__PositiveSolution() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ResetRobot() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__ResetRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct ResetRobot;

impl rosidl_runtime_rs::Service for ResetRobot {
    type Request = ResetRobot_Request;
    type Response = ResetRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ResetRobot() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__RobotMode() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__RobotMode
#[allow(missing_docs, non_camel_case_types)]
pub struct RobotMode;

impl rosidl_runtime_rs::Service for RobotMode {
    type Request = RobotMode_Request;
    type Response = RobotMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__RobotMode() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SetCollisionLevel() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__SetCollisionLevel
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCollisionLevel;

impl rosidl_runtime_rs::Service for SetCollisionLevel {
    type Request = SetCollisionLevel_Request;
    type Response = SetCollisionLevel_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SetCollisionLevel() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SpeedFactor() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__SpeedFactor
#[allow(missing_docs, non_camel_case_types)]
pub struct SpeedFactor;

impl rosidl_runtime_rs::Service for SpeedFactor {
    type Request = SpeedFactor_Request;
    type Response = SpeedFactor_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SpeedFactor() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SpeedJ() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__SpeedJ
#[allow(missing_docs, non_camel_case_types)]
pub struct SpeedJ;

impl rosidl_runtime_rs::Service for SpeedJ {
    type Request = SpeedJ_Request;
    type Response = SpeedJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SpeedJ() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SpeedL() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__SpeedL
#[allow(missing_docs, non_camel_case_types)]
pub struct SpeedL;

impl rosidl_runtime_rs::Service for SpeedL {
    type Request = SpeedL_Request;
    type Response = SpeedL_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__SpeedL() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__Tool() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__Tool
#[allow(missing_docs, non_camel_case_types)]
pub struct Tool;

impl rosidl_runtime_rs::Service for Tool {
    type Request = Tool_Request;
    type Response = Tool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__Tool() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ToolDI() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__ToolDI
#[allow(missing_docs, non_camel_case_types)]
pub struct ToolDI;

impl rosidl_runtime_rs::Service for ToolDI {
    type Request = ToolDI_Request;
    type Response = ToolDI_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ToolDI() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ToolDOExecute() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__ToolDOExecute
#[allow(missing_docs, non_camel_case_types)]
pub struct ToolDOExecute;

impl rosidl_runtime_rs::Service for ToolDOExecute {
    type Request = ToolDOExecute_Request;
    type Response = ToolDOExecute_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__ToolDOExecute() }
    }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__User() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__srv__User
#[allow(missing_docs, non_camel_case_types)]
pub struct User;

impl rosidl_runtime_rs::Service for User {
    type Request = User_Request;
    type Response = User_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mg400_msgs__srv__User() }
    }
}


