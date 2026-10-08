// Copyright 2023 HarvestX Inc.
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

#ifndef __MG400_COMMON_MG400_IK_UTIL_HPP__
#define __MG400_COMMON_MG400_IK_UTIL_HPP__

#include <vector>

namespace mg400_common
{
class MG400IKUtil
{
private:
  static constexpr double ROUND_DECIMALS = 8;

public:
  MG400IKUtil() = default;
  bool InMG400Range(const std::vector<double> &);
  std::vector<double> InverseKinematics(const std::vector<double> &);
  std::vector<double> ToolCoordToBaseCoord(
    const std::vector<double> &,
    const std::vector<double> &);
};

}  // namespace mg400_common
#endif
