/// Endorsed evidence contains an attestation evidence provided by the enclave
/// and the corresponding attestation endorsements provided by the hostlib.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EndorsedEvidence {
    #[prost(message, optional, tag = "1")]
    pub evidence: ::core::option::Option<super::super::attestation::v1::Evidence>,
    #[prost(message, optional, tag = "2")]
    pub endorsements: ::core::option::Option<
        super::super::attestation::v1::Endorsements,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct GetEndorsedEvidenceRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct GetEndorsedEvidenceResponse {
    #[prost(message, optional, tag = "1")]
    pub endorsed_evidence: ::core::option::Option<EndorsedEvidence>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct InvokeRequest {
    /// Body of the request, encrypted using Hybrid Public Key Encryption (HPKE).
    /// <<https://www.rfc-editor.org/rfc/rfc9180.html>>
    #[prost(message, optional, tag = "2")]
    pub encrypted_request: ::core::option::Option<
        super::super::crypto::v1::EncryptedRequest,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct InvokeResponse {
    /// Body of the request, encrypted using Hybrid Public Key Encryption (HPKE).
    /// <<https://www.rfc-editor.org/rfc/rfc9180.html>>
    #[prost(message, optional, tag = "2")]
    pub encrypted_response: ::core::option::Option<
        super::super::crypto::v1::EncryptedResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RequestWrapper {
    #[prost(oneof = "request_wrapper::Request", tags = "2, 3")]
    pub request: ::core::option::Option<request_wrapper::Request>,
}
/// Nested message and enum types in `RequestWrapper`.
pub mod request_wrapper {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        InvokeRequest(super::InvokeRequest),
        #[prost(message, tag = "3")]
        GetEndorsedEvidenceRequest(super::GetEndorsedEvidenceRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ResponseWrapper {
    #[prost(oneof = "response_wrapper::Response", tags = "2, 3")]
    pub response: ::core::option::Option<response_wrapper::Response>,
}
/// Nested message and enum types in `ResponseWrapper`.
pub mod response_wrapper {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Response {
        #[prost(message, tag = "2")]
        InvokeResponse(super::InvokeResponse),
        #[prost(message, tag = "3")]
        GetEndorsedEvidenceResponse(super::GetEndorsedEvidenceResponse),
    }
}
/// Request message for the remote attestation.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AttestRequest {
    /// Maps unique IDs of attestation providers to the corresponding evidence.
    #[prost(btree_map = "string, message", tag = "1")]
    pub endorsed_evidence: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        EndorsedEvidence,
    >,
}
/// Response message for the remote attestation.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AttestResponse {
    /// Maps unique IDs of attestation providers to the corresponding evidence.
    #[prost(btree_map = "string, message", tag = "1")]
    pub endorsed_evidence: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        EndorsedEvidence,
    >,
}
/// Noise handshake message containing fields for all handshake patterns.
/// <<http://www.noiseprotocol.org/noise.html#handshake-patterns>>
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct NoiseHandshakeMessage {
    /// Noise Protocol ephemeral public key 'e'.
    /// <<http://www.noiseprotocol.org/noise.html#overview-of-handshake-state-machine>>
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "crate::base64data")]
    pub ephemeral_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Noise Protocol static public key 's'.
    /// <<http://www.noiseprotocol.org/noise.html#overview-of-handshake-state-machine>>
    ///
    /// Note: For some Noise patterns (such as XX and IX) static public key may be
    /// encrypted with the chaining key to hide peer's identity.
    /// <<http://www.noiseprotocol.org/noise.html#handshake-patterns>>
    #[prost(bytes = "vec", tag = "2")]
    #[serde(with = "crate::base64data")]
    pub static_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Payload encrypted with the current chaining key.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(with = "crate::base64data")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
/// Message that binds the Noise session (and optionally other data) to the
/// Attestation Evidence.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SessionBinding {
    /// Representation the serialized message cryptographically bound to the
    /// handshake and the associated data (e.g., a signature).
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "crate::base64data")]
    pub binding: ::prost::alloc::vec::Vec<u8>,
}
/// Request message for the crypto handshake request needed to establish a set of
/// session keys.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct HandshakeRequest {
    /// Bindings to the attestation evidence, per binding type.
    #[prost(btree_map = "string, message", tag = "3")]
    pub attestation_bindings: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        SessionBinding,
    >,
    #[prost(oneof = "handshake_request::HandshakeType", tags = "1")]
    #[serde(flatten)]
    pub handshake_type: ::core::option::Option<handshake_request::HandshakeType>,
}
/// Nested message and enum types in `HandshakeRequest`.
pub mod handshake_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum HandshakeType {
        #[prost(message, tag = "1")]
        NoiseHandshakeMessage(super::NoiseHandshakeMessage),
    }
}
/// Response message for the crypto handshake request needed to establish a set
/// of session keys.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct HandshakeResponse {
    /// Bindings to the attestation evidence, per binding type.
    #[prost(btree_map = "string, message", tag = "3")]
    pub attestation_bindings: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        SessionBinding,
    >,
    #[prost(oneof = "handshake_response::HandshakeType", tags = "1")]
    #[serde(flatten)]
    pub handshake_type: ::core::option::Option<handshake_response::HandshakeType>,
}
/// Nested message and enum types in `HandshakeResponse`.
pub mod handshake_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum HandshakeType {
        #[prost(message, tag = "1")]
        NoiseHandshakeMessage(super::NoiseHandshakeMessage),
    }
}
/// Message for encrypted data exchange after a secure session is established.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EncryptedMessage {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "crate::base64data")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", optional, tag = "2")]
    #[serde(with = "crate::base64data::option_bytes")]
    pub associated_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "3")]
    #[serde(with = "crate::base64data::option_bytes")]
    pub nonce: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Message with decrypted content (not to be transmitted over the wire).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct PlaintextMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub plaintext: ::prost::alloc::vec::Vec<u8>,
}
/// Request message for the Oak protocol attested secure session.
/// This message is a wrapper containing different message types including:
/// attestation, handshake and encrypted data exchange.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SessionRequest {
    #[prost(oneof = "session_request::Request", tags = "1, 2, 3")]
    #[serde(flatten)]
    pub request: ::core::option::Option<session_request::Request>,
}
/// Nested message and enum types in `SessionRequest`.
pub mod session_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1")]
        AttestRequest(super::AttestRequest),
        #[prost(message, tag = "2")]
        HandshakeRequest(super::HandshakeRequest),
        #[prost(message, tag = "3")]
        EncryptedMessage(super::EncryptedMessage),
    }
}
/// Wrapper around SessionRequest that is used in cases where it is necessary to
/// identify a contiguous session across multiple invocations/streams.
///
/// This message is intended for cases where it's not possible to maintain a gRPC
/// stream for the lifetime of the encryption channel. The `session_id` is an
/// arbitrary identifier that can be used to look up a previously created
/// `ServerSession` instances cached by the server.
///
///   This shouldn't be used for general session ID needs; for example, to track
///   sessions that span multiple servers. In that case, create your own
///   ID-containing envelope, place that inside the encrypted message.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SessionRequestWithSessionId {
    /// Unique string to identify the session. This should be at least 128 bits of
    /// unique information.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<SessionRequest>,
}
/// Response message for the Oak protocol attested secure session.
/// This message is a wrapper containing different message types including:
/// attestation, handshake and encrypted data exchange.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SessionResponse {
    #[prost(oneof = "session_response::Response", tags = "1, 2, 3")]
    #[serde(flatten)]
    pub response: ::core::option::Option<session_response::Response>,
}
/// Nested message and enum types in `SessionResponse`.
pub mod session_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        AttestResponse(super::AttestResponse),
        #[prost(message, tag = "2")]
        HandshakeResponse(super::HandshakeResponse),
        #[prost(message, tag = "3")]
        EncryptedMessage(super::EncryptedMessage),
    }
}
