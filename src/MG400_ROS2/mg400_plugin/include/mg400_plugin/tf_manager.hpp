// Copyright 2025 HarvestX Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef __MG400_PLUGIN_TF_MANAGER_HPP__
#define __MG400_PLUGIN_TF_MANAGER_HPP__

#include <memory>
#include <mutex>

#include <rclcpp/rclcpp.hpp>
#include <tf2_ros/buffer.h>
#include <tf2_ros/transform_listener.h>

namespace mg400_plugin
{

/**
 * @brief Thread-safe singleton class for managing TF functionality
 *
 * This class provides a centralized way to access TF buffer and listener,
 * preventing multiple instances from being created across different plugins.
 */
class TFManager final
{
public:
  /**
   * @brief Get the singleton instance of TFManager
   *
   * @return TFManager& Reference to the singleton instance
   */
  static TFManager & getInstance();

  /**
   * @brief Initialize the TF manager with a clock interface
   *
   * @param clock_if Clock interface for the TF buffer
   */
  void initialize(const rclcpp::node_interfaces::NodeClockInterface::SharedPtr clock_if);

  /**
   * @brief Get the TF buffer
   *
   * @return std::shared_ptr<tf2_ros::Buffer> Shared pointer to the TF buffer
   */
  std::shared_ptr<tf2_ros::Buffer> getBuffer();

  /**
   * @brief Get the TF listener
   *
   * @return std::shared_ptr<tf2_ros::TransformListener> Shared pointer to the TF listener
   */
  std::shared_ptr<tf2_ros::TransformListener> getListener();

  /**
   * @brief Check if the TF manager is initialized
   *
   * @return true if initialized, false otherwise
   */
  bool isInitialized() const;

  // Delete copy constructor and assignment operator to ensure singleton
  TFManager(const TFManager &) = delete;
  TFManager & operator=(const TFManager &) = delete;

private:
  TFManager() = default;
  ~TFManager() = default;

  std::shared_ptr<tf2_ros::Buffer> tf_buffer_;
  std::shared_ptr<tf2_ros::TransformListener> tf_listener_;
  bool initialized_;

  mutable std::mutex mutex_;
};

}  // namespace mg400_plugin

#endif  // __MG400_PLUGIN_TF_MANAGER_HPP__
