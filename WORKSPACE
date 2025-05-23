#
# Copyright 2018 The Project Oak Authors
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

workspace(name = "oak")

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

# The `name` argument in all `http_archive` rules should be equal to the
# WORKSPACE name of the corresponding library.

# Google Abseil.
# https://github.com/abseil/abseil-cpp
http_archive(
    name = "com_google_absl",
    sha256 = "f50e5ac311a81382da7fa75b97310e4b9006474f9560ac46f54a9967f07d4ae3",
    strip_prefix = "abseil-cpp-20240722.0",
    urls = [
        # Abseil LTS 20240116.1
        "https://github.com/abseil/abseil-cpp/archive/refs/tags/20240722.0.tar.gz",
    ],
)

# BoringSSL.
# https://github.com/google/boringssl
http_archive(
    name = "bssl",
    patch_args = ["-p1"],
    patch_tool = "patch",
    patches = ["//third_party/boringssl:boringssl.patch"],
    sha256 = "a6d197fe3e5ad4d59dd5912a6ec9a8b69fc87f496ab11e05e7d0017b0ec70ecd",
    strip_prefix = "boringssl-7a6e828dc53ba9a56bd49915f2a0780d63af97d2",
    urls = [
        # Head commit on 2024-07-25.
        "https://github.com/google/boringssl/archive/7a6e828dc53ba9a56bd49915f2a0780d63af97d2.zip",
    ],
)

# GoogleTest
# https://github.com/google/googletest
http_archive(
    name = "com_google_googletest",
    sha256 = "8c0ceafa3ea24bf78e3519b7846d99e76c45899aa4dac4d64e7dd62e495de9fd",
    strip_prefix = "googletest-b514bdc898e2951020cbdca1304b75f5950d1f59",
    urls = [
        # Latest commit for version 1.13.0. This requires at least C++14.
        "https://github.com/google/googletest/archive/b514bdc898e2951020cbdca1304b75f5950d1f59.zip",
    ],
)

# GoogleBenchmark
# https://github.com/google/benchmark
http_archive(
    name = "com_github_google_benchmark",
    strip_prefix = "benchmark-1.9.1",
    urls = [
        "https://github.com/google/benchmark/archive/refs/tags/v1.9.1.zip",
    ],
)

# C++ gRPC support.
# https://github.com/grpc/grpc
http_archive(
    name = "com_github_grpc_grpc",
    sha256 = "f40bde4ce2f31760f65dc49a2f50876f59077026494e67dccf23992548b1b04f",
    strip_prefix = "grpc-1.62.0",
    urls = [
        "https://github.com/grpc/grpc/archive/refs/tags/v1.62.0.tar.gz",
    ],
)

load("@com_github_grpc_grpc//bazel:grpc_deps.bzl", "grpc_deps")

grpc_deps()

load("@com_github_grpc_grpc//bazel:grpc_extra_deps.bzl", "grpc_extra_deps")

grpc_extra_deps()

# Java gRPC support.
# https://github.com/grpc/grpc-java
http_archive(
    name = "io_grpc_grpc_java",
    sha256 = "4a37fbdf88c8344e14a12bb261aa3eb1401fa47cfc312fb82260592aa993171a",
    strip_prefix = "grpc-java-1.62.0",
    urls = [
        "https://github.com/grpc/grpc-java/archive/refs/tags/v1.62.0.tar.gz",
    ],
)

load("@io_grpc_grpc_java//:repositories.bzl", "IO_GRPC_GRPC_JAVA_ARTIFACTS", "IO_GRPC_GRPC_JAVA_OVERRIDE_TARGETS", "grpc_java_repositories")

grpc_java_repositories()

# Kotlin gRPC
http_archive(
    name = "com_github_grpc_grpc_kotlin",
    repo_mapping = {"@io_bazel_rules_kotlin": "@rules_kotlin"},
    strip_prefix = "grpc-kotlin-1.4.2",
    url = "https://github.com/grpc/grpc-kotlin/archive/refs/tags/v1.4.2.tar.gz",
)

