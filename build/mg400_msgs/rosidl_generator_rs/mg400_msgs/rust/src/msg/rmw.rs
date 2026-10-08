#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__Arch() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__Arch__init(msg: *mut Arch) -> bool;
    fn mg400_msgs__msg__Arch__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arch>, size: usize) -> bool;
    fn mg400_msgs__msg__Arch__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arch>);
    fn mg400_msgs__msg__Arch__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arch>, out_seq: *mut rosidl_runtime_rs::Sequence<Arch>) -> bool;
}

// Corresponds to mg400_msgs__msg__Arch
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arch {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u8,

}

impl Arch {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX0: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX1: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX2: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX3: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX4: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX5: u8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX6: u8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX7: u8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX8: u8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INDEX9: u8 = 9;

}


impl Default for Arch {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__Arch__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__Arch__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arch {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Arch__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Arch__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Arch__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arch {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arch where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/Arch";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__Arch() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__CollisionLevel() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__CollisionLevel__init(msg: *mut CollisionLevel) -> bool;
    fn mg400_msgs__msg__CollisionLevel__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CollisionLevel>, size: usize) -> bool;
    fn mg400_msgs__msg__CollisionLevel__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CollisionLevel>);
    fn mg400_msgs__msg__CollisionLevel__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CollisionLevel>, out_seq: *mut rosidl_runtime_rs::Sequence<CollisionLevel>) -> bool;
}

// Corresponds to mg400_msgs__msg__CollisionLevel
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CollisionLevel {

    // This member is not documented.
    #[allow(missing_docs)]
    pub level: u8,

}

impl CollisionLevel {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OFF: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LEVEL1: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LEVEL2: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LEVEL3: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LEVEL4: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LEVEL5: u8 = 5;

}


impl Default for CollisionLevel {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__CollisionLevel__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__CollisionLevel__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CollisionLevel {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__CollisionLevel__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__CollisionLevel__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__CollisionLevel__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CollisionLevel {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CollisionLevel where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/CollisionLevel";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__CollisionLevel() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__Command() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__Command__init(msg: *mut Command) -> bool;
    fn mg400_msgs__msg__Command__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Command>, size: usize) -> bool;
    fn mg400_msgs__msg__Command__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Command>);
    fn mg400_msgs__msg__Command__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Command>, out_seq: *mut rosidl_runtime_rs::Sequence<Command>) -> bool;
}

// Corresponds to mg400_msgs__msg__Command
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// ===============================================================================
///  List of command-type IDs
///  (Commented-out commands have not been implemented.)
/// ===============================================================================

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Command {
    /// ===============================================================================
    pub command_type: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_j_params: super::super::msg::rmw::MovJ,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_l_params: super::super::msg::rmw::MovL,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_mov_j_params: super::super::msg::rmw::JointMovJ,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_jio_params: super::super::msg::rmw::MovJIO,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_lio_params: super::super::msg::rmw::MovLIO,

}

impl Command {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CT_MOV_J: u16 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CT_MOV_L: u16 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CT_JOINT_MOV_J: u16 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CT_MOV_JIO: u16 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CT_MOV_LIO: u16 = 5;

    /// uint16 CT_ARC = 6
    /// uint16 CT_CIRCLE = 7
    /// uint16 CT_MOVE_JOG = 8
    /// uint16 CT_SYNC = 9
    /// uint16 CT_REL_MOV_J_USER = 10
    /// uint16 CT_REL_MOV_L_USER = 11
    /// uint16 CT_REL_JOINT_MOV_J = 12
    /// uint16 CT_REL_MOV_J_EXT = 13
    /// uint16 CT_SYNC_ALL = 14
    pub const CT_TOOL_DO: u16 = 25;

}


impl Default for Command {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__Command__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__Command__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Command {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Command__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Command__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Command__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Command {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Command where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/Command";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__Command() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DIIndex() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__DIIndex__init(msg: *mut DIIndex) -> bool;
    fn mg400_msgs__msg__DIIndex__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DIIndex>, size: usize) -> bool;
    fn mg400_msgs__msg__DIIndex__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DIIndex>);
    fn mg400_msgs__msg__DIIndex__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DIIndex>, out_seq: *mut rosidl_runtime_rs::Sequence<DIIndex>) -> bool;
}

// Corresponds to mg400_msgs__msg__DIIndex
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIIndex {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u32,

}

