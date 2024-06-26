//
// Copyright 2024 The Project Oak Authors
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
//

use core::ffi::c_size_t;

pub fn syscall_unstable_switch_proccess(pid: c_size_t) -> ! {
    log::debug!("Switching to a different process (pid: {})", pid);
    crate::PROCCESSES.execute(pid).expect("failed to switch to process")
}
