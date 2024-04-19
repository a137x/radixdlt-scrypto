use radix_common::data::scrypto::model::*;
use radix_rust::rust::collections::IndexSet;
use radix_rust::rust::fmt::Debug;
use sbor::*;

pub const NON_FUNGIBLE_PROOF_BLUEPRINT: &str = "NonFungibleProof";

pub const NON_FUNGIBLE_PROOF_GET_LOCAL_IDS_IDENT: &str = "NonFungibleProof_get_local_ids";

#[derive(Debug, Clone, Eq, PartialEq, Sbor)]
pub struct NonFungibleProofGetLocalIdsInput {}

pub type NonFungibleProofGetLocalIdsOutput = IndexSet<NonFungibleLocalId>;
