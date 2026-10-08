#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to easy_handeye2_msgs__srv__ListAlgorithms_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAlgorithms_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ListAlgorithms_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListAlgorithms_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ListAlgorithms_Request {
  type RmwMsg = super::srv::rmw::ListAlgorithms_Request;

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


// Corresponds to easy_handeye2_msgs__srv__ListAlgorithms_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListAlgorithms_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub algorithms: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_algorithm: std::string::String,

}



impl Default for ListAlgorithms_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ListAlgorithms_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ListAlgorithms_Response {
  type RmwMsg = super::srv::rmw::ListAlgorithms_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        algorithms: msg.algorithms
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        current_algorithm: msg.current_algorithm.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        algorithms: msg.algorithms
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        current_algorithm: msg.current_algorithm.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      algorithms: msg.algorithms
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      current_algorithm: msg.current_algorithm.to_string(),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__SetAlgorithm_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAlgorithm_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub new_algorithm: std::string::String,

}



impl Default for SetAlgorithm_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetAlgorithm_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetAlgorithm_Request {
  type RmwMsg = super::srv::rmw::SetAlgorithm_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        new_algorithm: msg.new_algorithm.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        new_algorithm: msg.new_algorithm.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      new_algorithm: msg.new_algorithm.to_string(),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__SetAlgorithm_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetAlgorithm_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SetAlgorithm_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetAlgorithm_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetAlgorithm_Response {
  type RmwMsg = super::srv::rmw::SetAlgorithm_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__ComputeCalibration_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCalibration_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ComputeCalibration_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ComputeCalibration_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCalibration_Request {
  type RmwMsg = super::srv::rmw::ComputeCalibration_Request;

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


// Corresponds to easy_handeye2_msgs__srv__ComputeCalibration_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ComputeCalibration_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub valid: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calibration: super::msg::HandeyeCalibration,

}



impl Default for ComputeCalibration_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ComputeCalibration_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ComputeCalibration_Response {
  type RmwMsg = super::srv::rmw::ComputeCalibration_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
        calibration: super::msg::HandeyeCalibration::into_rmw_message(std::borrow::Cow::Owned(msg.calibration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
        calibration: super::msg::HandeyeCalibration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.calibration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
      calibration: super::msg::HandeyeCalibration::from_rmw_message(msg.calibration),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__SaveCalibration_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveCalibration_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SaveCalibration_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveCalibration_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveCalibration_Request {
  type RmwMsg = super::srv::rmw::SaveCalibration_Request;

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


// Corresponds to easy_handeye2_msgs__srv__SaveCalibration_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveCalibration_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filepath: std_msgs::msg::String,

}



impl Default for SaveCalibration_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveCalibration_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveCalibration_Response {
  type RmwMsg = super::srv::rmw::SaveCalibration_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        filepath: std_msgs::msg::String::into_rmw_message(std::borrow::Cow::Owned(msg.filepath)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        filepath: std_msgs::msg::String::into_rmw_message(std::borrow::Cow::Borrowed(&msg.filepath)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      filepath: std_msgs::msg::String::from_rmw_message(msg.filepath),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__RemoveSample_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveSample_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sample_index: i8,

}



impl Default for RemoveSample_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RemoveSample_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RemoveSample_Request {
  type RmwMsg = super::srv::rmw::RemoveSample_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sample_index: msg.sample_index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      sample_index: msg.sample_index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sample_index: msg.sample_index,
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__RemoveSample_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RemoveSample_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: super::msg::SampleList,

}



impl Default for RemoveSample_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RemoveSample_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RemoveSample_Response {
  type RmwMsg = super::srv::rmw::RemoveSample_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        samples: super::msg::SampleList::into_rmw_message(std::borrow::Cow::Owned(msg.samples)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        samples: super::msg::SampleList::into_rmw_message(std::borrow::Cow::Borrowed(&msg.samples)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      samples: super::msg::SampleList::from_rmw_message(msg.samples),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__TakeSample_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TakeSample_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for TakeSample_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TakeSample_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TakeSample_Request {
  type RmwMsg = super::srv::rmw::TakeSample_Request;

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


// Corresponds to easy_handeye2_msgs__srv__TakeSample_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TakeSample_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: super::msg::SampleList,

}



impl Default for TakeSample_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TakeSample_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TakeSample_Response {
  type RmwMsg = super::srv::rmw::TakeSample_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        samples: super::msg::SampleList::into_rmw_message(std::borrow::Cow::Owned(msg.samples)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        samples: super::msg::SampleList::into_rmw_message(std::borrow::Cow::Borrowed(&msg.samples)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      samples: super::msg::SampleList::from_rmw_message(msg.samples),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__SaveSamples_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveSamples_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SaveSamples_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveSamples_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveSamples_Request {
  type RmwMsg = super::srv::rmw::SaveSamples_Request;

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


// Corresponds to easy_handeye2_msgs__srv__SaveSamples_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveSamples_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SaveSamples_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveSamples_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveSamples_Response {
  type RmwMsg = super::srv::rmw::SaveSamples_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__LoadSamples_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSamples_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for LoadSamples_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadSamples_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoadSamples_Request {
  type RmwMsg = super::srv::rmw::LoadSamples_Request;

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


// Corresponds to easy_handeye2_msgs__srv__LoadSamples_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoadSamples_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: super::msg::SampleList,

}



impl Default for LoadSamples_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoadSamples_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoadSamples_Response {
  type RmwMsg = super::srv::rmw::LoadSamples_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        samples: super::msg::SampleList::into_rmw_message(std::borrow::Cow::Owned(msg.samples)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        samples: super::msg::SampleList::into_rmw_message(std::borrow::Cow::Borrowed(&msg.samples)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      samples: super::msg::SampleList::from_rmw_message(msg.samples),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__CheckStartingPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckStartingPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for CheckStartingPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckStartingPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CheckStartingPose_Request {
  type RmwMsg = super::srv::rmw::CheckStartingPose_Request;

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


// Corresponds to easy_handeye2_msgs__srv__CheckStartingPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckStartingPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub can_calibrate: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: super::msg::TargetPoseList,

}



impl Default for CheckStartingPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckStartingPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CheckStartingPose_Response {
  type RmwMsg = super::srv::rmw::CheckStartingPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        can_calibrate: msg.can_calibrate,
        target_poses: super::msg::TargetPoseList::into_rmw_message(std::borrow::Cow::Owned(msg.target_poses)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      can_calibrate: msg.can_calibrate,
        target_poses: super::msg::TargetPoseList::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target_poses)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      can_calibrate: msg.can_calibrate,
      target_poses: super::msg::TargetPoseList::from_rmw_message(msg.target_poses),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__EnumerateTargetPoses_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnumerateTargetPoses_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EnumerateTargetPoses_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnumerateTargetPoses_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EnumerateTargetPoses_Request {
  type RmwMsg = super::srv::rmw::EnumerateTargetPoses_Request;

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


// Corresponds to easy_handeye2_msgs__srv__EnumerateTargetPoses_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnumerateTargetPoses_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: super::msg::TargetPoseList,

}



impl Default for EnumerateTargetPoses_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnumerateTargetPoses_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EnumerateTargetPoses_Response {
  type RmwMsg = super::srv::rmw::EnumerateTargetPoses_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_poses: super::msg::TargetPoseList::into_rmw_message(std::borrow::Cow::Owned(msg.target_poses)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_poses: super::msg::TargetPoseList::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target_poses)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      target_poses: super::msg::TargetPoseList::from_rmw_message(msg.target_poses),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__SelectTargetPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectTargetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_pose_index: i64,

}



impl Default for SelectTargetPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SelectTargetPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SelectTargetPose_Request {
  type RmwMsg = super::srv::rmw::SelectTargetPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_pose_index: msg.target_pose_index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      target_pose_index: msg.target_pose_index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      target_pose_index: msg.target_pose_index,
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__SelectTargetPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectTargetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: super::msg::TargetPoseList,

}



impl Default for SelectTargetPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SelectTargetPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SelectTargetPose_Response {
  type RmwMsg = super::srv::rmw::SelectTargetPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        target_poses: super::msg::TargetPoseList::into_rmw_message(std::borrow::Cow::Owned(msg.target_poses)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        target_poses: super::msg::TargetPoseList::into_rmw_message(std::borrow::Cow::Borrowed(&msg.target_poses)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      target_poses: super::msg::TargetPoseList::from_rmw_message(msg.target_poses),
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanToSelectedTargetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PlanToSelectedTargetPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PlanToSelectedTargetPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PlanToSelectedTargetPose_Request {
  type RmwMsg = super::srv::rmw::PlanToSelectedTargetPose_Request;

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


// Corresponds to easy_handeye2_msgs__srv__PlanToSelectedTargetPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlanToSelectedTargetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for PlanToSelectedTargetPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PlanToSelectedTargetPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PlanToSelectedTargetPose_Response {
  type RmwMsg = super::srv::rmw::PlanToSelectedTargetPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to easy_handeye2_msgs__srv__ExecutePlan_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecutePlan_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ExecutePlan_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ExecutePlan_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ExecutePlan_Request {
  type RmwMsg = super::srv::rmw::ExecutePlan_Request;

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


// Corresponds to easy_handeye2_msgs__srv__ExecutePlan_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecutePlan_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ExecutePlan_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ExecutePlan_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ExecutePlan_Response {
  type RmwMsg = super::srv::rmw::ExecutePlan_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
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


