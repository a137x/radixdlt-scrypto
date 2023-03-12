use crate::blueprints::resource::vault::VaultInfoSubstate;
use crate::blueprints::resource::*;
use crate::errors::RuntimeError;
use crate::errors::{ApplicationError, InterpreterError};
use crate::kernel::heap::DroppedBucket;
use crate::kernel::heap::DroppedBucketResource;
use crate::kernel::kernel_api::{KernelNodeApi, KernelSubstateApi};
use crate::types::*;
use native_sdk::access_rules::AccessRulesObject;
use native_sdk::metadata::Metadata;
use native_sdk::resource::SysBucket;
use native_sdk::runtime::Runtime;
use radix_engine_interface::api::node_modules::metadata::{METADATA_GET_IDENT, METADATA_SET_IDENT};
use radix_engine_interface::api::substate_api::LockFlags;
use radix_engine_interface::api::types::{RENodeId, ResourceManagerOffset, SubstateOffset};
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::resource::AccessRule::{AllowAll, DenyAll};
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::math::Decimal;
use radix_engine_interface::schema::{KeyValueStoreSchema, NonFungibleSchema};
use radix_engine_interface::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct FungibleResourceManagerSubstate {
    pub resource_address: ResourceAddress, // TODO: Figure out a way to remove?
    pub divisibility: u8,
    pub total_supply: Decimal,
}

impl FungibleResourceManagerSubstate {
    pub fn new(
        divisibility: u8,
        resource_address: ResourceAddress,
    ) -> FungibleResourceManagerSubstate {
        Self {
            divisibility,
            total_supply: 0.into(),
            resource_address,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct NonFungibleResourceManagerSubstate {
    pub resource_address: ResourceAddress, // TODO: Figure out a way to remove?
    pub total_supply: Decimal,
    pub id_type: NonFungibleIdType,
    pub non_fungible_table: KeyValueStoreId,
    pub mutable_fields: BTreeSet<String>,
}

/// Represents an error when accessing a bucket.
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum ResourceManagerError {
    InvalidAmount(Decimal, u8),
    MaxMintAmountExceeded,
    NonFungibleAlreadyExists(NonFungibleGlobalId),
    NonFungibleNotFound(NonFungibleGlobalId),
    MismatchingBucketResource,
    NonFungibleIdTypeDoesNotMatch(NonFungibleIdType, NonFungibleIdType),
    InvalidNonFungibleIdType,
}

fn build_non_fungible_resource_manager_substate<Y>(
    resource_address: ResourceAddress,
    id_type: NonFungibleIdType,
    supply: usize,
    mutable_fields: BTreeSet<String>,
    non_fungible_schema: NonFungibleSchema,
    api: &mut Y,
) -> Result<(NonFungibleResourceManagerSubstate, KeyValueStoreId), RuntimeError>
where
    Y: ClientApi<RuntimeError>,
{
    let mut aggregator = TypeAggregator::<ScryptoCustomTypeKind>::new();
    let non_fungible_type = aggregator.add_child_type_and_descendents::<NonFungibleLocalId>();
    let key_schema = generate_full_schema(aggregator);
    let mut kv_schema = non_fungible_schema.schema;
    kv_schema.type_kinds.extend(key_schema.type_kinds);
    {
        let mut variants = BTreeMap::new();
        variants.insert(OPTION_VARIANT_NONE, vec![]);
        variants.insert(OPTION_VARIANT_SOME, vec![non_fungible_schema.non_fungible]);
        let type_kind = TypeKind::Enum { variants };
        kv_schema.type_kinds.push(type_kind);
    }
    kv_schema.type_metadata.extend(key_schema.type_metadata);
    {
        let metadata = TypeMetadata {
            type_name: Cow::Borrowed("Option"),
            child_names: Some(ChildNames::EnumVariants(btreemap!(
                OPTION_VARIANT_NONE => TypeMetadata::no_child_names("None"),
                OPTION_VARIANT_SOME => TypeMetadata::no_child_names("Some"),
            ))),
        };
        kv_schema.type_metadata.push(metadata);
    }
    kv_schema
        .type_validations
        .extend(key_schema.type_validations);
    kv_schema.type_validations.push(TypeValidation::None);
    let value_index = LocalTypeIndex::SchemaLocalIndex(kv_schema.type_validations.len() - 1);

    let kv_schema = KeyValueStoreSchema {
        schema: kv_schema,
        key: non_fungible_type,
        value: value_index,
    };

    let nf_store_id = api.new_key_value_store(kv_schema)?;

    let resource_manager = NonFungibleResourceManagerSubstate {
        resource_address,
        id_type,
        total_supply: supply.into(),
        non_fungible_table: nf_store_id,
        mutable_fields,
    };

    Ok((resource_manager, nf_store_id))
}

fn globalize_non_fungible_resource_manager<Y>(
    resource_address: ResourceAddress,
    substate: NonFungibleResourceManagerSubstate,
    access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    metadata: BTreeMap<String, String>,
    api: &mut Y,
) -> Result<(), RuntimeError>
where
    Y: ClientApi<RuntimeError>,
{
    let object_id = api.new_object(
        NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
        vec![scrypto_encode(&substate).unwrap()],
    )?;

    let (resman_access_rules, vault_access_rules) = build_access_rules(access_rules);
    let resman_access_rules = AccessRulesObject::sys_new(resman_access_rules, api)?;
    let vault_access_rules = AccessRulesObject::sys_new(vault_access_rules, api)?;
    let metadata = Metadata::sys_create_with_data(metadata, api)?;

    api.globalize_with_address(
        RENodeId::Object(object_id),
        btreemap!(
            NodeModuleId::AccessRules => resman_access_rules.id(),
            NodeModuleId::AccessRules1 => vault_access_rules.id(),
            NodeModuleId::Metadata => metadata.id(),
        ),
        resource_address.into(),
    )?;

    Ok(())
}

fn build_non_fungible_bucket<Y>(
    resource_address: ResourceAddress,
    id_type: NonFungibleIdType,
    nf_store_id: KeyValueStoreId,
    entries: BTreeMap<NonFungibleLocalId, Vec<u8>>,
    api: &mut Y,
) -> Result<Bucket, RuntimeError>
where
    Y: ClientApi<RuntimeError>,
{
    let bucket = {
        for (non_fungible_local_id, data) in &entries {
            if non_fungible_local_id.id_type() != id_type {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(
                        ResourceManagerError::NonFungibleIdTypeDoesNotMatch(
                            non_fungible_local_id.id_type(),
                            id_type,
                        ),
                    ),
                ));
            }

            let non_fungible_handle = api.sys_lock_substate(
                RENodeId::KeyValueStore(nf_store_id),
                SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                    scrypto_encode(non_fungible_local_id).unwrap(),
                )),
                LockFlags::MUTABLE,
            )?;

            // TODO: Change interface so that we accept Option instead
            let value: ScryptoValue = scrypto_decode(data).unwrap();
            api.sys_write_substate(non_fungible_handle, scrypto_encode(&Some(value)).unwrap())?;

            api.sys_drop_lock(non_fungible_handle)?;
        }
        let ids = entries.into_keys().collect();