load(
    "@com_github_grpc_grpc_kotlin//:repositories.bzl",
    "IO_GRPC_GRPC_KOTLIN_ARTIFACTS",
    "IO_GRPC_GRPC_KOTLIN_OVERRIDE_TARGETS",
    "grpc_kt_repositories",
)

grpc_kt_repositories()

### --- Base Proto Support --- ###
http_archive(
    name = "com_google_protobuf",
    sha256 = "63150aba23f7a90fd7d87bdf514e459dd5fe7023fdde01b56ac53335df64d4bd",
    strip_prefix = "protobuf-29.2",
    url = "https://github.com/protocolbuffers/protobuf/releases/download/v29.2/protobuf-29.2.tar.gz",
)

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

protobuf_deps()

# External Java rules.
# https://github.com/bazelbuild/rules_jvm_external
http_archive(
    name = "rules_jvm_external",
    sha256 = "f86fd42a809e1871ca0aabe89db0d440451219c3ce46c58da240c7dcdc00125f",
    strip_prefix = "rules_jvm_external-5.2",
    urls = [
        # Rules Java v5.2 (2023-04-13).
        "https://github.com/bazelbuild/rules_jvm_external/releases/download/5.2/rules_jvm_external-5.2.tar.gz",
    ],
)

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

# Maven rules.
load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    artifacts = [
        "co.nstant.in:cbor:0.9",
        "com.google.crypto.tink:tink:1.12.0",
        "com.google.flogger:flogger-system-backend:0.8",
        "com.google.flogger:google-extensions:0.8",
        "com.google.protobuf:protobuf-kotlin:3.18.0",
        "org.assertj:assertj-core:3.12.1",
        "org.bouncycastle:bcpkix-jdk18on:1.77",
        "org.bouncycastle:bcprov-jdk18on:1.77",
        "org.jetbrains.kotlin:kotlin-test:2.0.0",
        "org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3",
        "org.jetbrains.kotlinx:kotlinx-coroutines-test:1.7.3",
        "org.mockito:mockito-core:3.3.3",
    ] + IO_GRPC_GRPC_JAVA_ARTIFACTS + IO_GRPC_GRPC_KOTLIN_ARTIFACTS,
    generate_compat_repositories = True,
    override_targets = dict(
        IO_GRPC_GRPC_KOTLIN_OVERRIDE_TARGETS.items() +
        IO_GRPC_GRPC_JAVA_OVERRIDE_TARGETS.items(),
    ),
    repositories = [
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)

load("@maven//:compat.bzl", "compat_repositories")

compat_repositories()

# Bazel rules for Android applications.
# https://github.com/bazelbuild/rules_android
http_archive(
    name = "build_bazel_rules_android",
    sha256 = "cd06d15dd8bb59926e4d65f9003bfc20f9da4b2519985c27e190cddc8b7a7806",
    strip_prefix = "rules_android-0.1.1",
    urls = ["https://github.com/bazelbuild/rules_android/archive/v0.1.1.zip"],
)

load("@build_bazel_rules_android//android:rules.bzl", "android_sdk_repository")

android_sdk_repository(
    name = "androidsdk",
    api_level = 30,
    build_tools_version = "30.0.0",
)

http_archive(
    name = "rules_foreign_cc",
    sha256 = "5816f4198184a1e0e682d7e6b817331219929401e2f18358fac7f7b172737976",
    strip_prefix = "rules_foreign_cc-0.10.0",
    url = "https://github.com/bazelbuild/rules_foreign_cc/archive/refs/tags/0.10.0.tar.gz",
)

load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")

# This sets up some common toolchains for building targets. For more details, please see
# https://bazelbuild.github.io/rules_foreign_cc/0.9.0/flatten.html#rules_foreign_cc_dependencies
rules_foreign_cc_dependencies()

http_archive(
    name = "rules_kotlin",
    sha256 = "d9898c3250e0442436eeabde4e194c30d6c76a4a97f517b18cefdfd4e345725a",
    url = "https://github.com/bazelbuild/rules_kotlin/releases/download/v1.9.1/rules_kotlin-v1.9.1.tar.gz",
)

load("@rules_kotlin//kotlin:repositories.bzl", "kotlin_repositories")

