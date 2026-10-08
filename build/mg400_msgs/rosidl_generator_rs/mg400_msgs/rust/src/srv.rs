#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mg400_msgs__srv__AccJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for AccJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AccJ_Request {
  type RmwMsg = super::srv::rmw::AccJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to mg400_msgs__srv__AccJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AccJ_Response {
  type RmwMsg = super::srv::rmw::AccJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__AccL_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for AccL_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccL_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AccL_Request {
  type RmwMsg = super::srv::rmw::AccL_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to mg400_msgs__srv__AccL_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccL_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AccL_Response {
  type RmwMsg = super::srv::rmw::AccL_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__Arch_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arch_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::Arch,

}



impl Default for Arch_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Arch_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Arch_Request {
  type RmwMsg = super::srv::rmw::Arch_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::Arch::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::Arch::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: super::msg::Arch::from_rmw_message(msg.index),
    }
  }
}


// Corresponds to mg400_msgs__srv__Arch_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Arch_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Arch_Response {
  type RmwMsg = super::srv::rmw::Arch_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__CP_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u16,

}



impl Default for CP_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CP_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CP_Request {
  type RmwMsg = super::srv::rmw::CP_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to mg400_msgs__srv__CP_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CP_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CP_Response {
  type RmwMsg = super::srv::rmw::CP_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__ClearError_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ClearError_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearError_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ClearError_Request {
  type RmwMsg = super::srv::rmw::ClearError_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__ClearError_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearError_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ClearError_Response {
  type RmwMsg = super::srv::rmw::ClearError_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__DI_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::DIIndex,

}



impl Default for DI_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DI_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DI_Request {
  type RmwMsg = super::srv::rmw::DI_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::DIIndex::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::DIIndex::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: super::msg::DIIndex::from_rmw_message(msg.index),
    }
  }
}


// Corresponds to mg400_msgs__srv__DI_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DI_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DI_Response {
  type RmwMsg = super::srv::rmw::DI_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__DO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::DOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::DOStatus,

}



impl Default for DO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DO_Request {
  type RmwMsg = super::srv::rmw::DO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::DOIndex::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::DOIndex::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: super::msg::DOIndex::from_rmw_message(msg.index),
      status: super::msg::DOStatus::from_rmw_message(msg.status),
    }
  }
}


// Corresponds to mg400_msgs__srv__DO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DO_Response {
  type RmwMsg = super::srv::rmw::DO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__DisableRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DisableRobot_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DisableRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Request {
  type RmwMsg = super::srv::rmw::DisableRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__DisableRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DisableRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Response {
  type RmwMsg = super::srv::rmw::DisableRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__EmergencyStop_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EmergencyStop_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EmergencyStop_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Request {
  type RmwMsg = super::srv::rmw::EmergencyStop_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__EmergencyStop_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EmergencyStop_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Response {
  type RmwMsg = super::srv::rmw::EmergencyStop_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__EnableRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Request {
  type RmwMsg = super::srv::rmw::EnableRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        num_of_params: msg.num_of_params,
        load: msg.load,
        center_x: msg.center_x,
        center_y: msg.center_y,
        center_z: msg.center_z,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      num_of_params: msg.num_of_params,
      load: msg.load,
      center_x: msg.center_x,
      center_y: msg.center_y,
      center_z: msg.center_z,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      num_of_params: msg.num_of_params,
      load: msg.load,
      center_x: msg.center_x,
      center_y: msg.center_y,
      center_z: msg.center_z,
    }
  }
}


// Corresponds to mg400_msgs__srv__EnableRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Response {
  type RmwMsg = super::srv::rmw::EnableRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__GetAngle_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetAngle_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAngle_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Request {
  type RmwMsg = super::srv::rmw::GetAngle_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__GetAngle_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAngle_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Response {
  type RmwMsg = super::srv::rmw::GetAngle_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_id: msg.error_id,
        joint1: msg.joint1,
        joint2: msg.joint2,
        joint3: msg.joint3,
        joint4: msg.joint4,
        joint5: msg.joint5,
        joint6: msg.joint6,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_id: msg.error_id,
      joint1: msg.joint1,
      joint2: msg.joint2,
      joint3: msg.joint3,
      joint4: msg.joint4,
      joint5: msg.joint5,
      joint6: msg.joint6,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_id: msg.error_id,
      joint1: msg.joint1,
      joint2: msg.joint2,
      joint3: msg.joint3,
      joint4: msg.joint4,
      joint5: msg.joint5,
      joint6: msg.joint6,
    }
  }
}


// Corresponds to mg400_msgs__srv__GetErrorID_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetErrorID_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetErrorID_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Request {
  type RmwMsg = super::srv::rmw::GetErrorID_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__GetErrorID_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_ids: super::msg::ErrorID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for GetErrorID_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetErrorID_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Response {
  type RmwMsg = super::srv::rmw::GetErrorID_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_ids: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_ids)).into_owned(),
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_ids: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_ids)).into_owned(),
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_ids: super::msg::ErrorID::from_rmw_message(msg.error_ids),
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__GetPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPose_Request {
  type RmwMsg = super::srv::rmw::GetPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__GetPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPose_Response {
  type RmwMsg = super::srv::rmw::GetPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_id: msg.error_id,
        pose1: msg.pose1,
        pose2: msg.pose2,
        pose3: msg.pose3,
        pose4: msg.pose4,
        pose5: msg.pose5,
        pose6: msg.pose6,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_id: msg.error_id,
      pose1: msg.pose1,
      pose2: msg.pose2,
      pose3: msg.pose3,
      pose4: msg.pose4,
      pose5: msg.pose5,
      pose6: msg.pose6,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_id: msg.error_id,
      pose1: msg.pose1,
      pose2: msg.pose2,
      pose3: msg.pose3,
      pose4: msg.pose4,
      pose5: msg.pose5,
      pose6: msg.pose6,
    }
  }
}