        let info = BucketInfoSubstate {
            resource_address,
            resource_type: ResourceType::NonFungible { id_type },
        };
        let bucket_id = api.new_object(
            BUCKET_BLUEPRINT,
            vec![
                scrypto_encode(&info).unwrap(),
                scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                scrypto_encode(&LiquidNonFungibleResource::new(ids)).unwrap(),
                scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
            ],
        )?;

        Bucket(bucket_id)
    };

    Ok(bucket)
}

fn build_fungible_resource_manager_substate_with_initial_supply<Y>(
    resource_address: ResourceAddress,
    divisibility: u8,
    initial_supply: Decimal,
    api: &mut Y,
) -> Result<(FungibleResourceManagerSubstate, Bucket), RuntimeError>
where
    Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
{
    let mut resource_manager = FungibleResourceManagerSubstate::new(divisibility, resource_address);

    let bucket = {
        // check amount
        let resource_type = ResourceType::Fungible { divisibility };
        if !resource_type.check_amount(initial_supply) {
            return Err(RuntimeError::ApplicationError(
                ApplicationError::ResourceManagerError(ResourceManagerError::InvalidAmount(
                    initial_supply,
                    divisibility,
                )),
            ));
        }

        // TODO: refactor this into mint function
        if initial_supply > dec!("1000000000000000000") {
            return Err(RuntimeError::ApplicationError(
                ApplicationError::ResourceManagerError(ResourceManagerError::MaxMintAmountExceeded),
            ));
        }
        resource_manager.total_supply = initial_supply;

        let bucket_info = BucketInfoSubstate {
            resource_address,
            resource_type: ResourceType::Fungible { divisibility },
        };
        let liquid_resource = LiquidFungibleResource::new(initial_supply);
        let bucket_id = api.new_object(
            BUCKET_BLUEPRINT,
            vec![
                scrypto_encode(&bucket_info).unwrap(),
                scrypto_encode(&liquid_resource).unwrap(),
                scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                scrypto_encode(&LiquidNonFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
            ],
        )?;

        Bucket(bucket_id)
    };

    Ok((resource_manager, bucket))
}

fn build_access_rules(
    mut access_rules_map: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
) -> (AccessRulesConfig, AccessRulesConfig) {
    let (mint_access_rule, mint_mutability) = access_rules_map
        .remove(&Mint)
        .unwrap_or((DenyAll, rule!(deny_all)));
    let (burn_access_rule, burn_mutability) = access_rules_map
        .remove(&Burn)
        .unwrap_or((DenyAll, rule!(deny_all)));
    let (update_non_fungible_data_access_rule, update_non_fungible_data_mutability) =
        access_rules_map
            .remove(&UpdateNonFungibleData)
            .unwrap_or((AllowAll, rule!(deny_all)));
    let (update_metadata_access_rule, update_metadata_mutability) = access_rules_map
        .remove(&UpdateMetadata)
        .unwrap_or((DenyAll, rule!(deny_all)));

    let mut resman_access_rules = AccessRulesConfig::new();
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::Metadata, METADATA_SET_IDENT.to_string()),
        update_metadata_access_rule,
        update_metadata_mutability,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::Metadata, METADATA_GET_IDENT.to_string()),
        AllowAll,
        DenyAll,
    );
    resman_access_rules.set_group_access_rule_and_mutability(
        "mint".to_string(),
        mint_access_rule,
        mint_mutability,
    );
    resman_access_rules.set_group_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            NON_FUNGIBLE_MINT_RESOURCE_MANAGER_MINT_IDENT.to_string(),
        ),
        "mint".to_string(),
        DenyAll,
    );
    resman_access_rules.set_group_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            NON_FUNGIBLE_RESOURCE_MANAGER_MINT_UUID_IDENT.to_string(),
        ),
        "mint".to_string(),
        DenyAll,
    );
    resman_access_rules.set_group_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            RESOURCE_MANAGER_MINT_FUNGIBLE_IDENT.to_string(),
        ),
        "mint".to_string(),
        DenyAll,
    );

    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::SELF, RESOURCE_MANAGER_BURN_IDENT.to_string()),
        burn_access_rule,
        burn_mutability,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            NON_FUNGIBLE_RESOURCE_MANAGER_UPDATE_DATA_IDENT.to_string(),
        ),
        update_non_fungible_data_access_rule,
        update_non_fungible_data_mutability,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            RESOURCE_MANAGER_CREATE_BUCKET_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            RESOURCE_MANAGER_GET_RESOURCE_TYPE_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            RESOURCE_MANAGER_GET_TOTAL_SUPPLY_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            RESOURCE_MANAGER_CREATE_VAULT_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            NON_FUNGIBLE_RESOURCE_MANAGER_EXISTS_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    resman_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            NON_FUNGIBLE_RESOURCE_MANAGER_GET_NON_FUNGIBLE_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );

    let (deposit_access_rule, deposit_mutability) = access_rules_map
        .remove(&ResourceMethodAuthKey::Deposit)
        .unwrap_or((AllowAll, rule!(deny_all)));
    let (withdraw_access_rule, withdraw_mutability) = access_rules_map
        .remove(&ResourceMethodAuthKey::Withdraw)
        .unwrap_or((AllowAll, rule!(deny_all)));
    let (recall_access_rule, recall_mutability) = access_rules_map
        .remove(&ResourceMethodAuthKey::Recall)
        .unwrap_or((DenyAll, rule!(deny_all)));

    let mut vault_access_rules = AccessRulesConfig::new();
    vault_access_rules.set_group_access_rule_and_mutability(
        "withdraw".to_string(),
        withdraw_access_rule,
        withdraw_mutability,
    );
    vault_access_rules.set_group_access_rule_and_mutability(
        "recall".to_string(),
        recall_access_rule,
        recall_mutability,
    );
    vault_access_rules.set_group_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_TAKE_IDENT.to_string()),
        "withdraw".to_string(),
        DenyAll,
    );
    vault_access_rules.set_group_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_TAKE_NON_FUNGIBLES_IDENT.to_string(),
        ),
        "withdraw".to_string(),
        DenyAll,
    );
    vault_access_rules.set_group_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_LOCK_FEE_IDENT.to_string()),
        "withdraw".to_string(),
        DenyAll,
    );

    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_PUT_IDENT.to_string()),
        deposit_access_rule,
        deposit_mutability,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_GET_AMOUNT_IDENT.to_string()),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_GET_RESOURCE_ADDRESS_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_GET_NON_FUNGIBLE_LOCAL_IDS_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_CREATE_PROOF_IDENT.to_string()),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_CREATE_PROOF_BY_AMOUNT_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_CREATE_PROOF_BY_IDS_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_LOCK_AMOUNT_IDENT.to_string()),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_LOCK_NON_FUNGIBLES_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(NodeModuleId::SELF, VAULT_UNLOCK_AMOUNT_IDENT.to_string()),
        AllowAll,
        DenyAll,
    );
    vault_access_rules.set_access_rule_and_mutability(
        MethodKey::new(
            NodeModuleId::SELF,
            VAULT_UNLOCK_NON_FUNGIBLES_IDENT.to_string(),
        ),
        AllowAll,
        DenyAll,
    );

    (resman_access_rules, vault_access_rules)
}


