use crate::resource::NativeAuthZone;
use native_blueprints_interface::resource::*;
use radix_engine_common::data::scrypto::model::*;
use radix_engine_common::data::scrypto::{ScryptoCategorize, ScryptoDecode};
use radix_engine_common::math::Decimal;
use radix_engine_common::types::*;
use radix_engine_system_interface::{ClientApi, ACTOR_REF_AUTH_ZONE};
use sbor::rust::prelude::*;

pub struct LocalAuthZone {}

impl LocalAuthZone {
    pub fn drain<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        api: &mut Y,
    ) -> Result<Vec<Proof>, E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).drain(api)
    }

    pub fn drop_proofs<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        api: &mut Y,
    ) -> Result<(), E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).drop_proofs(api)
    }

    pub fn drop_regular_proofs<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        api: &mut Y,
    ) -> Result<(), E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).drop_regular_proofs(api)
    }

    pub fn drop_signature_proofs<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        api: &mut Y,
    ) -> Result<(), E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).drop_signature_proofs(api)
    }

    pub fn pop<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        api: &mut Y,
    ) -> Result<Option<Proof>, E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).pop(api)
    }

    pub fn create_proof_of_amount<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        amount: Decimal,
        resource_address: ResourceAddress,
        api: &mut Y,
    ) -> Result<Proof, E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).create_proof_of_amount(amount, resource_address, api)
    }

    pub fn create_proof_of_non_fungibles<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        ids: &IndexSet<NonFungibleLocalId>,
        resource_address: ResourceAddress,
        api: &mut Y,
    ) -> Result<Proof, E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).create_proof_of_non_fungibles(ids, resource_address, api)
    }

    pub fn create_proof_of_all<Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        resource_address: ResourceAddress,
        api: &mut Y,
    ) -> Result<Proof, E>
    where
        Y: ClientApi<E>,
    {
        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).create_proof_of_all(resource_address, api)
    }

    pub fn push<P: Into<Proof>, Y, E: Debug + ScryptoCategorize + ScryptoDecode>(
        proof: P,
        api: &mut Y,
    ) -> Result<(), E>
    where
        Y: ClientApi<E>,
    {
        let proof: Proof = proof.into();

        let auth_zone = api.actor_get_node_id(ACTOR_REF_AUTH_ZONE)?;
        AuthZoneRef(auth_zone).push(proof, api)
    }
}
