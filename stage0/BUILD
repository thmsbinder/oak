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

# TODO: b/349778041 - CI not to build this for Linux.
rust_library(
    name = "stage0",
    srcs = glob(["src/**"]),
    tags = ["ci-build-for-x86_64-unknown-none"],
    deps = [
        "//oak_core",
        "//oak_dice",
        "//oak_linux_boot_params",
        "//oak_proto_rust",
        "//oak_sev_guest",
        "//oak_sev_snp_attestation_report",
        "//sev_serial",
        "//stage0_dice",
        "@oak_crates_index//:linked_list_allocator",
        "@oak_crates_index//:spinning_top",
        "@oak_crates_index//:static_assertions",
        "@oak_crates_index//:strum",
        "@oak_crates_index//:zerocopy",
        "@oak_no_std_crates_index//:bitflags",
        "@oak_no_std_crates_index//:elf",
        "@oak_no_std_crates_index//:hex",
        "@oak_no_std_crates_index//:hkdf",
        "@oak_no_std_crates_index//:log",
        "@oak_no_std_crates_index//:rand_core",
        "@oak_no_std_crates_index//:sha2",
        "@oak_no_std_crates_index//:zeroize",
    ] + select({
        # TODO: b/347706173 - Try to reduce / remove items in this select by moving all to no_std.
        "@platforms//os:none": [
            "@oak_no_std_crates_index//:coset",
            "@oak_no_std_crates_index//:p256",
            "@oak_no_std_crates_index//:prost",
            "@oak_no_std_crates_index//:x86_64",
        ],
        "//conditions:default": [
            "@oak_crates_index//:coset",
            "@oak_crates_index//:p256",
            "@oak_crates_index//:prost",
            "@oak_crates_index//:x86_64",
        ],
    }),
)