pub struct NonFungibleResourceManagerBlueprint;

impl NonFungibleResourceManagerBlueprint {
    pub(crate) fn create<Y>(
        id_type: NonFungibleIdType,
        non_fungible_schema: NonFungibleSchema,
        metadata: BTreeMap<String, String>,
        access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
        api: &mut Y,
    ) -> Result<ResourceAddress, RuntimeError>
        where
            Y: KernelNodeApi + ClientApi<RuntimeError>,
    {
        let global_node_id = api.kernel_allocate_node_id(RENodeType::GlobalResourceManager)?;
        let resource_address: ResourceAddress = global_node_id.into();
        Self::create_with_address(
            id_type,
            non_fungible_schema,
            metadata,
            access_rules,
            resource_address.to_array_without_entity_id(),
            api,
        )
    }

    pub(crate) fn create_with_address<Y>(
        id_type: NonFungibleIdType,
        non_fungible_schema: NonFungibleSchema,
        metadata: BTreeMap<String, String>,
        access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
        resource_address: [u8; 26], // TODO: Clean this up
        api: &mut Y,
    ) -> Result<ResourceAddress, RuntimeError>
        where
            Y: ClientApi<RuntimeError>,
    {
        let resource_address = ResourceAddress::Normal(resource_address);

        // If address isn't user frame allocated or pre_allocated then
        // using this node_id will fail on create_node below
        let (resource_manager_substate, _) = build_non_fungible_resource_manager_substate(
            resource_address,
            id_type,
            0,
            BTreeSet::new(),
            non_fungible_schema,
            api,
        )?;

        globalize_non_fungible_resource_manager(
            resource_address,
            resource_manager_substate,
            access_rules,
            metadata,
            api,
        )?;

        Ok(resource_address)
    }

