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

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

package(
    default_visibility = ["//visibility:public"],
    licenses = ["notice"],
)

exports_files(["README.md"])

LIB_DEPS = [
    "//micro_rpc",
    "//oak_attestation",
    "//oak_containers/agent:lib",
    "//oak_containers/orchestrator:oak_containers_orchestrator",
    "//oak_containers_sdk",
    "//oak_crypto",
    "//oak_debug_service",
    "//oak_functions_abi",
    "//oak_proto_rust",
    "//oak_proto_rust/grpc",
    "@oak_crates_index//:anyhow",
    "@oak_crates_index//:http",
    "@oak_crates_index//:libloading",
    "@oak_crates_index//:opentelemetry",
    "@oak_crates_index//:ouroboros",
    "@oak_crates_index//:prost",
    "@oak_crates_index//:tempfile",
    "@oak_crates_index//:tikv-jemallocator",
    "@oak_crates_index//:tokio",
    "@oak_crates_index//:tokio-stream",
    "@oak_crates_index//:tonic",
    "@oak_crates_index//:tower",
    "@oak_crates_index//:tower-http",
    "@oak_crates_index//:tracing",
]

LIB_SRCS = [
    "src/lib.rs",
    "src/native_handler.rs",
    "tests/integration_test.rs",
    "tests/native_test.rs",
]

rust_library(
    name = "lib",
    srcs = LIB_SRCS,
    compile_data = [":README.md"],
    crate_features = [
        "deny_sensitive_logging",
        "native",
    ],
    crate_name = "oak_functions_containers_app",
    deps = LIB_DEPS + ["//oak_functions_service:lib_unrestricted"],
)

rust_library(
    name = "lib_insecure",
    srcs = LIB_SRCS,
    compile_data = [":README.md"],
    crate_features = [
        "allow_sensitive_logging",
        "native",
    ],
    crate_name = "oak_functions_containers_app",
    deps = LIB_DEPS + ["//oak_functions_service:lib_insecure_unrestricted"],
)

APP_DEPS = [
    "//oak_containers/agent:lib",
    "//oak_containers/orchestrator:lib",
    "//oak_containers_sdk",
    "//oak_crypto",
    "//oak_proto_rust",
    "@oak_crates_index//:anyhow",
    "@oak_crates_index//:clap",
    "@oak_crates_index//:log",
    "@oak_crates_index//:opentelemetry",
    "@oak_crates_index//:opentelemetry-otlp",
    "@oak_crates_index//:opentelemetry_sdk",
    "@oak_crates_index//:prost",
    "@oak_crates_index//:stderrlog",
    "@oak_crates_index//:tikv-jemallocator",
    "@oak_crates_index//:tonic",
    "@oak_crates_index//:tokio",
    "@oak_crates_index//:tokio-stream",
    "@oak_crates_index//:tokio-vsock",
]

rust_binary(
    name = "oak_functions_containers_app",
    srcs = ["src/main.rs"],
    crate_features = [
        "deny_sensitive_logging",
        "native",
    ],
    deps = APP_DEPS + [
        ":lib",
        "//oak_functions_service:lib_unrestricted",
    ],
)

rust_binary(
    name = "oak_functions_containers_app_insecure",
    srcs = ["src/main.rs"],
    crate_features = [
        "allow_sensitive_logging",
        "native",
    ],
    deps = APP_DEPS + [
        ":lib_insecure",
        "//oak_functions_service:lib_insecure_unrestricted",
    ],
)

rust_test(
    name = "lib_test",
    crate = ":lib",
)
