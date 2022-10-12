use sbor::rust::collections::BTreeSet;
use sbor::rust::vec::Vec;
use sbor::*;
use scrypto::core::{FnIdent, MethodIdent, NativeMethod, ReceiverMethodIdent};

use crate::core::{AuthZoneMethod, Receiver};
use crate::engine::types::RENodeId;
use crate::engine::{api::*, call_engine};
use crate::math::Decimal;
use crate::native_functions;
use crate::resource::*;

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZonePopInput {}

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZonePushInput {
    pub proof: Proof,
}

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZoneCreateProofInput {
    pub resource_address: ResourceAddress,
}

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZoneCreateProofByAmountInput {
    pub amount: Decimal,
    pub resource_address: ResourceAddress,
}

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZoneCreateProofByIdsInput {
    pub ids: BTreeSet<NonFungibleId>,
    pub resource_address: ResourceAddress,
}

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZoneClearInput {}

#[derive(Debug, TypeId, Encode, Decode)]
pub struct AuthZoneDrainInput {}

/// Represents the auth zone, which is used by system for checking
/// if this component is allowed to
///
/// 1. Call methods on another component;
/// 2. Access resource system.
pub struct ComponentAuthZone {}

impl ComponentAuthZone {
    native_functions! {
        {
            let input = RadixEngineInput::GetOwnedRENodeIds();
            let owned_node_ids: Vec<RENodeId> = call_engine(input);
            let node_id = owned_node_ids.into_iter().find(|n| matches!(n, RENodeId::AuthZone(..))).expect("AuthZone does not exist");
            Receiver::Ref(node_id)
        }, NativeMethod::AuthZone => {
            pub fn pop() -> Proof {
                AuthZoneMethod::Pop,
                AuthZonePopInput {}
            }

            pub fn create_proof(resource_address: ResourceAddress) -> Proof {
                AuthZoneMethod::CreateProof,
                AuthZoneCreateProofInput {
                    resource_address
                }
            }

            pub fn create_proof_by_amount(amount: Decimal, resource_address: ResourceAddress) -> Proof {
                AuthZoneMethod::CreateProofByAmount,
                AuthZoneCreateProofByAmountInput {
                    amount, resource_address
                }
            }

            pub fn create_proof_by_ids(ids: &BTreeSet<NonFungibleId>, resource_address: ResourceAddress) -> Proof {
                AuthZoneMethod::CreateProofByIds,
                AuthZoneCreateProofByIdsInput {
                    ids: ids.clone(),
                    resource_address
                }
            }
        }
    }

    pub fn push<P: Into<Proof>>(proof: P) {
        let input = RadixEngineInput::GetOwnedRENodeIds();
        let owned_node_ids: Vec<RENodeId> = call_engine(input);
        let node_id = owned_node_ids
            .into_iter()
            .find(|n| matches!(n, RENodeId::AuthZone(..)))
            .expect("AuthZone does not exist");

        let proof: Proof = proof.into();
        let input = RadixEngineInput::Invoke(
            FnIdent::Method(ReceiverMethodIdent {
                receiver: Receiver::Ref(node_id),
                method_ident: MethodIdent::Native(NativeMethod::AuthZone(AuthZoneMethod::Push)),
            }),
            scrypto::buffer::scrypto_encode(&(AuthZonePushInput { proof })),
        );
        call_engine(input)
    }
}