    pub(crate) fn create_with_initial_supply<Y>(
        id_type: NonFungibleIdType,
        non_fungible_schema: NonFungibleSchema,
        metadata: BTreeMap<String, String>,
        access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
        entries: BTreeMap<NonFungibleLocalId, Vec<u8>>,
        api: &mut Y,
    ) -> Result<(ResourceAddress, Bucket), RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let global_node_id = api.kernel_allocate_node_id(RENodeType::GlobalResourceManager)?;
        let resource_address: ResourceAddress = global_node_id.into();

        // TODO: Do this check in a better way (e.g. via type check)
        if id_type == NonFungibleIdType::UUID {
            return Err(RuntimeError::ApplicationError(
                ApplicationError::ResourceManagerError(
                    ResourceManagerError::InvalidNonFungibleIdType,
                ),
            ));
        }

        let (resource_manager, nf_store_id) = build_non_fungible_resource_manager_substate(
            resource_address,
            id_type,
            entries.len(),
            BTreeSet::new(),
            non_fungible_schema,
            api,
        )?;

        let bucket =
            build_non_fungible_bucket(resource_address, id_type, nf_store_id, entries, api)?;

        globalize_non_fungible_resource_manager(
            resource_address,
            resource_manager,
            access_rules,
            metadata,
            api,
        )?;