impl DIIndex {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D1: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D2: u32 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D3: u32 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D4: u32 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D5: u32 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D6: u32 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D7: u32 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D8: u32 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D9: u32 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D10: u32 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D11: u32 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D12: u32 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D13: u32 = 13;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D14: u32 = 14;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D15: u32 = 15;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D16: u32 = 16;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D17: u32 = 17;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D18: u32 = 18;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D19: u32 = 19;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D20: u32 = 20;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D21: u32 = 21;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D22: u32 = 22;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D23: u32 = 23;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D24: u32 = 24;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D25: u32 = 25;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D26: u32 = 26;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D27: u32 = 27;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D28: u32 = 28;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D29: u32 = 29;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D30: u32 = 30;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D31: u32 = 31;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D32: u32 = 32;

}


impl Default for DIIndex {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__DIIndex__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__DIIndex__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DIIndex {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DIIndex__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DIIndex__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DIIndex__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DIIndex {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DIIndex where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/DIIndex";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DIIndex() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DOIndex() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__DOIndex__init(msg: *mut DOIndex) -> bool;
    fn mg400_msgs__msg__DOIndex__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOIndex>, size: usize) -> bool;
    fn mg400_msgs__msg__DOIndex__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOIndex>);
    fn mg400_msgs__msg__DOIndex__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOIndex>, out_seq: *mut rosidl_runtime_rs::Sequence<DOIndex>) -> bool;
}

// Corresponds to mg400_msgs__msg__DOIndex
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOIndex {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u32,

}

impl DOIndex {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D1: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D2: u32 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D3: u32 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D4: u32 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D5: u32 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D6: u32 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D7: u32 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D8: u32 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D9: u32 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D10: u32 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D11: u32 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D12: u32 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D13: u32 = 13;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D14: u32 = 14;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D15: u32 = 15;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D16: u32 = 16;

}


impl Default for DOIndex {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__DOIndex__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__DOIndex__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOIndex {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DOIndex__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DOIndex__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DOIndex__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOIndex {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOIndex where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/DOIndex";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DOIndex() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DOStatus() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__DOStatus__init(msg: *mut DOStatus) -> bool;
    fn mg400_msgs__msg__DOStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOStatus>, size: usize) -> bool;
    fn mg400_msgs__msg__DOStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOStatus>);
    fn mg400_msgs__msg__DOStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<DOStatus>) -> bool;
}

// Corresponds to mg400_msgs__msg__DOStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,

}

impl DOStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LOW: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HIGH: u8 = 1;

}


impl Default for DOStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__DOStatus__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__DOStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DOStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DOStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DOStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOStatus where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/DOStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DOStatus() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DistanceMode() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__DistanceMode__init(msg: *mut DistanceMode) -> bool;
    fn mg400_msgs__msg__DistanceMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DistanceMode>, size: usize) -> bool;
    fn mg400_msgs__msg__DistanceMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DistanceMode>);
    fn mg400_msgs__msg__DistanceMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DistanceMode>, out_seq: *mut rosidl_runtime_rs::Sequence<DistanceMode>) -> bool;
}

// Corresponds to mg400_msgs__msg__DistanceMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DistanceMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,

}

impl DistanceMode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PERCENTAGE: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FROM_START_OR_TARGET: u8 = 1;

}


impl Default for DistanceMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__DistanceMode__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__DistanceMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DistanceMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DistanceMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DistanceMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__DistanceMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DistanceMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DistanceMode where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/DistanceMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__DistanceMode() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__ErrorID() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__ErrorID__init(msg: *mut ErrorID) -> bool;
    fn mg400_msgs__msg__ErrorID__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ErrorID>, size: usize) -> bool;
    fn mg400_msgs__msg__ErrorID__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ErrorID>);
    fn mg400_msgs__msg__ErrorID__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ErrorID>, out_seq: *mut rosidl_runtime_rs::Sequence<ErrorID>) -> bool;
}

// Corresponds to mg400_msgs__msg__ErrorID
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ErrorID {

    // This member is not documented.
    #[allow(missing_docs)]
    pub controller: super::super::msg::rmw::IDArray,


    // This member is not documented.
    #[allow(missing_docs)]
    pub servo: [super::super::msg::rmw::IDArray; 5],

}



impl Default for ErrorID {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__ErrorID__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__ErrorID__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ErrorID {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ErrorID__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ErrorID__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ErrorID__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ErrorID {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ErrorID where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/ErrorID";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__ErrorID() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__IDArray() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__IDArray__init(msg: *mut IDArray) -> bool;
    fn mg400_msgs__msg__IDArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IDArray>, size: usize) -> bool;
    fn mg400_msgs__msg__IDArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IDArray>);
    fn mg400_msgs__msg__IDArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IDArray>, out_seq: *mut rosidl_runtime_rs::Sequence<IDArray>) -> bool;
}

// Corresponds to mg400_msgs__msg__IDArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IDArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ids: rosidl_runtime_rs::Sequence<i32>,

}



