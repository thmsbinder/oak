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

pub mod crypto;
pub mod handler;
pub mod oak_session_context;
pub mod orchestrator_client;
pub mod standalone;
pub mod tonic;

// Unix Domain Sockets do not use URIs, hence this URI will never be used.
// It is defined purely since in order to create a channel, since a URI has to
// be supplied to create an `Endpoint`. Even though in this case the endpoint
// is technically a file, tonic expects us to provide our own connector, and
// this ignored endpoint. :(
static IGNORED_ENDPOINT_URI: &str = "file://[::]:0";

// Path used to facilitate inter-process communication between the orchestrator
// and the trusted application.
const ORCHESTRATOR_IPC_SOCKET: &str = "/oak_utils/orchestrator_ipc";

// Re-export structs so that they are available at the top level of the SDK.
pub use crypto::InstanceEncryptionKeyHandle;
pub use oak_session_context::{ApplicationHandler, OakSessionContext};
pub use orchestrator_client::OrchestratorClient;
