#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mg400_msgs__msg__Arch

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Arch::default())
  }
}

impl rosidl_runtime_rs::Message for Arch {
  type RmwMsg = super::msg::rmw::Arch;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to mg400_msgs__msg__CollisionLevel

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CollisionLevel::default())
  }
}

impl rosidl_runtime_rs::Message for CollisionLevel {
  type RmwMsg = super::msg::rmw::CollisionLevel;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        level: msg.level,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      level: msg.level,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      level: msg.level,
    }
  }
}


// Corresponds to mg400_msgs__msg__Command
/// ===============================================================================
///  List of command-type IDs
///  (Commented-out commands have not been implemented.)
/// ===============================================================================

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Command {
    /// ===============================================================================
    pub command_type: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_j_params: super::msg::MovJ,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_l_params: super::msg::MovL,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_mov_j_params: super::msg::JointMovJ,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_jio_params: super::msg::MovJIO,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mov_lio_params: super::msg::MovLIO,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Command::default())
  }
}

impl rosidl_runtime_rs::Message for Command {
  type RmwMsg = super::msg::rmw::Command;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command_type: msg.command_type,
        mov_j_params: super::msg::MovJ::into_rmw_message(std::borrow::Cow::Owned(msg.mov_j_params)).into_owned(),
        mov_l_params: super::msg::MovL::into_rmw_message(std::borrow::Cow::Owned(msg.mov_l_params)).into_owned(),
        joint_mov_j_params: super::msg::JointMovJ::into_rmw_message(std::borrow::Cow::Owned(msg.joint_mov_j_params)).into_owned(),
        mov_jio_params: super::msg::MovJIO::into_rmw_message(std::borrow::Cow::Owned(msg.mov_jio_params)).into_owned(),
        mov_lio_params: super::msg::MovLIO::into_rmw_message(std::borrow::Cow::Owned(msg.mov_lio_params)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      command_type: msg.command_type,
        mov_j_params: super::msg::MovJ::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mov_j_params)).into_owned(),
        mov_l_params: super::msg::MovL::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mov_l_params)).into_owned(),
        joint_mov_j_params: super::msg::JointMovJ::into_rmw_message(std::borrow::Cow::Borrowed(&msg.joint_mov_j_params)).into_owned(),
        mov_jio_params: super::msg::MovJIO::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mov_jio_params)).into_owned(),
        mov_lio_params: super::msg::MovLIO::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mov_lio_params)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command_type: msg.command_type,
      mov_j_params: super::msg::MovJ::from_rmw_message(msg.mov_j_params),
      mov_l_params: super::msg::MovL::from_rmw_message(msg.mov_l_params),
      joint_mov_j_params: super::msg::JointMovJ::from_rmw_message(msg.joint_mov_j_params),
      mov_jio_params: super::msg::MovJIO::from_rmw_message(msg.mov_jio_params),
      mov_lio_params: super::msg::MovLIO::from_rmw_message(msg.mov_lio_params),
    }
  }
}


// Corresponds to mg400_msgs__msg__DIIndex

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DIIndex::default())
  }
}

impl rosidl_runtime_rs::Message for DIIndex {
  type RmwMsg = super::msg::rmw::DIIndex;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to mg400_msgs__msg__DOIndex

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DOIndex::default())
  }
}

impl rosidl_runtime_rs::Message for DOIndex {
  type RmwMsg = super::msg::rmw::DOIndex;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to mg400_msgs__msg__DOStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DOStatus::default())
  }
}

impl rosidl_runtime_rs::Message for DOStatus {
  type RmwMsg = super::msg::rmw::DOStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to mg400_msgs__msg__DistanceMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DistanceMode::default())
  }
}

impl rosidl_runtime_rs::Message for DistanceMode {
  type RmwMsg = super::msg::rmw::DistanceMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
    }
  }
}


// Corresponds to mg400_msgs__msg__ErrorID

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ErrorID {

    // This member is not documented.
    #[allow(missing_docs)]
    pub controller: super::msg::IDArray,


    // This member is not documented.
    #[allow(missing_docs)]
    pub servo: [super::msg::IDArray; 5],

}



