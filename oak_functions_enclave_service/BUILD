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

load("@rules_rust//rust:defs.bzl", "rust_library")

package(
    default_visibility = ["//visibility:public"],
    licenses = ["notice"],
)

DEPS = [
    "//micro_rpc",
    "//oak_attestation",
    "//oak_core",
    "//oak_crypto",
    "//oak_proto_rust",
    "//oak_restricted_kernel_sdk",
    "@oak_no_std_crates_index//:anyhow",
    "@oak_no_std_crates_index//:log",
    "@oak_no_std_crates_index//:prost",
]

rust_library(
    name = "oak_functions_enclave_service",
    srcs = glob(["src/**"]),
    # The features are inverted on purpose; this means the "default" package will not have sensitive logging enabled.
    crate_features = ["deny_sensitive_logging"],
    target_compatible_with = [
        "//:x86_64-none-setting",
    ],
    deps = DEPS + [
        "//oak_functions_service",
    ],
)

rust_library(
    name = "oak_functions_enclave_service_allow_sensitive_logging",
    srcs = glob(["src/**"]),
    target_compatible_with = [
        "//:x86_64-none-setting",
    ],
    deps = DEPS + [
        "//oak_functions_service:oak_functions_service_allow_sensitive_logging",
    ],
)