kotlin_repositories()  # if you want the default. Otherwise see custom kotlinc distribution below

load("@rules_kotlin//kotlin:core.bzl", "kt_register_toolchains")

kt_register_toolchains()  # to use the default toolchain, otherwise see toolchains below

# C++ CBOR support.
# https://android.googlesource.com/platform/external/libcppbor
git_repository(
    name = "libcppbor",
    build_file = "@//:third_party/google/libcppbor/BUILD",
    # Head commit on 2023-12-04.
    commit = "20d2be8672d24bfb441d075f82cc317d17d601f8",
    patches = [
        "@//:third_party/google/libcppbor/remove_macro.patch",
        "@//:third_party/google/libcppbor/limits.patch",
    ],
    remote = "https://android.googlesource.com/platform/external/libcppbor",
)

http_archive(
    name = "cose_lib",
    build_file = "@//:third_party/BUILD.cose_lib",
    sha256 = "e41a068b573bb07ed2a50cb3c39ae10995977cad82e24a7873223277e7fdb4e5",
    strip_prefix = "cose-lib-2023.09.08",
    url = "https://github.com/android/cose-lib/archive/refs/tags/v2023.09.08.tar.gz",
)

# Run clang-tidy on C++ code with the following command:
# bazel build //cc/... \
#   --aspects=@bazel_clang_tidy//clang_tidy:clang_tidy.bzl%clang_tidy_aspect \
#   --output_groups=report
git_repository(
    name = "bazel_clang_tidy",
    commit = "f43f9d61c201b314c62a3ebcf2d4a37f1a3b06f7",
    remote = "https://github.com/erenon/bazel_clang_tidy.git",
)

# Bazel rules for building OCI images and runtime bundles.
http_archive(
    name = "rules_oci",
    sha256 = "56d5499025d67a6b86b2e6ebae5232c72104ae682b5a21287770bd3bf0661abf",
    strip_prefix = "rules_oci-1.7.5",
    url = "https://github.com/bazel-contrib/rules_oci/releases/download/v1.7.5/rules_oci-v1.7.5.tar.gz",
)

load("@rules_oci//oci:dependencies.bzl", "rules_oci_dependencies")

rules_oci_dependencies()

load("@rules_oci//oci:repositories.bzl", "LATEST_CRANE_VERSION", "LATEST_ZOT_VERSION", "oci_register_toolchains")

oci_register_toolchains(
    name = "oci",
    crane_version = LATEST_CRANE_VERSION,
    zot_version = LATEST_ZOT_VERSION,
)

load("@rules_oci//oci:pull.bzl", "oci_pull")

# This is the base docker image we use to bundle example apps like hello world
# enclave apps. We don't build these, we pull them from the existing repo.
#
# E.g.: //oak_containers/examples/hello_world/enclave_app:bundle . You can find
# these images at: gcr.io/distroless/cc-debian12 . We do not need root access
# so you can search with ":nonroot" (gcr.io/distroless/cc-debian12:nonroot) or
# "latest" (gcr.io/distroless/cc-debian12:latest). Note files tagged as ".sig"
# or ".att" do not contain images. You can find a given digest (like the one
# below) at http://gcr.io/distroless/cc-debian12@{digest} where {digest}
# includes the "sha256:" bit.
oci_pull(
    name = "distroless_cc_debian12",
    digest = "sha256:6714977f9f02632c31377650c15d89a7efaebf43bab0f37c712c30fc01edb973",
    image = "gcr.io/distroless/cc-debian12",
    platforms = ["linux/amd64"],
)

load("@aspect_bazel_lib//lib:repositories.bzl", "register_expand_template_toolchains")

register_expand_template_toolchains()

load("@//bazel:repositories.bzl", "oak_toolchain_repositories")

oak_toolchain_repositories()

# Expected hashes for our base image tarballs
SYSROOT_SHA256 = "d6f608cf14b27bd4ae68f135b601b86bb9157a1a7a8fc08e43d7ff4ab7a18665"

BASE_IMAGE_SHA256 = "f539ecab633fa0a760ec49e917a0719f2d3ffc1eb6fe7853d518d17699fa035e"

