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

use oak_proto_rust::oak::{attestation::v1::EventAttestationResults, Variant};

/// Verification Policy that takes a generic evidence and endorsement and
/// performs attestation verification.
///
/// Verification Policy correspond to the "Appraisal Policy for Evidence"
/// provided by the RATS standard.
/// <https://datatracker.ietf.org/doc/html/rfc9334#section-8.5>
pub trait Policy<V: ?Sized>: Send + Sync {
    fn verify(
        &self,
        evidence: &V,
        endorsement: &Variant,
        milliseconds_since_epoch: i64,
    ) -> anyhow::Result<EventAttestationResults>;
}

/// Verification Policy that takes an encoded Event and an encoded Event
/// Endorsement and performs attestation verification for this specific Event.
pub trait EventPolicy = Policy<[u8]>;
