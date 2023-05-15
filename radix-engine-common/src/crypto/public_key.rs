use crate::crypto::*;
use crate::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;
use sbor::*;

/// Represents any natively supported public key.
#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "public_key")
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sbor)]
pub enum PublicKey {
    EcdsaSecp256k1(EcdsaSecp256k1PublicKey),
    EddsaEd25519(EddsaEd25519PublicKey),
}

impl From<EcdsaSecp256k1PublicKey> for PublicKey {
    fn from(public_key: EcdsaSecp256k1PublicKey) -> Self {
        Self::EcdsaSecp256k1(public_key)
    }
}

impl From<EddsaEd25519PublicKey> for PublicKey {
    fn from(public_key: EddsaEd25519PublicKey) -> Self {
        Self::EddsaEd25519(public_key)
    }
}

impl HasPublicKeyHash for PublicKey {
    type TypedPublicKeyHash = PublicKeyHash;

    fn get_hash(&self) -> Self::TypedPublicKeyHash {
        PublicKeyHash::new_from_public_key(self)
    }
}