        Ok((resource_address, bucket))
    }

    pub(crate) fn create_uuid_with_initial_supply<Y>(
        non_fungible_schema: NonFungibleSchema,
        metadata: BTreeMap<String, String>,
        access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
        entries: Vec<Vec<u8>>,
        api: &mut Y,
    ) -> Result<(ResourceAddress, Bucket), RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let global_node_id = api.kernel_allocate_node_id(RENodeType::GlobalResourceManager)?;
        let resource_address: ResourceAddress = global_node_id.into();

        let mut non_fungible_entries = BTreeMap::new();
        for entry in entries {
            let uuid = Runtime::generate_uuid(api)?;
            let id = NonFungibleLocalId::uuid(uuid).unwrap();
            non_fungible_entries.insert(id, entry);
        }

        let (resource_manager, nf_store_id) = build_non_fungible_resource_manager_substate(
            resource_address,
            NonFungibleIdType::UUID,
            non_fungible_entries.len(),
            BTreeSet::new(),
            non_fungible_schema,
            api,
        )?;

        let bucket = build_non_fungible_bucket(
            resource_address,
            NonFungibleIdType::UUID,
            nf_store_id,
            non_fungible_entries,
            api,
        )?;

        globalize_non_fungible_resource_manager(
            resource_address,
            resource_manager,
            access_rules,
            metadata,
            api,
        )?;

        Ok((resource_address, bucket))
    }

    pub(crate) fn mint_non_fungible<Y>(
        receiver: RENodeId,
        entries: BTreeMap<NonFungibleLocalId, Vec<u8>>,
        api: &mut Y,
    ) -> Result<Bucket, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let (bucket_id, non_fungibles) = {
            let resource_manager: &mut NonFungibleResourceManagerSubstate =
                api.kernel_get_substate_ref_mut(resman_handle)?;
            let resource_address = resource_manager.resource_address;
            if resource_manager.id_type == NonFungibleIdType::UUID {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(
                        ResourceManagerError::InvalidNonFungibleIdType,
                    ),
                ));
            }

            let amount: Decimal = entries.len().into();
            resource_manager.total_supply += amount;
            // Allocate non-fungibles
            let mut ids = BTreeSet::new();
            let mut non_fungibles = BTreeMap::new();
            for (id, data) in entries.clone().into_iter() {
                if id.id_type() != resource_manager.id_type {
                    return Err(RuntimeError::ApplicationError(
                        ApplicationError::ResourceManagerError(
                            ResourceManagerError::NonFungibleIdTypeDoesNotMatch(
                                id.id_type(),
                                resource_manager.id_type,
                            ),
                        ),
                    ));
                }

                //let non_fungible = NonFungible::new(data.0, data.1);
                let non_fungible: ScryptoValue = scrypto_decode(&data).unwrap();
                ids.insert(id.clone());
                non_fungibles.insert(id, non_fungible);
            }

            let info = BucketInfoSubstate {
                resource_address,
                resource_type: ResourceType::NonFungible {
                    id_type: resource_manager.id_type,
                },
            };
            let bucket_id = api.new_object(
                BUCKET_BLUEPRINT,
                vec![
                    scrypto_encode(&info).unwrap(),
                    scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                    scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                    scrypto_encode(&LiquidNonFungibleResource::new(ids)).unwrap(),
                    scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
                ],
            )?;

            (bucket_id, non_fungibles)
        };

        let (nf_store_id, resource_address) = {
            let resource_manager: &NonFungibleResourceManagerSubstate =
                api.kernel_get_substate_ref(resman_handle)?;
            (
                resource_manager.non_fungible_table,
                resource_manager.resource_address,
            )
        };

        for (id, non_fungible) in non_fungibles {
            let non_fungible_handle = api.sys_lock_substate(
                RENodeId::KeyValueStore(nf_store_id),
                SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                    scrypto_encode(&id).unwrap(),
                )),
                LockFlags::MUTABLE,
            )?;

            {
                let non_fungible_mut: &mut Option<ScryptoValue> =
                    api.kernel_get_substate_ref_mut(non_fungible_handle)?;

                if let Option::Some(..) = non_fungible_mut {
                    return Err(RuntimeError::ApplicationError(
                        ApplicationError::ResourceManagerError(
                            ResourceManagerError::NonFungibleAlreadyExists(
                                NonFungibleGlobalId::new(resource_address, id),
                            ),
                        ),
                    ));
                }

                // FIXME: verify data
                //let value: ScryptoValue =
                //scrypto_decode(&scrypto_encode(&non_fungible).unwrap()).unwrap();
                *non_fungible_mut = Option::Some(non_fungible);
            }

            api.sys_drop_lock(non_fungible_handle)?;
        }

        Runtime::emit_event(
            api,
            MintNonFungibleResourceEvent {
                ids: entries.into_iter().map(|(k, _)| k).collect(),
            },
        )?;

        Ok(Bucket(bucket_id))
    }

    pub(crate) fn mint_uuid_non_fungible<Y>(
        receiver: RENodeId,
        entries: Vec<Vec<u8>>,
        api: &mut Y,
    ) -> Result<Bucket, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let (bucket_id, ids) = {
            let resource_manager: &mut NonFungibleResourceManagerSubstate =
                api.kernel_get_substate_ref_mut(resman_handle)?;
            let resource_address = resource_manager.resource_address;
            let nf_store_id = resource_manager.non_fungible_table;
            let id_type = resource_manager.id_type;

            if id_type != NonFungibleIdType::UUID {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(
                        ResourceManagerError::InvalidNonFungibleIdType,
                    ),
                ));
            }

            let amount: Decimal = entries.len().into();
            resource_manager.total_supply += amount;
            // Allocate non-fungibles
            let mut ids = BTreeSet::new();
            for data in entries {
                // TODO: Is this enough bits to prevent hash collisions?
                // TODO: Possibly use an always incrementing timestamp
                let uuid = Runtime::generate_uuid(api)?;
                let id = NonFungibleLocalId::uuid(uuid).unwrap();
                ids.insert(id.clone());

                {
                    let non_fungible_handle = api.sys_lock_substate(
                        RENodeId::KeyValueStore(nf_store_id),
                        SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                            scrypto_encode(&id).unwrap(),
                        )),
                        LockFlags::MUTABLE,
                    )?;
                    let non_fungible_mut: &mut Option<ScryptoValue> =
                        api.kernel_get_substate_ref_mut(non_fungible_handle)?;

                    // FIXME: verify data
                    //let non_fungible = NonFungible::new(data.0, data.1);
                    let value: ScryptoValue = scrypto_decode(&data).unwrap();
                    //scrypto_decode(&scrypto_encode(&non_fungible).unwrap()).unwrap();
                    *non_fungible_mut = Option::Some(value);

                    api.sys_drop_lock(non_fungible_handle)?;
                }
            }

            let info = BucketInfoSubstate {
                resource_address,
                resource_type: ResourceType::NonFungible { id_type },
            };
            let bucket_id = api.new_object(
                BUCKET_BLUEPRINT,
                vec![
                    scrypto_encode(&info).unwrap(),
                    scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                    scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                    scrypto_encode(&LiquidNonFungibleResource::new(ids.clone())).unwrap(),
                    scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
                ],
            )?;

            (bucket_id, ids)
        };

        Runtime::emit_event(api, MintNonFungibleResourceEvent { ids })?;

        Ok(Bucket(bucket_id))
    }

    pub(crate) fn update_non_fungible_data<Y>(
        receiver: RENodeId,
        id: NonFungibleLocalId,
        data: Vec<u8>,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_address = resource_manager.resource_address;
        let non_fungible_table_id = resource_manager.non_fungible_table;

        let non_fungible_handle = api.sys_lock_substate(
            RENodeId::KeyValueStore(non_fungible_table_id),
            SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                scrypto_encode(&id).unwrap(),
            )),
            LockFlags::MUTABLE,
        )?;
        let non_fungible_mut: &mut Option<ScryptoValue> =
            api.kernel_get_substate_ref_mut(non_fungible_handle)?;
        if let Some(ref mut non_fungible_substate) = non_fungible_mut {
            *non_fungible_substate = scrypto_decode(&data).unwrap();
        } else {
            let non_fungible_global_id = NonFungibleGlobalId::new(resource_address, id);
            return Err(RuntimeError::ApplicationError(
                ApplicationError::ResourceManagerError(ResourceManagerError::NonFungibleNotFound(
                    non_fungible_global_id,
                )),
            ));
        }

        api.sys_drop_lock(non_fungible_handle)?;

        Ok(())
    }

    pub(crate) fn non_fungible_exists<Y>(
        receiver: RENodeId,
        id: NonFungibleLocalId,
        api: &mut Y,
    ) -> Result<bool, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::read_only(),
        )?;

        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let non_fungible_table_id = resource_manager.non_fungible_table;

        let non_fungible_handle = api.sys_lock_substate(
            RENodeId::KeyValueStore(non_fungible_table_id),
            SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                scrypto_encode(&id).unwrap(),
            )),
            LockFlags::read_only(),
        )?;
        let non_fungible: &Option<ScryptoValue> =
            api.kernel_get_substate_ref(non_fungible_handle)?;
        let exists = matches!(non_fungible, Option::Some(..));

        Ok(exists)
    }

    pub(crate) fn get_non_fungible<Y>(
        receiver: RENodeId,
        id: NonFungibleLocalId,
        api: &mut Y,
    ) -> Result<ScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::read_only(),
        )?;

        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let non_fungible_table_id = resource_manager.non_fungible_table;

        let non_fungible_global_id =
            NonFungibleGlobalId::new(resource_manager.resource_address, id.clone());

        let non_fungible_handle = api.sys_lock_substate(
            RENodeId::KeyValueStore(non_fungible_table_id),
            SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                scrypto_encode(&id).unwrap(),
            )),
            LockFlags::read_only(),
        )?;
        let wrapper: &Option<ScryptoValue> = api.kernel_get_substate_ref(non_fungible_handle)?;
        if let Some(non_fungible) = wrapper {
            Ok(non_fungible.clone())
        } else {
            Err(RuntimeError::ApplicationError(
                ApplicationError::ResourceManagerError(ResourceManagerError::NonFungibleNotFound(
                    non_fungible_global_id,
                )),
            ))
        }
    }

    pub(crate) fn create_bucket<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<Bucket, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_address = resource_manager.resource_address;
        let id_type = resource_manager.id_type;
        let bucket_id = api.new_object(
            BUCKET_BLUEPRINT,
            vec![
                scrypto_encode(&BucketInfoSubstate {
                    resource_address,
                    resource_type: ResourceType::NonFungible { id_type },
                })
                    .unwrap(),
                scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                scrypto_encode(&LiquidNonFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
            ],
        )?;

        Ok(Bucket(bucket_id))

    }

    pub(crate) fn burn<Y>(
        receiver: RENodeId,
        bucket: Bucket,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        // FIXME: check if the bucket is locked!!!
        let dropped_bucket: DroppedBucket = api
            .kernel_drop_node(RENodeId::Object(bucket.0))?
            .into();

        // Construct the event and only emit it once all of the operations are done.
        match dropped_bucket.resource {
            DroppedBucketResource::Fungible(..) => {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(
                        ResourceManagerError::MismatchingBucketResource,
                    ),
                ));
            }
            DroppedBucketResource::NonFungible(resource) => {
                Runtime::emit_event(
                    api,
                    BurnNonFungibleResourceEvent {
                        ids: resource.ids().clone(),
                    },
                )?;

                // Check if resource matches
                // TODO: Move this check into actor check
                {
                    let resource_manager: &mut NonFungibleResourceManagerSubstate =
                        api.kernel_get_substate_ref_mut(resman_handle)?;
                    if dropped_bucket.info.resource_address != resource_manager.resource_address {
                        return Err(RuntimeError::ApplicationError(
                            ApplicationError::ResourceManagerError(
                                ResourceManagerError::MismatchingBucketResource,
                            ),
                        ));
                    }

                    // Update total supply
                    // TODO: there might be better for maintaining total supply, especially for non-fungibles
                    // Update total supply
                    resource_manager.total_supply -= resource.amount();

                    // Burn non-fungible
                    let node_id = RENodeId::KeyValueStore(resource_manager.non_fungible_table);

                    for id in resource.into_ids() {
                        let non_fungible_handle = api.sys_lock_substate(
                            node_id,
                            SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                                scrypto_encode(&id).unwrap(),
                            )),
                            LockFlags::MUTABLE,
                        )?;

                        let non_fungible_mut: &mut Option<ScryptoValue> =
                            api.kernel_get_substate_ref_mut(non_fungible_handle)?;
                        *non_fungible_mut = Option::None;
                        api.sys_drop_lock(non_fungible_handle)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn create_vault<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<Own, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_address = resource_manager.resource_address;
        let id_type = resource_manager.id_type;
        let info = VaultInfoSubstate {
            resource_address,
            resource_type: ResourceType::NonFungible { id_type },
        };
        let vault_id = api.new_object(
            VAULT_BLUEPRINT,
            vec![
                scrypto_encode(&info).unwrap(),
                scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                scrypto_encode(&LiquidNonFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
            ],
        )?;

        Runtime::emit_event(
            api,
            VaultCreationEvent {
                vault_id: RENodeId::Object(vault_id),
            },
        )?;

        Ok(Own::Vault(vault_id))
    }

    pub(crate) fn get_resource_type<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<ResourceType, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::read_only(),
        )?;

        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_type = ResourceType::NonFungible {
            id_type: resource_manager.id_type,
        };

        Ok(resource_type)
    }

    pub(crate) fn get_total_supply<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<Decimal, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::read_only(),
        )?;
        let resource_manager: &NonFungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let total_supply = resource_manager.total_supply;
        Ok(total_supply)
    }
}

