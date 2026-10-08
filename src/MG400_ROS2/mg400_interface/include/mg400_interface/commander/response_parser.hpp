// Copyright 2022 HarvestX Inc.
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

#ifndef __MG400_INTERFACE_COMMANDER_RESPONSE_PARSER_HPP__
#define __MG400_INTERFACE_COMMANDER_RESPONSE_PARSER_HPP__


#include <array>
#include <regex>
#include <stdexcept>
#include <string>
#include <vector>

#include "mg400_interface/command_utils.hpp"

namespace mg400_interface
{
typedef struct
{
  int error_id;
  std::string ret_val;
  std::string func_name;
} DashboardResponse;

class DashboardCommandException : public std::runtime_error
{
private:
  DashboardResponse response_;
  std::string error_message_;

public:
  explicit DashboardCommandException(const DashboardResponse & response)
  : std::runtime_error("DashboardCommandException"),
    response_(response)
  {
    this->error_message_ = response_.func_name + " failed: " +
      std::to_string(response_.error_id) + " " + response_.ret_val;
  }

  const DashboardResponse & getDashboardResponse() const
  {
    return response_;
  }

  const char * what() const noexcept override
  {
    return this->error_message_.c_str();
  }
};

class ResponseParser
{
public:
  static bool parseResponse(const std::string &, DashboardResponse &);
  static size_t countArrayElements(const std::string &);
  static std::array<std::vector<int>, 6> takeErrorMessage(const std::string &);
  static std::vector<double> takePoseArray(const std::string &);
  static std::vector<double> takeCartesianPoseArray(const std::string &);
  static std::vector<double> takeCartesianPoseArray4(const std::string &);
  static std::vector<double> takeAngleArray(const std::string &);
  static std::vector<double> takeAngleArray4(const std::string &);
  static int takeInt(const std::string &);
};
}  // namespace mg400_interface
#endif
