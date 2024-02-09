use crate::resource::Bucket;
use crate::resource::Proof;
use radix_engine_common::data::scrypto::model::NonFungibleLocalId;
use radix_engine_common::prelude::*;
use sbor::rust::collections::IndexSet;

pub const NON_FUNGIBLE_VAULT_BLUEPRINT: &str = "NonFungibleVault";

pub const NON_FUNGIBLE_VAULT_TAKE_NON_FUNGIBLES_IDENT: &str = "take_non_fungibles";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultTakeNonFungiblesInput {
    pub non_fungible_local_ids: IndexSet<NonFungibleLocalId>,
}

pub type NonFungibleVaultTakeNonFungiblesOutput = Bucket;

pub const NON_FUNGIBLE_VAULT_GET_NON_FUNGIBLE_LOCAL_IDS_IDENT: &str = "get_non_fungible_local_ids";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultGetNonFungibleLocalIdsInput {
    pub limit: u32,
}

pub type NonFungibleVaultGetNonFungibleLocalIdsOutput = IndexSet<NonFungibleLocalId>;

pub const NON_FUNGIBLE_VAULT_CONTAINS_NON_FUNGIBLE_IDENT: &str = "contains_non_fungible";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultContainsNonFungibleInput {
    pub id: NonFungibleLocalId,
}

pub type NonFungibleVaultContainsNonFungibleOutput = bool;

pub const NON_FUNGIBLE_VAULT_RECALL_NON_FUNGIBLES_IDENT: &str = "recall_non_fungibles";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor, ManifestSbor)]
pub struct NonFungibleVaultRecallNonFungiblesInput {
    pub non_fungible_local_ids: IndexSet<NonFungibleLocalId>,
}

pub type NonFungibleVaultRecallNonFungiblesOutput = Bucket;

pub const NON_FUNGIBLE_VAULT_CREATE_PROOF_OF_NON_FUNGIBLES_IDENT: &str =
    "create_proof_of_non_fungibles";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultCreateProofOfNonFungiblesInput {
    pub ids: IndexSet<NonFungibleLocalId>,
}

pub type NonFungibleVaultCreateProofOfNonFungiblesOutput = Proof;

pub const NON_FUNGIBLE_VAULT_LOCK_NON_FUNGIBLES_IDENT: &str = "lock_non_fungibles";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultLockNonFungiblesInput {
    pub local_ids: IndexSet<NonFungibleLocalId>,
}

pub type NonFungibleVaultLockNonFungiblesOutput = ();

pub const NON_FUNGIBLE_VAULT_UNLOCK_NON_FUNGIBLES_IDENT: &str = "unlock_non_fungibles";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultUnlockNonFungiblesInput {
    pub local_ids: IndexSet<NonFungibleLocalId>,
}

pub type NonFungibleVaultUnlockNonFungiblesOutput = ();

pub const NON_FUNGIBLE_VAULT_BURN_NON_FUNGIBLES_IDENT: &str = "burn_non_fungibles";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoSbor)]
pub struct NonFungibleVaultBurnNonFungiblesInput {
    pub non_fungible_local_ids: IndexSet<NonFungibleLocalId>,
}

pub type NonFungibleVaultBurnNonFungiblesOutput = ();
