#!/usr/bin/env python3

import cv2
import numpy as np

import rclpy
from rclpy.node import Node

from sensor_msgs.msg import Image, CameraInfo
from cv_bridge import CvBridge

from geometry_msgs.msg import TransformStamped
from tf2_ros import TransformBroadcaster


class ChessboardDetector(Node):

    def __init__(self):
        super().__init__('chessboard_detector')

        # =========================
        # 参数：图像/内参话题、TF 帧名和棋盘格物理尺寸。
        # =========================

        self.declare_parameter('image_topic',
                               '/camera/camera/color/image_raw')

        self.declare_parameter('camera_info_topic',
                               '/camera/camera/color/camera_info')

        self.declare_parameter('camera_frame',
                               'camera_color_optical_frame')

        self.declare_parameter('board_frame',
                               'chessboard')

        self.declare_parameter('square_size',
                               0.025)

        self.declare_parameter('pattern_width',
                               3)

        self.declare_parameter('pattern_height',
                               3)

        self.image_topic = self.get_parameter(
            'image_topic').value

        self.camera_info_topic = self.get_parameter(
            'camera_info_topic').value

        self.camera_frame = self.get_parameter(
            'camera_frame').value

        self.board_frame = self.get_parameter(
            'board_frame').value

        self.square_size = self.get_parameter(
            'square_size').value

        self.pattern_size = (
            self.get_parameter('pattern_width').value,
            self.get_parameter('pattern_height').value
        )

        # =========================
        # OpenCV 图像桥接和 TF 发布器。
        # =========================

        self.bridge = CvBridge()

        self.camera_matrix = None
        self.dist_coeffs = None

        self.tf_broadcaster = TransformBroadcaster(self)

        # =========================
        # 图像与内参分别订阅；收到内参前不会执行位姿估计。
        # =========================

        self.image_sub = self.create_subscription(
            Image,
            self.image_topic,
            self.image_callback,
            10
        )

        self.camera_info_sub = self.create_subscription(
            CameraInfo,
            self.camera_info_topic,
            self.camera_info_callback,
            10
        )

        self.get_logger().info(
            'Chessboard detector started'
        )

        self.get_logger().info(
            f'Pattern: {self.pattern_size[0]}x'
            f'{self.pattern_size[1]} inner corners'
        )

        self.get_logger().info(
            f'Square size: {self.square_size} m'
        )

        self.get_logger().info(
            f'Camera frame: {self.camera_frame}'
        )

        self.get_logger().info(
            f'Board frame: {self.board_frame}'
        )

    # ============================================================
    # Camera info
    # ============================================================

    def camera_info_callback(self, msg):

        self.camera_matrix = np.array(
            msg.k,
            dtype=np.float64
        ).reshape(3, 3)

        self.dist_coeffs = np.array(
            msg.d,
            dtype=np.float64
        )

    # ============================================================
    # Image callback
    # ============================================================

    def image_callback(self, msg):

        if self.camera_matrix is None:
            return

        try:
            image = self.bridge.imgmsg_to_cv2(
                msg,
                desired_encoding='mono8'
            )
        except Exception as e:
            self.get_logger().error(
                f'cv_bridge error: {e}'
            )
            return

        # --------------------------------------------------------
        # Find chessboard
        # --------------------------------------------------------

        flags = (
            cv2.CALIB_CB_EXHAUSTIVE |
            cv2.CALIB_CB_ACCURACY
        )

        found, corners = cv2.findChessboardCornersSB(
            image,
            self.pattern_size,
            flags=flags
        )

        if not found:
            return

        # --------------------------------------------------------
        # 构造棋盘角点在棋盘自身坐标系中的三维坐标。
        #
        # 3x3 inner corners
        #
        # 单位：米。
        # --------------------------------------------------------

        objp = np.zeros(
            (
                self.pattern_size[0] *
                self.pattern_size[1],
                3
            ),
            dtype=np.float32
        )

        objp[:, :2] = np.mgrid[
            0:self.pattern_size[0],
            0:self.pattern_size[1]
        ].T.reshape(-1, 2)

        objp *= self.square_size

        # --------------------------------------------------------
        # 根据三维棋盘点和二维图像角点求 camera_optical -> chessboard。
        # --------------------------------------------------------

        success, rvec, tvec = cv2.solvePnP(
            objp,
            corners,
            self.camera_matrix,
            self.dist_coeffs,
            flags=cv2.SOLVEPNP_ITERATIVE
        )

        if not success:
            return

        # --------------------------------------------------------
        # Rotation matrix
        # --------------------------------------------------------

        rotation_matrix, _ = cv2.Rodrigues(rvec)

        # --------------------------------------------------------
        # Convert rotation matrix -> quaternion
        # --------------------------------------------------------

        qx, qy, qz, qw = self.rotation_matrix_to_quaternion(
            rotation_matrix
        )

        # --------------------------------------------------------
        # 使用图像时间戳发布 TF，避免与机械臂运动时刻错配。
        #
        # camera_color_optical_frame
        #              |
        #              ↓
        #          chessboard
        # --------------------------------------------------------

        transform = TransformStamped()

        transform.header.stamp = msg.header.stamp

        transform.header.frame_id = self.camera_frame
        transform.child_frame_id = self.board_frame

        transform.transform.translation.x = float(tvec[0])
        transform.transform.translation.y = float(tvec[1])
        transform.transform.translation.z = float(tvec[2])

        transform.transform.rotation.x = qx
        transform.transform.rotation.y = qy
        transform.transform.rotation.z = qz
        transform.transform.rotation.w = qw

        self.tf_broadcaster.sendTransform(transform)

    # ============================================================
    # Rotation matrix -> quaternion
    # ============================================================

    @staticmethod
    def rotation_matrix_to_quaternion(R):

        trace = np.trace(R)

        if trace > 0:

            s = 0.5 / np.sqrt(trace + 1.0)

            qw = 0.25 / s
            qx = (R[2, 1] - R[1, 2]) * s
            qy = (R[0, 2] - R[2, 0]) * s
            qz = (R[1, 0] - R[0, 1]) * s

        elif R[0, 0] > R[1, 1] and R[0, 0] > R[2, 2]:

            s = 2.0 * np.sqrt(
                1.0 + R[0, 0] - R[1, 1] - R[2, 2]
            )

            qw = (R[2, 1] - R[1, 2]) / s
            qx = 0.25 * s
            qy = (R[0, 1] + R[1, 0]) / s
            qz = (R[0, 2] + R[2, 0]) / s

        elif R[1, 1] > R[2, 2]:

            s = 2.0 * np.sqrt(
                1.0 + R[1, 1] - R[0, 0] - R[2, 2]
            )

            qw = (R[0, 2] - R[2, 0]) / s
            qx = (R[0, 1] + R[1, 0]) / s
            qy = 0.25 * s
            qz = (R[1, 2] + R[2, 1]) / s

        else:

            s = 2.0 * np.sqrt(
                1.0 + R[2, 2] - R[0, 0] - R[1, 1]
            )

            qw = (R[1, 0] - R[0, 1]) / s
            qx = (R[0, 2] + R[2, 0]) / s
            qy = (R[1, 2] + R[2, 1]) / s
            qz = 0.25 * s

        return qx, qy, qz, qw


def main(args=None):

    rclpy.init(args=args)

    node = ChessboardDetector()

    try:
        rclpy.spin(node)

    except KeyboardInterrupt:
        pass

    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