NVIDIA_BASE_IMAGE_SHA256 = "10e665a269b79aef1e12a45a60abd1bf4638edae3bad0c41cec764ceacbfe0a9"

# Experimental sysroot for the build toolchain, based on Oak Containers sysimage.
#
# Rebuild it using:
# $ oak_containers/system_image/build-base.sh sysroot
#
# Upload it using:
# $ oak_containers/system_image/push-base.sh sysroot
#
# (See oak_containers/system_image/README.md for more details.)
http_archive(
    name = "oak_cc_toolchain_sysroot",
    build_file = "//:toolchain/sysroot.BUILD",
    sha256 = SYSROOT_SHA256,
    url = "https://storage.googleapis.com/oak-bins/sysroot/" + SYSROOT_SHA256 + ".tar.xz",
)

# The binary is used for stage0 tdx measurement test
# (//tdx_measurement:tdx_measurement_test) only.
STAGE0_BIN_TDX_COMMIT = "0689771e6fd6d174121eaa0b7df5fe54c4746ce3"

http_file(
    name = "stage0_tdx_bin_for_test",
    downloaded_file_path = "stage0_tdx_bin_for_test",
    sha256 = "87fe23ad59066718f97acfe2672f70e6ddfa488f7593d59b8886f67d0ca08715",
    url = "https://storage.googleapis.com/oak-bins/binary/" + STAGE0_BIN_TDX_COMMIT + "/stage0_bin_tdx/binary",
)

http_file(
    name = "oak_containers_system_image_base",
    downloaded_file_path = "base-image.tar.xz",
    sha256 = BASE_IMAGE_SHA256,
    url = "https://storage.googleapis.com/oak-bins/base-image/" + BASE_IMAGE_SHA256 + ".tar.xz",
)

http_file(
    name = "oak_containers_nvidia_system_image_base",
    downloaded_file_path = "nvidia-base-image.tar.xz",
    sha256 = NVIDIA_BASE_IMAGE_SHA256,
    url = "https://storage.googleapis.com/oak-bins/nvidia-base-image/" + NVIDIA_BASE_IMAGE_SHA256 + ".tar.xz",
)

# Register a hermetic C++ toolchain to ensure that binaries use a glibc version supported by
# distroless images. The glibc version provided by nix may be too new.
# This (currently) needs to be loaded after rules_oci because it defaults to using an older version
# of aspect-build/bazel-lib.
http_archive(
    name = "aspect_gcc_toolchain",
    patches = [
        "@//:third_party/aspect-gcc.patch",
    ],
    sha256 = "3341394b1376fb96a87ac3ca01c582f7f18e7dc5e16e8cf40880a31dd7ac0e1e",
    strip_prefix = "gcc-toolchain-0.4.2",
    urls = [
        "https://github.com/aspect-build/gcc-toolchain/archive/refs/tags/0.4.2.tar.gz",
    ],
)

load("@aspect_gcc_toolchain//toolchain:repositories.bzl", "gcc_toolchain_dependencies")

gcc_toolchain_dependencies()

load("@aspect_gcc_toolchain//toolchain:defs.bzl", "ARCHS", "gcc_register_toolchain")