// Corresponds to mg400_msgs__srv__InverseSolution_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InverseSolution_Request::default())
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Request {
  type RmwMsg = super::srv::rmw::InverseSolution_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        r: msg.r,
        user: msg.user,
        tool: msg.tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      r: msg.r,
      user: msg.user,
      tool: msg.tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      r: msg.r,
      user: msg.user,
      tool: msg.tool,
    }
  }
}


// Corresponds to mg400_msgs__srv__InverseSolution_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InverseSolution_Response::default())
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Response {
  type RmwMsg = super::srv::rmw::InverseSolution_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_id: msg.error_id,
        j1: msg.j1,
        j2: msg.j2,
        j3: msg.j3,
        j4: msg.j4,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_id: msg.error_id,
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_id: msg.error_id,
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
    }
  }
}


// Corresponds to mg400_msgs__srv__JointMovJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::JointMovJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Request {
  type RmwMsg = super::srv::rmw::JointMovJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        j1: msg.j1,
        j2: msg.j2,
        j3: msg.j3,
        j4: msg.j4,
        j5: msg.j5,
        j6: msg.j6,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      j5: msg.j5,
      j6: msg.j6,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      j5: msg.j5,
      j6: msg.j6,
    }
  }
}


// Corresponds to mg400_msgs__srv__JointMovJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for JointMovJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::JointMovJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Response {
  type RmwMsg = super::srv::rmw::JointMovJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__MoveJog_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub jog: super::msg::MoveJog,

}



impl Default for MoveJog_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveJog_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Request {
  type RmwMsg = super::srv::rmw::MoveJog_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        jog: super::msg::MoveJog::into_rmw_message(std::borrow::Cow::Owned(msg.jog)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        jog: super::msg::MoveJog::into_rmw_message(std::borrow::Cow::Borrowed(&msg.jog)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      jog: super::msg::MoveJog::from_rmw_message(msg.jog),
    }
  }
}


// Corresponds to mg400_msgs__srv__MoveJog_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for MoveJog_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveJog_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Response {
  type RmwMsg = super::srv::rmw::MoveJog_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__PayLoad_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PayLoad_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PayLoad_Request {
  type RmwMsg = super::srv::rmw::PayLoad_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        weight: msg.weight,
        inertia: msg.inertia,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      weight: msg.weight,
      inertia: msg.inertia,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      weight: msg.weight,
      inertia: msg.inertia,
    }
  }
}


// Corresponds to mg400_msgs__srv__PayLoad_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PayLoad_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PayLoad_Response {
  type RmwMsg = super::srv::rmw::PayLoad_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__PositiveSolution_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PositiveSolution_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PositiveSolution_Request {
  type RmwMsg = super::srv::rmw::PositiveSolution_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        j1: msg.j1,
        j2: msg.j2,
        j3: msg.j3,
        j4: msg.j4,
        user: msg.user,
        tool: msg.tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      user: msg.user,
      tool: msg.tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      user: msg.user,
      tool: msg.tool,
    }
  }
}


// Corresponds to mg400_msgs__srv__PositiveSolution_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PositiveSolution_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PositiveSolution_Response {
  type RmwMsg = super::srv::rmw::PositiveSolution_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        error_id: msg.error_id,
        x: msg.x,
        y: msg.y,
        z: msg.z,
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      error_id: msg.error_id,
      x: msg.x,
      y: msg.y,
      z: msg.z,
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      error_id: msg.error_id,
      x: msg.x,
      y: msg.y,
      z: msg.z,
      r: msg.r,
    }
  }
}


