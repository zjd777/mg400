
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mg400_msgs__action__CommandQueue_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub commands: Vec<super::msg::Command>,

}



impl Default for CommandQueue_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_Goal {
  type RmwMsg = super::action::rmw::CommandQueue_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        commands: msg.commands
          .into_iter()
          .map(|elem| super::msg::Command::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        commands: msg.commands
          .iter()
          .map(|elem| super::msg::Command::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      commands: msg.commands
          .into_iter()
          .map(super::msg::Command::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to mg400_msgs__action__CommandQueue_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::msg::ErrorID,

}



impl Default for CommandQueue_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_Result::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_Result {
  type RmwMsg = super::action::rmw::CommandQueue_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: super::msg::ErrorID::from_rmw_message(msg.error_id),
    }
  }
}


// Corresponds to mg400_msgs__action__CommandQueue_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::PoseStamped,

    /// Current joint angles in radian
    pub current_angles: [f64; 4],

}



impl Default for CommandQueue_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_Feedback {
  type RmwMsg = super::action::rmw::CommandQueue_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_pose)).into_owned(),
        current_angles: msg.current_angles,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_pose)).into_owned(),
        current_angles: msg.current_angles,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_pose),
      current_angles: msg.current_angles,
    }
  }
}


// Corresponds to mg400_msgs__action__CommandQueue_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::CommandQueue_Feedback,

}



