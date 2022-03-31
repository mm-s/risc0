// Copyright 2022 Risc0, Inc.
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

#include "risc0/r0vm/prove/debug.h"

namespace risc0 {

void DebugState::start(const size_t maxSteps, MemoryHandler& io) {
  init(maxSteps, io);
}

void DebugState::step() {
  // TODO define step interface and event loop
}

} // namespace risc0
