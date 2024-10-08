/*
 * Copyright 2023 The Project Oak Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef CC_CONTAINERS_SDK_ORCHESTRATOR_CLIENT_H_
#define CC_CONTAINERS_SDK_ORCHESTRATOR_CLIENT_H_

#include <memory>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "cc/containers/sdk/common.h"
#include "grpcpp/create_channel.h"
#include "grpcpp/security/credentials.h"
#include "proto/containers/interfaces.grpc.pb.h"
#include "proto/containers/interfaces.pb.h"

namespace oak::containers::sdk {

/// A convenience class for interacting with the Oak Orchestrator from within an
/// enclave application.
class OrchestratorClient {
 public:
  /// Create a new client, using the default channel, an insecure channel to the
  /// pre-defined UDS.
  OrchestratorClient()
      : OrchestratorClient(grpc::CreateChannel(
            kOrchestratorSocket, grpc::InsecureChannelCredentials())) {}

  /// Create a new client using the provided channel. This is useful for
  /// testing, or environments that provide orchestrator-like functionality via
  /// other means.
  explicit OrchestratorClient(const std::shared_ptr<grpc::Channel>& channel)
      : stub_(::oak::containers::Orchestrator::NewStub(channel)) {}

  absl::StatusOr<std::string> GetApplicationConfig() const;
  absl::Status NotifyAppReady() const;
  absl::StatusOr<oak::session::v1::EndorsedEvidence> GetEndorsedEvidence()
      const;

 private:
  std::unique_ptr<::oak::containers::Orchestrator::Stub> stub_;
};

}  // namespace oak::containers::sdk

#endif  // CC_CONTAINERS_SDK_ORCHESTRATOR_CLIENT_H_
