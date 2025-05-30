#
# Copyright 2023 The Project Oak Authors
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

load("@rules_java//java:defs.bzl", "java_library")
load("@rules_kotlin//kotlin:jvm.bzl", "kt_jvm_library")

package(
    default_visibility = ["//:default_visibility"],
    licenses = ["notice"],
)

kt_jvm_library(
    name = "decryption_result",
    srcs = [
        "DecryptionResult.kt",
    ],
)

java_library(
    name = "client_encryptor",
    srcs = [
        "ClientEncryptor.java",
    ],
    deps = [
        ":decryption_result",
        "//java/src/main/java/com/google/oak/crypto/hpke",
        "//java/src/main/java/com/google/oak/util",
        "//proto/crypto:crypto_java_proto",
        "@com_google_protobuf//java/core",
    ],
)

# Clients are responsible for bundling in "libhpke-jni.so".
java_library(
    name = "client_encryptor_android",
    srcs = [
        "ClientEncryptor.java",
    ],
    deps = [
        ":decryption_result",
        "//java/src/main/java/com/google/oak/crypto/hpke:hpke_android",
        "//java/src/main/java/com/google/oak/util",
        "//proto/crypto:crypto_java_proto",
        "@com_google_protobuf//java/lite",
    ],
)

java_library(
    name = "server_encryptor",
    srcs = [
        "ServerEncryptor.java",
    ],
    deps = [
        ":decryption_result",
        "//java/src/main/java/com/google/oak/crypto/hpke",
        "//java/src/main/java/com/google/oak/util",
        "//proto/crypto:crypto_java_proto",
        "@com_google_protobuf//java/core",
    ],
)

# Clients are responsible for bundling in "libhpke-jni.so".
java_library(
    name = "server_encryptor_android",
    srcs = [
        "ServerEncryptor.java",
    ],
    deps = [
        ":decryption_result",
        "//java/src/main/java/com/google/oak/crypto/hpke:hpke_android",
        "//java/src/main/java/com/google/oak/util",
        "//proto/crypto:crypto_java_proto",
        "@com_google_protobuf//java/lite",
    ],
)