impl Default for ErrorID {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ErrorID::default())
  }
}

impl rosidl_runtime_rs::Message for ErrorID {
  type RmwMsg = super::msg::rmw::ErrorID;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        controller: super::msg::IDArray::into_rmw_message(std::borrow::Cow::Owned(msg.controller)).into_owned(),
        servo: msg.servo
          .map(|elem| super::msg::IDArray::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        controller: super::msg::IDArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.controller)).into_owned(),
        servo: msg.servo
          .iter()
          .map(|elem| super::msg::IDArray::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      controller: super::msg::IDArray::from_rmw_message(msg.controller),
      servo: msg.servo
        .map(super::msg::IDArray::from_rmw_message),
    }
  }
}


// Corresponds to mg400_msgs__msg__IDArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IDArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ids: Vec<i32>,

}



impl Default for IDArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IDArray::default())
  }
}

impl rosidl_runtime_rs::Message for IDArray {
  type RmwMsg = super::msg::rmw::IDArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ids: msg.ids.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ids: msg.ids.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ids: msg.ids
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to mg400_msgs__msg__JointMovJ

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::JointMovJ::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ {
  type RmwMsg = super::msg::rmw::JointMovJ;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_angles: msg.joint_angles,
        set_speed_j: msg.set_speed_j,
        speed_j: msg.speed_j,
        set_acc_j: msg.set_acc_j,
        acc_j: msg.acc_j,
        set_cp: msg.set_cp,
        cp: msg.cp,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_angles: msg.joint_angles,
      set_speed_j: msg.set_speed_j,
      speed_j: msg.speed_j,
      set_acc_j: msg.set_acc_j,
      acc_j: msg.acc_j,
      set_cp: msg.set_cp,
      cp: msg.cp,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_angles: msg.joint_angles,
      set_speed_j: msg.set_speed_j,
      speed_j: msg.speed_j,
      set_acc_j: msg.set_acc_j,
      acc_j: msg.acc_j,
      set_cp: msg.set_cp,
      cp: msg.cp,
    }
  }
}


// Corresponds to mg400_msgs__msg__MovJ

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::PoseStamped,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MovJ::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ {
  type RmwMsg = super::msg::rmw::MovJ;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        set_speed_j: msg.set_speed_j,
        speed_j: msg.speed_j,
        set_acc_j: msg.set_acc_j,
        acc_j: msg.acc_j,
        set_cp: msg.set_cp,
        cp: msg.cp,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      set_speed_j: msg.set_speed_j,
      speed_j: msg.speed_j,
      set_acc_j: msg.set_acc_j,
      acc_j: msg.acc_j,
      set_cp: msg.set_cp,
      cp: msg.cp,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
      set_speed_j: msg.set_speed_j,
      speed_j: msg.speed_j,
      set_acc_j: msg.set_acc_j,
      acc_j: msg.acc_j,
      set_cp: msg.set_cp,
      cp: msg.cp,
    }
  }
}


// Corresponds to mg400_msgs__msg__MovJIO

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: super::msg::DistanceMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::DOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::DOStatus,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MovJIO::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO {
  type RmwMsg = super::msg::rmw::MovJIO;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        mode: super::msg::DistanceMode::into_rmw_message(std::borrow::Cow::Owned(msg.mode)).into_owned(),
        distance: msg.distance,
        index: super::msg::DOIndex::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        set_speed_j: msg.set_speed_j,
        speed_j: msg.speed_j,
        set_acc_j: msg.set_acc_j,
        acc_j: msg.acc_j,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        mode: super::msg::DistanceMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mode)).into_owned(),
      distance: msg.distance,
        index: super::msg::DOIndex::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      set_speed_j: msg.set_speed_j,
      speed_j: msg.speed_j,
      set_acc_j: msg.set_acc_j,
      acc_j: msg.acc_j,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
      mode: super::msg::DistanceMode::from_rmw_message(msg.mode),
      distance: msg.distance,
      index: super::msg::DOIndex::from_rmw_message(msg.index),
      status: super::msg::DOStatus::from_rmw_message(msg.status),
      set_speed_j: msg.set_speed_j,
      speed_j: msg.speed_j,
      set_acc_j: msg.set_acc_j,
      acc_j: msg.acc_j,
    }
  }
}