pub struct ResourceManagerBlueprint;

impl ResourceManagerBlueprint {

    pub(crate) fn create_fungible<Y>(
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: ResourceManagerCreateFungibleInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        let global_node_id = api.kernel_allocate_node_id(RENodeType::GlobalResourceManager)?;
        let address = create_fungible_resource_manager(
            global_node_id,
            input.divisibility,
            input.metadata,
            input.access_rules,
            api,
        )?;
        Ok(IndexedScryptoValue::from_typed(&address))
    }

    pub(crate) fn create_fungible_with_initial_supply<Y>(
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: ResourceManagerCreateFungibleWithInitialSupplyInput =
            input.as_typed().map_err(|e| {
                RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
            })?;

        let global_node_id = api.kernel_allocate_node_id(RENodeType::GlobalResourceManager)?;
        let resource_address: ResourceAddress = global_node_id.into();

        let (resource_manager_substate, bucket) =
            build_fungible_resource_manager_substate_with_initial_supply(
                resource_address,
                input.divisibility,
                input.initial_supply,
                api,
            )?;

        let object_id = api.new_object(
            RESOURCE_MANAGER_BLUEPRINT,
            vec![scrypto_encode(&resource_manager_substate).unwrap()],
        )?;

        let (resman_access_rules, vault_access_rules) = build_access_rules(input.access_rules);
        let resman_access_rules = AccessRulesObject::sys_new(resman_access_rules, api)?;
        let vault_access_rules = AccessRulesObject::sys_new(vault_access_rules, api)?;
        let metadata = Metadata::sys_create_with_data(input.metadata, api)?;

        api.globalize_with_address(
            RENodeId::Object(object_id),
            btreemap!(
                NodeModuleId::AccessRules => resman_access_rules.id(),
                NodeModuleId::AccessRules1 => vault_access_rules.id(),
                NodeModuleId::Metadata => metadata.id(),
            ),
            resource_address.into(),
        )?;

        Ok(IndexedScryptoValue::from_typed(&(resource_address, bucket)))
    }