impl Default for CommandQueue_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_FeedbackMessage {
  type RmwMsg = super::action::rmw::CommandQueue_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::CommandQueue_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::CommandQueue_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::CommandQueue_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Goal {
  type RmwMsg = super::action::rmw::JointMovJ_Goal;

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


// Corresponds to mg400_msgs__action__JointMovJ_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::msg::ErrorID,

}



impl Default for JointMovJ_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_Result::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Result {
  type RmwMsg = super::action::rmw::JointMovJ_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: super::msg::ErrorID::from_rmw_message(msg.error_id),
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_angles: [f64; 4],

}



impl Default for JointMovJ_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_Feedback {
  type RmwMsg = super::action::rmw::JointMovJ_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_angles: msg.current_angles,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_angles: msg.current_angles,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_angles: msg.current_angles,
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::JointMovJ_Feedback,

}



impl Default for JointMovJ_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_FeedbackMessage {
  type RmwMsg = super::action::rmw::JointMovJ_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::JointMovJ_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::JointMovJ_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::JointMovJ_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Goal {

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



impl Default for MovJ_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_Goal {
  type RmwMsg = super::action::rmw::MovJ_Goal;

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


// Corresponds to mg400_msgs__action__MovJ_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::msg::ErrorID,

}



impl Default for MovJ_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_Result {
  type RmwMsg = super::action::rmw::MovJ_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: super::msg::ErrorID::from_rmw_message(msg.error_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::PoseStamped,

}



impl Default for MovJ_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_Feedback {
  type RmwMsg = super::action::rmw::MovJ_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_pose),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MovJ_Feedback,

}



impl Default for MovJ_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_FeedbackMessage {
  type RmwMsg = super::action::rmw::MovJ_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MovJ_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MovJ_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MovJ_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Goal {

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


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for MovJIO_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Goal {
  type RmwMsg = super::action::rmw::MovJIO_Goal;

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
        set_cp: msg.set_cp,
        cp: msg.cp,
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
      set_cp: msg.set_cp,
      cp: msg.cp,
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
      set_cp: msg.set_cp,
      cp: msg.cp,
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::msg::ErrorID,

}



impl Default for MovJIO_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Result {
  type RmwMsg = super::action::rmw::MovJIO_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: super::msg::ErrorID::from_rmw_message(msg.error_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::PoseStamped,

}



impl Default for MovJIO_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Feedback {
  type RmwMsg = super::action::rmw::MovJIO_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_pose),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MovJIO_Feedback,

}



impl Default for MovJIO_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_FeedbackMessage {
  type RmwMsg = super::action::rmw::MovJIO_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MovJIO_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MovJIO_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MovJIO_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Goal {

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



impl Default for MovL_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_Goal {
  type RmwMsg = super::action::rmw::MovL_Goal;

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


// Corresponds to mg400_msgs__action__MovL_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::msg::ErrorID,

}



impl Default for MovL_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_Result {
  type RmwMsg = super::action::rmw::MovL_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: super::msg::ErrorID::from_rmw_message(msg.error_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::PoseStamped,

}



impl Default for MovL_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_Feedback {
  type RmwMsg = super::action::rmw::MovL_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_pose),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MovL_Feedback,

}



impl Default for MovL_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_FeedbackMessage {
  type RmwMsg = super::action::rmw::MovL_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MovL_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MovL_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MovL_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Goal {

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


    // This member is not documented.
    #[allow(missing_docs)]
    pub set_cp: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cp: u8,

}



impl Default for MovLIO_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Goal {
  type RmwMsg = super::action::rmw::MovLIO_Goal;

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
        set_cp: msg.set_cp,
        cp: msg.cp,
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
      set_cp: msg.set_cp,
      cp: msg.cp,
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
      set_cp: msg.set_cp,
      cp: msg.cp,
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_id: super::msg::ErrorID,

}



impl Default for MovLIO_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Result {
  type RmwMsg = super::action::rmw::MovLIO_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Owned(msg.error_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
        error_id: super::msg::ErrorID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.error_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
      error_id: super::msg::ErrorID::from_rmw_message(msg.error_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_pose: geometry_msgs::msg::PoseStamped,

}



impl Default for MovLIO_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Feedback {
  type RmwMsg = super::action::rmw::MovLIO_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_pose),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MovLIO_Feedback,

}



impl Default for MovLIO_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_FeedbackMessage {
  type RmwMsg = super::action::rmw::MovLIO_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MovLIO_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MovLIO_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MovLIO_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to mg400_msgs__action__CommandQueue_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::CommandQueue_Goal,

}



impl Default for CommandQueue_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_SendGoal_Request {
  type RmwMsg = super::action::rmw::CommandQueue_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::CommandQueue_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::CommandQueue_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::CommandQueue_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to mg400_msgs__action__CommandQueue_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for CommandQueue_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_SendGoal_Response {
  type RmwMsg = super::action::rmw::CommandQueue_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to mg400_msgs__action__CommandQueue_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for CommandQueue_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_GetResult_Request {
  type RmwMsg = super::action::rmw::CommandQueue_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to mg400_msgs__action__CommandQueue_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandQueue_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::CommandQueue_Result,

}



impl Default for CommandQueue_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::CommandQueue_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CommandQueue_GetResult_Response {
  type RmwMsg = super::action::rmw::CommandQueue_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::CommandQueue_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::CommandQueue_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::CommandQueue_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::JointMovJ_Goal,

}



impl Default for JointMovJ_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_SendGoal_Request {
  type RmwMsg = super::action::rmw::JointMovJ_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::JointMovJ_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::JointMovJ_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::JointMovJ_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for JointMovJ_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_SendGoal_Response {
  type RmwMsg = super::action::rmw::JointMovJ_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for JointMovJ_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_GetResult_Request {
  type RmwMsg = super::action::rmw::JointMovJ_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to mg400_msgs__action__JointMovJ_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JointMovJ_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::JointMovJ_Result,

}



impl Default for JointMovJ_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::JointMovJ_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for JointMovJ_GetResult_Response {
  type RmwMsg = super::action::rmw::JointMovJ_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::JointMovJ_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::JointMovJ_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::JointMovJ_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MovJ_Goal,

}



impl Default for MovJ_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_SendGoal_Request {
  type RmwMsg = super::action::rmw::MovJ_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MovJ_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MovJ_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MovJ_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MovJ_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_SendGoal_Response {
  type RmwMsg = super::action::rmw::MovJ_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MovJ_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_GetResult_Request {
  type RmwMsg = super::action::rmw::MovJ_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJ_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MovJ_Result,

}



impl Default for MovJ_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJ_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_GetResult_Response {
  type RmwMsg = super::action::rmw::MovJ_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MovJ_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MovJ_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MovJ_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MovJIO_Goal,

}



impl Default for MovJIO_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_SendGoal_Request {
  type RmwMsg = super::action::rmw::MovJIO_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MovJIO_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MovJIO_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MovJIO_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MovJIO_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_SendGoal_Response {
  type RmwMsg = super::action::rmw::MovJIO_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MovJIO_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_GetResult_Request {
  type RmwMsg = super::action::rmw::MovJIO_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovJIO_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MovJIO_Result,

}



impl Default for MovJIO_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovJIO_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_GetResult_Response {
  type RmwMsg = super::action::rmw::MovJIO_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MovJIO_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MovJIO_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MovJIO_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MovL_Goal,

}



impl Default for MovL_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_SendGoal_Request {
  type RmwMsg = super::action::rmw::MovL_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MovL_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MovL_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MovL_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MovL_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_SendGoal_Response {
  type RmwMsg = super::action::rmw::MovL_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MovL_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_GetResult_Request {
  type RmwMsg = super::action::rmw::MovL_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovL_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MovL_Result,

}



impl Default for MovL_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovL_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_GetResult_Response {
  type RmwMsg = super::action::rmw::MovL_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MovL_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MovL_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MovL_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MovLIO_Goal,

}



impl Default for MovLIO_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_SendGoal_Request {
  type RmwMsg = super::action::rmw::MovLIO_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MovLIO_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MovLIO_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MovLIO_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MovLIO_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_SendGoal_Response {
  type RmwMsg = super::action::rmw::MovLIO_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MovLIO_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_GetResult_Request {
  type RmwMsg = super::action::rmw::MovLIO_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to mg400_msgs__action__MovLIO_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MovLIO_Result,

}



impl Default for MovLIO_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MovLIO_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_GetResult_Response {
  type RmwMsg = super::action::rmw::MovLIO_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MovLIO_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MovLIO_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MovLIO_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__CommandQueue() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__CommandQueue
#[allow(missing_docs, non_camel_case_types)]
pub struct CommandQueue;

impl rosidl_runtime_rs::Action for CommandQueue {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = CommandQueue_Goal;

  /// The result message defined in the action definition.
  type Result = CommandQueue_Result;

  /// The feedback message defined in the action definition.
  type Feedback = CommandQueue_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::CommandQueue_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::CommandQueue_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::CommandQueue_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__CommandQueue() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::CommandQueue_Goal,
  ) -> super::action::rmw::CommandQueue_SendGoal_Request {
   super::action::rmw::CommandQueue_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::CommandQueue_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::CommandQueue_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::CommandQueue_SendGoal_Response {
   super::action::rmw::CommandQueue_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::CommandQueue_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::CommandQueue_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::CommandQueue_Feedback,
  ) -> super::action::rmw::CommandQueue_FeedbackMessage {
    let mut message = super::action::rmw::CommandQueue_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::CommandQueue_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::CommandQueue_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::CommandQueue_GetResult_Request {
   super::action::rmw::CommandQueue_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::CommandQueue_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::CommandQueue_Result,
  ) -> super::action::rmw::CommandQueue_GetResult_Response {
   super::action::rmw::CommandQueue_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::CommandQueue_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::CommandQueue_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__JointMovJ() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__JointMovJ
#[allow(missing_docs, non_camel_case_types)]
pub struct JointMovJ;

impl rosidl_runtime_rs::Action for JointMovJ {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = JointMovJ_Goal;

  /// The result message defined in the action definition.
  type Result = JointMovJ_Result;

  /// The feedback message defined in the action definition.
  type Feedback = JointMovJ_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::JointMovJ_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::JointMovJ_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::JointMovJ_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__JointMovJ() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::JointMovJ_Goal,
  ) -> super::action::rmw::JointMovJ_SendGoal_Request {
   super::action::rmw::JointMovJ_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::JointMovJ_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::JointMovJ_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::JointMovJ_SendGoal_Response {
   super::action::rmw::JointMovJ_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::JointMovJ_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::JointMovJ_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::JointMovJ_Feedback,
  ) -> super::action::rmw::JointMovJ_FeedbackMessage {
    let mut message = super::action::rmw::JointMovJ_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::JointMovJ_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::JointMovJ_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::JointMovJ_GetResult_Request {
   super::action::rmw::JointMovJ_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::JointMovJ_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::JointMovJ_Result,
  ) -> super::action::rmw::JointMovJ_GetResult_Response {
   super::action::rmw::JointMovJ_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::JointMovJ_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::JointMovJ_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovJ() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovJ
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJ;

impl rosidl_runtime_rs::Action for MovJ {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MovJ_Goal;

  /// The result message defined in the action definition.
  type Result = MovJ_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MovJ_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MovJ_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MovJ_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MovJ_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovJ() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MovJ_Goal,
  ) -> super::action::rmw::MovJ_SendGoal_Request {
   super::action::rmw::MovJ_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MovJ_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MovJ_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MovJ_SendGoal_Response {
   super::action::rmw::MovJ_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MovJ_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MovJ_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MovJ_Feedback,
  ) -> super::action::rmw::MovJ_FeedbackMessage {
    let mut message = super::action::rmw::MovJ_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MovJ_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MovJ_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MovJ_GetResult_Request {
   super::action::rmw::MovJ_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MovJ_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MovJ_Result,
  ) -> super::action::rmw::MovJ_GetResult_Response {
   super::action::rmw::MovJ_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MovJ_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MovJ_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovJIO() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovJIO
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJIO;

impl rosidl_runtime_rs::Action for MovJIO {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MovJIO_Goal;

  /// The result message defined in the action definition.
  type Result = MovJIO_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MovJIO_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MovJIO_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MovJIO_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MovJIO_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovJIO() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MovJIO_Goal,
  ) -> super::action::rmw::MovJIO_SendGoal_Request {
   super::action::rmw::MovJIO_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MovJIO_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MovJIO_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MovJIO_SendGoal_Response {
   super::action::rmw::MovJIO_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MovJIO_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MovJIO_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MovJIO_Feedback,
  ) -> super::action::rmw::MovJIO_FeedbackMessage {
    let mut message = super::action::rmw::MovJIO_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MovJIO_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MovJIO_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MovJIO_GetResult_Request {
   super::action::rmw::MovJIO_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MovJIO_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MovJIO_Result,
  ) -> super::action::rmw::MovJIO_GetResult_Response {
   super::action::rmw::MovJIO_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MovJIO_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MovJIO_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovL() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovL
#[allow(missing_docs, non_camel_case_types)]
pub struct MovL;

impl rosidl_runtime_rs::Action for MovL {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MovL_Goal;

  /// The result message defined in the action definition.
  type Result = MovL_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MovL_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MovL_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MovL_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MovL_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovL() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MovL_Goal,
  ) -> super::action::rmw::MovL_SendGoal_Request {
   super::action::rmw::MovL_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MovL_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MovL_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MovL_SendGoal_Response {
   super::action::rmw::MovL_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MovL_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MovL_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MovL_Feedback,
  ) -> super::action::rmw::MovL_FeedbackMessage {
    let mut message = super::action::rmw::MovL_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MovL_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MovL_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MovL_GetResult_Request {
   super::action::rmw::MovL_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MovL_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MovL_Result,
  ) -> super::action::rmw::MovL_GetResult_Response {
   super::action::rmw::MovL_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MovL_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MovL_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "mg400_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovLIO() -> *const std::ffi::c_void;
}

// Corresponds to mg400_msgs__action__MovLIO
#[allow(missing_docs, non_camel_case_types)]
pub struct MovLIO;

impl rosidl_runtime_rs::Action for MovLIO {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MovLIO_Goal;

  /// The result message defined in the action definition.
  type Result = MovLIO_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MovLIO_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MovLIO_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MovLIO_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MovLIO_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__mg400_msgs__action__MovLIO() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MovLIO_Goal,
  ) -> super::action::rmw::MovLIO_SendGoal_Request {
   super::action::rmw::MovLIO_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MovLIO_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MovLIO_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MovLIO_SendGoal_Response {
   super::action::rmw::MovLIO_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MovLIO_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MovLIO_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MovLIO_Feedback,
  ) -> super::action::rmw::MovLIO_FeedbackMessage {
    let mut message = super::action::rmw::MovLIO_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MovLIO_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MovLIO_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MovLIO_GetResult_Request {
   super::action::rmw::MovLIO_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MovLIO_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MovLIO_Result,
  ) -> super::action::rmw::MovLIO_GetResult_Response {
   super::action::rmw::MovLIO_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MovLIO_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MovLIO_Result,
  ) {
    (response.status, response.result)
  }
}


