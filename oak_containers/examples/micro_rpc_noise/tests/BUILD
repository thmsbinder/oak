#
# Copyright 2024 The Project Oak Authors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

load("@rules_rust//rust:defs.bzl", "rust_test_suite")

package(
    default_visibility = ["//:internal"],
    licenses = ["notice"],
)

rust_test_suite(
    name = "micro_rpc_noise_tests",
    srcs = [
        "tests.rs",
    ],
    proc_macro_deps = ["@oak_crates_index//:async-trait"],
    deps = [
        "//micro_rpc",
        "//oak_containers/examples/micro_rpc_noise/proto:micro_rpc_noise_rust_proto",
        "//oak_containers/examples/micro_rpc_noise/service:micro_rpc_noise_service",
        "//oak_proto_rust",
        "//oak_session",
        "@oak_crates_index//:anyhow",
        "@oak_crates_index//:tokio",
    ],
)
