#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct GroupKeys {
    /// Encryption private key that was encrypted with HPKE using the encryption
    /// public key provided in the endorsed evidence.
    #[prost(message, optional, tag = "1")]
    pub encrypted_encryption_private_key: ::core::option::Option<
        super::super::crypto::v1::EncryptedRequest,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct GetGroupKeysRequest {
    /// Evidence contains the encryption public key for encrypting the group
    /// encryption key with Hybrid Encryption.
    /// <<https://datatracker.ietf.org/doc/rfc9180/>>
    #[prost(message, optional, tag = "1")]
    pub evidence: ::core::option::Option<super::super::attestation::v1::Evidence>,
    #[prost(message, optional, tag = "2")]
    pub endorsements: ::core::option::Option<
        super::super::attestation::v1::Endorsements,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct GetGroupKeysResponse {
    #[prost(message, optional, tag = "1")]
    pub group_keys: ::core::option::Option<GroupKeys>,
}