// Corresponds to mg400_msgs__srv__ResetRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetRobot_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ResetRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Request {
  type RmwMsg = super::srv::rmw::ResetRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__ResetRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ResetRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Response {
  type RmwMsg = super::srv::rmw::ResetRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__RobotMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RobotMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RobotMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Request {
  type RmwMsg = super::srv::rmw::RobotMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mg400_msgs__srv__RobotMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_mode: super::msg::RobotMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: i32,

}



impl Default for RobotMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RobotMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Response {
  type RmwMsg = super::srv::rmw::RobotMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_mode: super::msg::RobotMode::into_rmw_message(std::borrow::Cow::Owned(msg.robot_mode)).into_owned(),
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_mode: super::msg::RobotMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_mode)).into_owned(),
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_mode: super::msg::RobotMode::from_rmw_message(msg.robot_mode),
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__SetCollisionLevel_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub level: super::msg::CollisionLevel,

}



impl Default for SetCollisionLevel_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCollisionLevel_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Request {
  type RmwMsg = super::srv::rmw::SetCollisionLevel_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        level: super::msg::CollisionLevel::into_rmw_message(std::borrow::Cow::Owned(msg.level)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        level: super::msg::CollisionLevel::into_rmw_message(std::borrow::Cow::Borrowed(&msg.level)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      level: super::msg::CollisionLevel::from_rmw_message(msg.level),
    }
  }
}


// Corresponds to mg400_msgs__srv__SetCollisionLevel_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCollisionLevel_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Response {
  type RmwMsg = super::srv::rmw::SetCollisionLevel_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__SpeedFactor_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ratio: u8,

}



impl Default for SpeedFactor_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedFactor_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Request {
  type RmwMsg = super::srv::rmw::SpeedFactor_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ratio: msg.ratio,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ratio: msg.ratio,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ratio: msg.ratio,
    }
  }
}


// Corresponds to mg400_msgs__srv__SpeedFactor_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedFactor_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Response {
  type RmwMsg = super::srv::rmw::SpeedFactor_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__SpeedJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for SpeedJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedJ_Request {
  type RmwMsg = super::srv::rmw::SpeedJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to mg400_msgs__srv__SpeedJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedJ_Response {
  type RmwMsg = super::srv::rmw::SpeedJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__SpeedL_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: u8,

}



impl Default for SpeedL_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedL_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedL_Request {
  type RmwMsg = super::srv::rmw::SpeedL_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to mg400_msgs__srv__SpeedL_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedL_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedL_Response {
  type RmwMsg = super::srv::rmw::SpeedL_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__Tool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: super::msg::Tool,

}



impl Default for Tool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Tool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Tool_Request {
  type RmwMsg = super::srv::rmw::Tool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        tool: super::msg::Tool::into_rmw_message(std::borrow::Cow::Owned(msg.tool)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        tool: super::msg::Tool::into_rmw_message(std::borrow::Cow::Borrowed(&msg.tool)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      tool: super::msg::Tool::from_rmw_message(msg.tool),
    }
  }
}


// Corresponds to mg400_msgs__srv__Tool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Tool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Tool_Response {
  type RmwMsg = super::srv::rmw::Tool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__ToolDI_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::ToolDIIndex,

}



impl Default for ToolDI_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDI_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Request {
  type RmwMsg = super::srv::rmw::ToolDI_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::ToolDIIndex::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::ToolDIIndex::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: super::msg::ToolDIIndex::from_rmw_message(msg.index),
    }
  }
}


// Corresponds to mg400_msgs__srv__ToolDI_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDI_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Response {
  type RmwMsg = super::srv::rmw::ToolDI_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__ToolDOExecute_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOExecute_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: super::msg::ToolDOIndex,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::DOStatus,

}



impl Default for ToolDOExecute_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDOExecute_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDOExecute_Request {
  type RmwMsg = super::srv::rmw::ToolDOExecute_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::ToolDOIndex::into_rmw_message(std::borrow::Cow::Owned(msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: super::msg::ToolDOIndex::into_rmw_message(std::borrow::Cow::Borrowed(&msg.index)).into_owned(),
        status: super::msg::DOStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: super::msg::ToolDOIndex::from_rmw_message(msg.index),
      status: super::msg::DOStatus::from_rmw_message(msg.status),
    }
  }
}


// Corresponds to mg400_msgs__srv__ToolDOExecute_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDOExecute_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDOExecute_Response {
  type RmwMsg = super::srv::rmw::ToolDOExecute_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
  }
}


// Corresponds to mg400_msgs__srv__User_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user: super::msg::User,

}



impl Default for User_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::User_Request::default())
  }
}

impl rosidl_runtime_rs::Message for User_Request {
  type RmwMsg = super::srv::rmw::User_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user: super::msg::User::into_rmw_message(std::borrow::Cow::Owned(msg.user)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user: super::msg::User::into_rmw_message(std::borrow::Cow::Borrowed(&msg.user)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      user: super::msg::User::from_rmw_message(msg.user),
    }
  }
}


// Corresponds to mg400_msgs__srv__User_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::User_Response::default())
  }
}

impl rosidl_runtime_rs::Message for User_Response {
  type RmwMsg = super::srv::rmw::User_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: msg.error_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      error_id: msg.error_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: msg.error_id,
    }
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


