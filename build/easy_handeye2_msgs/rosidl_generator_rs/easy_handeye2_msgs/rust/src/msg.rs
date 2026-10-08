#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to easy_handeye2_msgs__msg__HandeyeCalibration

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandeyeCalibration {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::msg::HandeyeCalibrationParameters,


    // This member is not documented.
    #[allow(missing_docs)]
    pub transform: geometry_msgs::msg::Transform,

}



impl Default for HandeyeCalibration {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandeyeCalibration::default())
  }
}

impl rosidl_runtime_rs::Message for HandeyeCalibration {
  type RmwMsg = super::msg::rmw::HandeyeCalibration;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::HandeyeCalibrationParameters::into_rmw_message(std::borrow::Cow::Owned(msg.parameters)).into_owned(),
        transform: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Owned(msg.transform)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::HandeyeCalibrationParameters::into_rmw_message(std::borrow::Cow::Borrowed(&msg.parameters)).into_owned(),
        transform: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Borrowed(&msg.transform)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parameters: super::msg::HandeyeCalibrationParameters::from_rmw_message(msg.parameters),
      transform: geometry_msgs::msg::Transform::from_rmw_message(msg.transform),
    }
  }
}


// Corresponds to easy_handeye2_msgs__msg__HandeyeCalibrationParameters

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HandeyeCalibrationParameters {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub calibration_type: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_base_frame: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_effector_frame: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking_base_frame: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking_marker_frame: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub freehand_robot_movement: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_group_namespace: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_group: std::string::String,

}



impl Default for HandeyeCalibrationParameters {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HandeyeCalibrationParameters::default())
  }
}

impl rosidl_runtime_rs::Message for HandeyeCalibrationParameters {
  type RmwMsg = super::msg::rmw::HandeyeCalibrationParameters;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        calibration_type: msg.calibration_type.as_str().into(),
        robot_base_frame: msg.robot_base_frame.as_str().into(),
        robot_effector_frame: msg.robot_effector_frame.as_str().into(),
        tracking_base_frame: msg.tracking_base_frame.as_str().into(),
        tracking_marker_frame: msg.tracking_marker_frame.as_str().into(),
        freehand_robot_movement: msg.freehand_robot_movement,
        move_group_namespace: msg.move_group_namespace.as_str().into(),
        move_group: msg.move_group.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        calibration_type: msg.calibration_type.as_str().into(),
        robot_base_frame: msg.robot_base_frame.as_str().into(),
        robot_effector_frame: msg.robot_effector_frame.as_str().into(),
        tracking_base_frame: msg.tracking_base_frame.as_str().into(),
        tracking_marker_frame: msg.tracking_marker_frame.as_str().into(),
      freehand_robot_movement: msg.freehand_robot_movement,
        move_group_namespace: msg.move_group_namespace.as_str().into(),
        move_group: msg.move_group.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      calibration_type: msg.calibration_type.to_string(),
      robot_base_frame: msg.robot_base_frame.to_string(),
      robot_effector_frame: msg.robot_effector_frame.to_string(),
      tracking_base_frame: msg.tracking_base_frame.to_string(),
      tracking_marker_frame: msg.tracking_marker_frame.to_string(),
      freehand_robot_movement: msg.freehand_robot_movement,
      move_group_namespace: msg.move_group_namespace.to_string(),
      move_group: msg.move_group.to_string(),
    }
  }
}


// Corresponds to easy_handeye2_msgs__msg__Sample

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sample {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot: geometry_msgs::msg::Transform,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking: geometry_msgs::msg::Transform,

}



impl Default for Sample {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Sample::default())
  }
}

impl rosidl_runtime_rs::Message for Sample {
  type RmwMsg = super::msg::rmw::Sample;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Owned(msg.robot)).into_owned(),
        tracking: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Owned(msg.tracking)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot)).into_owned(),
        tracking: geometry_msgs::msg::Transform::into_rmw_message(std::borrow::Cow::Borrowed(&msg.tracking)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot: geometry_msgs::msg::Transform::from_rmw_message(msg.robot),
      tracking: geometry_msgs::msg::Transform::from_rmw_message(msg.tracking),
    }
  }
}


// Corresponds to easy_handeye2_msgs__msg__SampleList

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SampleList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::msg::HandeyeCalibrationParameters,


    // This member is not documented.
    #[allow(missing_docs)]
    pub samples: Vec<super::msg::Sample>,

}



impl Default for SampleList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SampleList::default())
  }
}

impl rosidl_runtime_rs::Message for SampleList {
  type RmwMsg = super::msg::rmw::SampleList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::HandeyeCalibrationParameters::into_rmw_message(std::borrow::Cow::Owned(msg.parameters)).into_owned(),
        samples: msg.samples
          .into_iter()
          .map(|elem| super::msg::Sample::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::HandeyeCalibrationParameters::into_rmw_message(std::borrow::Cow::Borrowed(&msg.parameters)).into_owned(),
        samples: msg.samples
          .iter()
          .map(|elem| super::msg::Sample::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parameters: super::msg::HandeyeCalibrationParameters::from_rmw_message(msg.parameters),
      samples: msg.samples
          .into_iter()
          .map(super::msg::Sample::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to easy_handeye2_msgs__msg__TargetPoseList

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TargetPoseList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: super::msg::HandeyeCalibrationParameters,


    // This member is not documented.
    #[allow(missing_docs)]
    pub home_pose: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_poses: Vec<geometry_msgs::msg::PoseStamped>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_target_pose_index: i64,

}



impl Default for TargetPoseList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TargetPoseList::default())
  }
}

impl rosidl_runtime_rs::Message for TargetPoseList {
  type RmwMsg = super::msg::rmw::TargetPoseList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::HandeyeCalibrationParameters::into_rmw_message(std::borrow::Cow::Owned(msg.parameters)).into_owned(),
        home_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.home_pose)).into_owned(),
        target_poses: msg.target_poses
          .into_iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        current_target_pose_index: msg.current_target_pose_index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameters: super::msg::HandeyeCalibrationParameters::into_rmw_message(std::borrow::Cow::Borrowed(&msg.parameters)).into_owned(),
        home_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.home_pose)).into_owned(),
        target_poses: msg.target_poses
          .iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      current_target_pose_index: msg.current_target_pose_index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parameters: super::msg::HandeyeCalibrationParameters::from_rmw_message(msg.parameters),
      home_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.home_pose),
      target_poses: msg.target_poses
          .into_iter()
          .map(geometry_msgs::msg::PoseStamped::from_rmw_message)
          .collect(),
      current_target_pose_index: msg.current_target_pose_index,
    }
  }
}