    pub(crate) fn create_fungible_with_initial_supply_and_address<Y>(
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: ResourceManagerCreateFungibleWithInitialSupplyAndAddressInput =
            input.as_typed().map_err(|e| {
                RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
            })?;

        let global_node_id =
            RENodeId::GlobalObject(ResourceAddress::Normal(input.resource_address).into());
        let resource_address: ResourceAddress = global_node_id.into();

        let (resource_manager_substate, bucket) =
            build_fungible_resource_manager_substate_with_initial_supply(
                resource_address,
                input.divisibility,
                input.initial_supply,
                api,
            )?;

        let object_id = api.new_object(
            RESOURCE_MANAGER_BLUEPRINT,
            vec![scrypto_encode(&resource_manager_substate).unwrap()],
        )?;

        let (resman_access_rules, vault_access_rules) = build_access_rules(input.access_rules);
        let resman_access_rules = AccessRulesObject::sys_new(resman_access_rules, api)?;
        let vault_access_rules = AccessRulesObject::sys_new(vault_access_rules, api)?;
        let metadata = Metadata::sys_create_with_data(input.metadata, api)?;

        api.globalize_with_address(
            RENodeId::Object(object_id),
            btreemap!(
                NodeModuleId::AccessRules => resman_access_rules.id(),
                NodeModuleId::AccessRules1 => vault_access_rules.id(),
                NodeModuleId::Metadata => metadata.id(),
            ),
            resource_address.into(),
        )?;

        Ok(IndexedScryptoValue::from_typed(&(resource_address, bucket)))
    }

    pub(crate) fn burn_bucket<Y>(
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: ResourceManagerBurnBucketInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        if input.bucket.sys_amount(api)?.is_zero() {
            api.kernel_drop_node(RENodeId::Object(input.bucket.0))?;
        } else {
            let resource_address = input.bucket.sys_resource_address(api)?;
            native_sdk::resource::ResourceManager(resource_address).burn(input.bucket, api)?;
        }

        Ok(IndexedScryptoValue::from_typed(&()))
    }



    pub(crate) fn mint_fungible<Y>(
        receiver: RENodeId,
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: ResourceManagerMintFungibleInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let bucket_id = {
            let resource_manager: &mut FungibleResourceManagerSubstate =
                api.kernel_get_substate_ref_mut(resman_handle)?;
            let divisibility = resource_manager.divisibility;
            let resource_type = ResourceType::Fungible { divisibility };

            // check amount
            if !resource_type.check_amount(input.amount) {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(ResourceManagerError::InvalidAmount(
                        input.amount,
                        divisibility,
                    )),
                ));
            }

