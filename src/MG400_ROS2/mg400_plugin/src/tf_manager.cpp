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

#include "mg400_plugin/tf_manager.hpp"

namespace mg400_plugin
{

TFManager & TFManager::getInstance()
{
  static TFManager instance;
  return instance;
}

void TFManager::initialize(const rclcpp::node_interfaces::NodeClockInterface::SharedPtr clock_if)
{
  std::lock_guard<std::mutex> lock(mutex_);

  if (initialized_) {
    return;  // Already initialized
  }

  tf_buffer_ = std::make_shared<tf2_ros::Buffer>(clock_if->get_clock());
  tf_listener_ = std::make_shared<tf2_ros::TransformListener>(*tf_buffer_);
  initialized_ = true;
}

std::shared_ptr<tf2_ros::Buffer> TFManager::getBuffer()
{
  std::lock_guard<std::mutex> lock(mutex_);
  return tf_buffer_;
}

std::shared_ptr<tf2_ros::TransformListener> TFManager::getListener()
{
  std::lock_guard<std::mutex> lock(mutex_);
  return tf_listener_;
}

bool TFManager::isInitialized() const
{
  std::lock_guard<std::mutex> lock(mutex_);
  return initialized_;
}

}  // namespace mg400_plugin