// Corresponds to mg400_msgs__msg__MovL

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::PoseStamped,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MovL::default())
  }
}

impl rosidl_runtime_rs::Message for MovL {
  type RmwMsg = super::msg::rmw::MovL;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        set_speed_l: msg.set_speed_l,
        speed_l: msg.speed_l,
        set_acc_l: msg.set_acc_l,
        acc_l: msg.acc_l,
        set_cp: msg.set_cp,
        cp: msg.cp,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      set_speed_l: msg.set_speed_l,
      speed_l: msg.speed_l,
      set_acc_l: msg.set_acc_l,
      acc_l: msg.acc_l,
      set_cp: msg.set_cp,
      cp: msg.cp,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
      set_speed_l: msg.set_speed_l,
      speed_l: msg.speed_l,
      set_acc_l: msg.set_acc_l,
      acc_l: msg.acc_l,
      set_cp: msg.set_cp,
      cp: msg.cp,
    }
  }
}


// Corresponds to mg400_msgs__msg__MovLIO

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: super::msg::DistanceMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::DOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::DOStatus,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MovLIO::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO {
  type RmwMsg = super::msg::rmw::MovLIO;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        mode: super::msg::DistanceMode::into_rmw_message(std::borrow::Cow::Owned(msg.mode)).into_owned(),
        distance: msg.distance,
        index: super::msg::DOIndex::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        set_speed_l: msg.set_speed_l,
        speed_l: msg.speed_l,
        set_acc_l: msg.set_acc_l,
        acc_l: msg.acc_l,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        mode: super::msg::DistanceMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.mode)).into_owned(),
      distance: msg.distance,
        index: super::msg::DOIndex::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      set_speed_l: msg.set_speed_l,
      speed_l: msg.speed_l,
      set_acc_l: msg.set_acc_l,
      acc_l: msg.acc_l,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
      mode: super::msg::DistanceMode::from_rmw_message(msg.mode),
      distance: msg.distance,
      index: super::msg::DOIndex::from_rmw_message(msg.index),
      status: super::msg::DOStatus::from_rmw_message(msg.status),
      set_speed_l: msg.set_speed_l,
      speed_l: msg.speed_l,
      set_acc_l: msg.set_acc_l,
      acc_l: msg.acc_l,
    }
  }
}


// Corresponds to mg400_msgs__msg__MoveJog

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog {

    // This member is not documented.
    #[allow(missing_docs)]
    pub jog_mode: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MoveJog::default())
  }
}

impl rosidl_runtime_rs::Message for MoveJog {
  type RmwMsg = super::msg::rmw::MoveJog;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        jog_mode: msg.jog_mode.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        jog_mode: msg.jog_mode.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      jog_mode: msg.jog_mode.to_string(),
    }
  }
}


// Corresponds to mg400_msgs__msg__RobotMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RobotMode::default())
  }
}

impl rosidl_runtime_rs::Message for RobotMode {
  type RmwMsg = super::msg::rmw::RobotMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_mode: msg.robot_mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      robot_mode: msg.robot_mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_mode: msg.robot_mode,
    }
  }
}


// Corresponds to mg400_msgs__msg__Tool

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Tool::default())
  }
}

impl rosidl_runtime_rs::Message for Tool {
  type RmwMsg = super::msg::rmw::Tool;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        tool: msg.tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      tool: msg.tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      tool: msg.tool,
    }
  }
}


// Corresponds to mg400_msgs__msg__ToolDIIndex

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ToolDIIndex::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDIIndex {
  type RmwMsg = super::msg::rmw::ToolDIIndex;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to mg400_msgs__msg__ToolDOIndex

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ToolDOIndex::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDOIndex {
  type RmwMsg = super::msg::rmw::ToolDOIndex;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to mg400_msgs__msg__User

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::User::default())
  }
}

impl rosidl_runtime_rs::Message for User {
  type RmwMsg = super::msg::rmw::User;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user: msg.user,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      user: msg.user,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      user: msg.user,
    }
  }
}