            // Practically impossible to overflow the Decimal type with this limit in place.
            if input.amount > dec!("1000000000000000000") {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(
                        ResourceManagerError::MaxMintAmountExceeded,
                    ),
                ));
            }

            resource_manager.total_supply += input.amount;

            let bucket_info = BucketInfoSubstate {
                resource_address: resource_manager.resource_address,
                resource_type: ResourceType::Fungible { divisibility },
            };
            let liquid_resource = LiquidFungibleResource::new(input.amount);
            let bucket_id = api.new_object(
                BUCKET_BLUEPRINT,
                vec![
                    scrypto_encode(&bucket_info).unwrap(),
                    scrypto_encode(&liquid_resource).unwrap(),
                    scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                    scrypto_encode(&LiquidNonFungibleResource::default()).unwrap(),
                    scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
                ],
            )?;

            bucket_id
        };

        Runtime::emit_event(
            api,
            MintFungibleResourceEvent {
                amount: input.amount,
            },
        )?;

        Ok(IndexedScryptoValue::from_typed(&Bucket(bucket_id)))
    }

    pub(crate) fn burn<Y>(
        receiver: RENodeId,
        bucket: Bucket,
        api: &mut Y,
    ) -> Result<(), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        // FIXME: check if the bucket is locked!!!
        let dropped_bucket: DroppedBucket = api
            .kernel_drop_node(RENodeId::Object(bucket.0))?
            .into();

        // Construct the event and only emit it once all of the operations are done.
        match dropped_bucket.resource {
            DroppedBucketResource::Fungible(resource) => {
                Runtime::emit_event(
                    api,
                    BurnFungibleResourceEvent {
                        amount: resource.amount(),
                    },
                )?;

                // Check if resource matches
                // TODO: Move this check into actor check
                {
                    let resource_manager: &mut FungibleResourceManagerSubstate =
                        api.kernel_get_substate_ref_mut(resman_handle)?;
                    if dropped_bucket.info.resource_address != resource_manager.resource_address {
                        return Err(RuntimeError::ApplicationError(
                            ApplicationError::ResourceManagerError(
                                ResourceManagerError::MismatchingBucketResource,
                            ),
                        ));
                    }

                    // Update total supply
                    // TODO: there might be better for maintaining total supply, especially for non-fungibles
                    // Update total supply
                    resource_manager.total_supply -= resource.amount();
                }
            }
            DroppedBucketResource::NonFungible(..) => {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::ResourceManagerError(
                        ResourceManagerError::MismatchingBucketResource,
                    ),
                ));
            }
        }

        Ok(())
    }

    pub(crate) fn create_bucket<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<Bucket, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let resource_manager: &FungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_address = resource_manager.resource_address;
        let divisibility = resource_manager.divisibility;
        let bucket_id = api.new_object(
            BUCKET_BLUEPRINT,
            vec![
                scrypto_encode(&BucketInfoSubstate {
                    resource_address,
                    resource_type: ResourceType::Fungible { divisibility },
                })
                    .unwrap(),
                scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                scrypto_encode(&LiquidNonFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
            ],
        )?;

        Ok(Bucket(bucket_id))

    }

    pub(crate) fn create_vault<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<Own, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::MUTABLE,
        )?;

        let resource_manager: &FungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_address = resource_manager.resource_address;
        let divisibility = resource_manager.divisibility;
        let info = VaultInfoSubstate {
            resource_address,
            resource_type: ResourceType::Fungible { divisibility },
        };
        let vault_id = api.new_object(
            VAULT_BLUEPRINT,
            vec![
                scrypto_encode(&info).unwrap(),
                scrypto_encode(&LiquidFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedFungibleResource::default()).unwrap(),
                scrypto_encode(&LiquidNonFungibleResource::default()).unwrap(),
                scrypto_encode(&LockedNonFungibleResource::default()).unwrap(),
            ],
        )?;

        Runtime::emit_event(
            api,
            VaultCreationEvent {
                vault_id: RENodeId::Object(vault_id),
            },
        )?;

        Ok(Own::Vault(vault_id))
    }



    pub(crate) fn get_resource_type<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<ResourceType, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::read_only(),
        )?;

        let resource_manager: &FungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let resource_type = ResourceType::Fungible {
            divisibility: resource_manager.divisibility,
        };

        Ok(resource_type)
    }

    pub(crate) fn get_total_supply<Y>(
        receiver: RENodeId,
        api: &mut Y,
    ) -> Result<Decimal, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let resman_handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager),
            LockFlags::read_only(),
        )?;
        let resource_manager: &FungibleResourceManagerSubstate =
            api.kernel_get_substate_ref(resman_handle)?;
        let total_supply = resource_manager.total_supply;
        Ok(total_supply)
    }


}

fn create_fungible_resource_manager<Y>(
    global_node_id: RENodeId,
    divisibility: u8,
    metadata: BTreeMap<String, String>,
    access_rules: BTreeMap<ResourceMethodAuthKey, (AccessRule, AccessRule)>,
    api: &mut Y,
) -> Result<ResourceAddress, RuntimeError>
where
    Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
{
    let resource_address: ResourceAddress = global_node_id.into();

    let resource_manager_substate =
        FungibleResourceManagerSubstate::new(divisibility, resource_address);

    let object_id = api.new_object(
        RESOURCE_MANAGER_BLUEPRINT,
        vec![scrypto_encode(&resource_manager_substate).unwrap()],
    )?;

    let (resman_access_rules, vault_access_rules) = build_access_rules(access_rules);
    let resman_access_rules = AccessRulesObject::sys_new(resman_access_rules, api)?;
    let vault_access_rules = AccessRulesObject::sys_new(vault_access_rules, api)?;
    let metadata = Metadata::sys_create_with_data(metadata, api)?;

    api.globalize_with_address(
        RENodeId::Object(object_id),
        btreemap!(
            NodeModuleId::AccessRules => resman_access_rules.id(),
            NodeModuleId::AccessRules1 => vault_access_rules.id(),
            NodeModuleId::Metadata => metadata.id(),
        ),
        resource_address.into(),
    )?;

    Ok(resource_address)
}