gcc_register_toolchain(
    name = "gcc_toolchain_x86_64",
    # Prevents aspect_gcc from rendering -nostdinc flag. Needed to compile wasmtime.
    # See b/352306808#comment25.
    extra_cflags = [
        "-B%workspace%/bin",
    ],
    # Manually override ldflags and includes to paths we know exist in our sysroot.
    # These are based on https://github.com/f0rmiga/gcc-toolchain/blob/36e3e1f430871b539ce9261f53491564aa91c170/sysroot/flags.bzl,
    # just adjusted for our environment.
    extra_ldflags = [
        "-B%workspace%/bin",
        "-B%sysroot%/usr/lib/x86_64-linux-gnu",
        "-B%sysroot%/lib64/x86_64-linux-gnu",
        "-L%sysroot%/lib64/x86_64-linux-gnu",
        "-L%sysroot%/usr/lib/x86_64-linux-gnu",
        "-L%sysroot%/lib/gcc/x86_64-linux-gnu/12",
    ],
    includes = [
        # Order matters here! Don't let it get sorted.
        "%sysroot%/lib/gcc/x86_64-linux-gnu/12/include",
        "%sysroot%/usr/include/x86_64-linux-gnu",
        "%sysroot%/usr/include/c++/12",
        "%sysroot%/usr/include/x86_64-linux-gnu/c++/12/",
        "%sysroot%/usr/include",
    ],
    # sha256 of the compiler package
    sha256 = "5d515f6e4b311d7636a3cf600cd02fde7d0beb0a2f143df4921ff5a61cbaebcb",
    # what prefix to strip from the compiler package
    strip_prefix = "x86-64-v3--glibc--stable-2024.02-1",
    # Use the sysroot which is effectively our system image for consistency.
    sysroot = "@oak_cc_toolchain_sysroot//:sysroot",
    # target_compatible_with defaults to os:linux.
    target_arch = ARCHS.x86_64,
    # Which compiler to use: this is GCC 12, just as in Debian.
    # Note: v4 compiler requires AVX512. But recent AMD workstation processor
    # e.g. 5995WX does not support AVX512. We have to use v3 which is more
    # friendly to AMD users.
    url = "https://toolchains.bootlin.com/downloads/releases/toolchains/x86-64-v3/tarballs/x86-64-v3--glibc--stable-2024.02-1.tar.bz2",
)

gcc_register_toolchain(
    name = "gcc_toolchain_x86_64_unknown_none",  # Repository @gcc_toolchain_x86_64_unknown_none
    extra_ldflags = ["-nostdlib"],
    target_arch = ARCHS.x86_64,
    target_compatible_with = [
        "@platforms//cpu:x86_64",
        "@platforms//os:none",
    ],
)

load("//bazel/rust:deps.bzl", "load_rust_repositories")

load_rust_repositories()

load("//bazel/rust:defs.bzl", "setup_rust_dependencies")

setup_rust_dependencies()

load("//bazel/rust:stdlibs.bzl", "setup_rebuilt_rust_stdlibs")

setup_rebuilt_rust_stdlibs()

load("//bazel/crates:repositories.bzl", "create_oak_crate_repositories")

create_oak_crate_repositories()

load("//bazel/crates:crates.bzl", "load_oak_crate_repositories")

load_oak_crate_repositories()

http_archive(
    name = "e2fsprogs",
    build_file = "@//:third_party/BUILD.e2fsprogs",
    sha256 = "144af53f2bbd921cef6f8bea88bb9faddca865da3fbc657cc9b4d2001097d5db",
    strip_prefix = "e2fsprogs-1.47.0",
    urls = ["https://mirrors.edge.kernel.org/pub/linux/kernel/people/tytso/e2fsprogs/v1.47.0/e2fsprogs-1.47.0.tar.xz"],
)

load("//bazel/nix:kernel.bzl", "nix_kernel_repo")

# Get the nix-built Kernels into our Bazel workspace, and verify the sha256
nix_kernel_repo(
    name = "nix_kernels",
    bzImage_sha256 = "9e4dd5c5cebb4be3bea8747035979ab373b92124bbb9dcdef19325cea7116717",
    bzImage_vanilla_sha256 = "1ce6de1a2c4885dbf6d445a5bac390e285a76104d52e5356501330b409aaf141",
)

# Tink C++
# https://github.com/tink-crypto/tink-cc
http_archive(
    name = "com_github_tink_crypto_tink_cc",
    sha256 = "363ce671ab5ce0b24f279d3647185597a25f407c3608db007315f79f151f436b",
    strip_prefix = "tink-cc-2.3.0",
    urls = [
        # Tink v2.3.0 release
        "https://github.com/tink-crypto/tink-cc/archive/refs/tags/v2.3.0.zip",
    ],
)

load("@com_github_tink_crypto_tink_cc//:tink_cc_deps.bzl", "tink_cc_deps")

tink_cc_deps()

load("@com_github_tink_crypto_tink_cc//:tink_cc_deps_init.bzl", "tink_cc_deps_init")

tink_cc_deps_init()