impl Default for IDArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__IDArray__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__IDArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IDArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__IDArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__IDArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__IDArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IDArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IDArray where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/IDArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__IDArray() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__JointMovJ() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__JointMovJ__init(msg: *mut JointMovJ) -> bool;
    fn mg400_msgs__msg__JointMovJ__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ>, size: usize) -> bool;
    fn mg400_msgs__msg__JointMovJ__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<JointMovJ>);
    fn mg400_msgs__msg__JointMovJ__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<JointMovJ>, out_seq: *mut rosidl_runtime_rs::Sequence<JointMovJ>) -> bool;
}

// Corresponds to mg400_msgs__msg__JointMovJ
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ {
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



impl Default for JointMovJ {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__JointMovJ__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__JointMovJ__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for JointMovJ {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__JointMovJ__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__JointMovJ__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__JointMovJ__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for JointMovJ {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for JointMovJ where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/JointMovJ";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__JointMovJ() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovJ() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__MovJ__init(msg: *mut MovJ) -> bool;
    fn mg400_msgs__msg__MovJ__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ>, size: usize) -> bool;
    fn mg400_msgs__msg__MovJ__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ>);
    fn mg400_msgs__msg__MovJ__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ>) -> bool;
}

// Corresponds to mg400_msgs__msg__MovJ
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ {

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



impl Default for MovJ {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__MovJ__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__MovJ__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovJ__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovJ__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovJ__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/MovJ";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovJ() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovJIO() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__MovJIO__init(msg: *mut MovJIO) -> bool;
    fn mg400_msgs__msg__MovJIO__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO>, size: usize) -> bool;
    fn mg400_msgs__msg__MovJIO__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO>);
    fn mg400_msgs__msg__MovJIO__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO>) -> bool;
}

// Corresponds to mg400_msgs__msg__MovJIO
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO {

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

}



impl Default for MovJIO {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__MovJIO__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__MovJIO__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovJIO__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovJIO__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovJIO__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/MovJIO";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovJIO() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovL() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__MovL__init(msg: *mut MovL) -> bool;
    fn mg400_msgs__msg__MovL__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL>, size: usize) -> bool;
    fn mg400_msgs__msg__MovL__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL>);
    fn mg400_msgs__msg__MovL__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL>) -> bool;
}

// Corresponds to mg400_msgs__msg__MovL
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL {

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



impl Default for MovL {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__MovL__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__MovL__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovL__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovL__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovL__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/MovL";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovL() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovLIO() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__MovLIO__init(msg: *mut MovLIO) -> bool;
    fn mg400_msgs__msg__MovLIO__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO>, size: usize) -> bool;
    fn mg400_msgs__msg__MovLIO__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO>);
    fn mg400_msgs__msg__MovLIO__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO>) -> bool;
}

// Corresponds to mg400_msgs__msg__MovLIO
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO {

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

}



impl Default for MovLIO {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__MovLIO__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__MovLIO__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovLIO__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovLIO__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MovLIO__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/MovLIO";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MovLIO() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MoveJog() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__MoveJog__init(msg: *mut MoveJog) -> bool;
    fn mg400_msgs__msg__MoveJog__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveJog>, size: usize) -> bool;
    fn mg400_msgs__msg__MoveJog__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveJog>);
    fn mg400_msgs__msg__MoveJog__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveJog>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveJog>) -> bool;
}

// Corresponds to mg400_msgs__msg__MoveJog
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog {

    // This member is not documented.
    #[allow(missing_docs)]
    pub jog_mode: rosidl_runtime_rs::String,

}

impl MoveJog {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J1_NEGATIVE: &'static str = "j1-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J1_POSITIVE: &'static str = "j1+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J2_NEGATIVE: &'static str = "j2-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J2_POSITIVE: &'static str = "j2+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J3_NEGATIVE: &'static str = "j3-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J3_POSITIVE: &'static str = "j3+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J4_NEGATIVE: &'static str = "j4-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const J4_POSITIVE: &'static str = "j4+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const X_NEGATIVE: &'static str = "X-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const X_POSITIVE: &'static str = "X+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const Y_NEGATIVE: &'static str = "Y-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const Y_POSITIVE: &'static str = "Y+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const Z_NEGATIVE: &'static str = "Z-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const Z_POSITIVE: &'static str = "Z+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RX_NEGATIVE: &'static str = "Rx-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RX_POSITIVE: &'static str = "Rx+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RY_NEGATIVE: &'static str = "Ry-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RY_POSITIVE: &'static str = "Ry+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RZ_NEGATIVE: &'static str = "Rz-";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RZ_POSITIVE: &'static str = "Rz+";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STOP: &'static str = "";

}


impl Default for MoveJog {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__MoveJog__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__MoveJog__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveJog {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MoveJog__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MoveJog__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__MoveJog__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveJog {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveJog where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/MoveJog";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__MoveJog() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__RobotMode() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__RobotMode__init(msg: *mut RobotMode) -> bool;
    fn mg400_msgs__msg__RobotMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotMode>, size: usize) -> bool;
    fn mg400_msgs__msg__RobotMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotMode>);
    fn mg400_msgs__msg__RobotMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotMode>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotMode>) -> bool;
}

// Corresponds to mg400_msgs__msg__RobotMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_mode: u64,

}

impl RobotMode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INIT: u64 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BRAKE_OPEN: u64 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DISABLED: u64 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ENABLE: u64 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BACKDRIVE: u64 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RUNNING: u64 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RECORDING: u64 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ERROR: u64 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PAUSE: u64 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const JOG: u64 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID: u64 = 12;

}


impl Default for RobotMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__RobotMode__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__RobotMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__RobotMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__RobotMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__RobotMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotMode where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/RobotMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__RobotMode() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__Tool() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__Tool__init(msg: *mut Tool) -> bool;
    fn mg400_msgs__msg__Tool__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Tool>, size: usize) -> bool;
    fn mg400_msgs__msg__Tool__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Tool>);
    fn mg400_msgs__msg__Tool__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Tool>, out_seq: *mut rosidl_runtime_rs::Sequence<Tool>) -> bool;
}

// Corresponds to mg400_msgs__msg__Tool
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: u8,

}

impl Tool {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL0: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL1: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL2: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL3: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL4: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL5: u8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL6: u8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL7: u8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL8: u8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TOOL9: u8 = 9;

}


impl Default for Tool {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__Tool__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__Tool__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Tool {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Tool__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Tool__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__Tool__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Tool {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Tool where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/Tool";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__Tool() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__ToolDIIndex() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__ToolDIIndex__init(msg: *mut ToolDIIndex) -> bool;
    fn mg400_msgs__msg__ToolDIIndex__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDIIndex>, size: usize) -> bool;
    fn mg400_msgs__msg__ToolDIIndex__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDIIndex>);
    fn mg400_msgs__msg__ToolDIIndex__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDIIndex>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDIIndex>) -> bool;
}

// Corresponds to mg400_msgs__msg__ToolDIIndex
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDIIndex {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u32,

}

impl ToolDIIndex {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D1: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D2: u32 = 2;

}


impl Default for ToolDIIndex {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__ToolDIIndex__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__ToolDIIndex__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDIIndex {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ToolDIIndex__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ToolDIIndex__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ToolDIIndex__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDIIndex {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDIIndex where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/ToolDIIndex";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__ToolDIIndex() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__ToolDOIndex() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__ToolDOIndex__init(msg: *mut ToolDOIndex) -> bool;
    fn mg400_msgs__msg__ToolDOIndex__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDOIndex>, size: usize) -> bool;
    fn mg400_msgs__msg__ToolDOIndex__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDOIndex>);
    fn mg400_msgs__msg__ToolDOIndex__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDOIndex>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDOIndex>) -> bool;
}

// Corresponds to mg400_msgs__msg__ToolDOIndex
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOIndex {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: u32,

}

impl ToolDOIndex {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D1: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const D2: u32 = 2;

}


impl Default for ToolDOIndex {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__ToolDOIndex__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__ToolDOIndex__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDOIndex {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ToolDOIndex__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ToolDOIndex__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__ToolDOIndex__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDOIndex {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDOIndex where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/ToolDOIndex";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__ToolDOIndex() }
  }
}


#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__User() -> *const std::ffi::c_void;
}

#[link(name = "mg400_msgs__rosidl_generator_c")]
extern "C" {
    fn mg400_msgs__msg__User__init(msg: *mut User) -> bool;
    fn mg400_msgs__msg__User__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<User>, size: usize) -> bool;
    fn mg400_msgs__msg__User__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<User>);
    fn mg400_msgs__msg__User__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<User>, out_seq: *mut rosidl_runtime_rs::Sequence<User>) -> bool;
}

// Corresponds to mg400_msgs__msg__User
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user: u8,

}

impl User {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER0: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER1: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER2: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER3: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER4: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER5: u8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER6: u8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER7: u8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER8: u8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USER9: u8 = 9;

}


impl Default for User {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mg400_msgs__msg__User__init(&mut msg as *mut _) {
        panic!("Call to mg400_msgs__msg__User__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for User {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__User__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__User__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mg400_msgs__msg__User__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for User {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for User where Self: Sized {
  const TYPE_NAME: &'static str = "mg400_msgs/msg/User";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mg400_msgs__msg__User() }
  }
}


