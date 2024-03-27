use super::*;
use crate::internal_prelude::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::locker::*;
use radix_native_sdk::modules::metadata::*;
use radix_native_sdk::modules::role_assignment::*;
use radix_native_sdk::resource::*;
use radix_native_sdk::runtime::*;

pub const STORER_ROLE: &str = "storer";
pub const STORER_UPDATER_ROLE: &str = "storer_updater";
pub const RECOVERER_ROLE: &str = "recoverer";
pub const RECOVERER_UPDATER_ROLE: &str = "recoverer_updater";

pub struct AccountLockerBlueprint;

#[allow(unused_variables)]
impl AccountLockerBlueprint {
    pub fn definition() -> BlueprintDefinitionInit {
        let mut aggregator = TypeAggregator::<ScryptoCustomTypeKind>::new();

        let feature_set = AccountLockerFeatureSet::all_features();
        let state = AccountLockerStateSchemaInit::create_schema_init(&mut aggregator);

        let functions = function_schema! {
            aggregator,
            AccountLocker {
                instantiate: None,
                instantiate_simple: None,
                store: Some(ReceiverInfo::normal_ref_mut()),
                store_batch: Some(ReceiverInfo::normal_ref_mut()),
                send_or_store: Some(ReceiverInfo::normal_ref_mut()),
                send_or_store_batch: Some(ReceiverInfo::normal_ref_mut()),
                recover: Some(ReceiverInfo::normal_ref_mut()),
                recover_non_fungibles: Some(ReceiverInfo::normal_ref_mut()),
                claim: Some(ReceiverInfo::normal_ref_mut()),
                claim_non_fungibles: Some(ReceiverInfo::normal_ref_mut()),
                get_amount: Some(ReceiverInfo::normal_ref()),
                get_non_fungible_local_ids: Some(ReceiverInfo::normal_ref()),
            }
        };

        let events = event_schema! {
            aggregator,
            [
                StoreEvent,
                BatchStoreEvent,
                RecoveryEvent,
                ClaimEvent,
            ]
        };

        let schema = generate_full_schema(aggregator);

        BlueprintDefinitionInit {
            blueprint_type: BlueprintType::default(),
            is_transient: false,
            feature_set,
            dependencies: indexset!(),
            schema: BlueprintSchemaInit {
                generics: vec![],
                schema,
                state,
                events,
                types: BlueprintTypeSchemaInit::default(),
                functions: BlueprintFunctionsSchemaInit { functions },
                hooks: BlueprintHooksInit::default(),
            },

            royalty_config: PackageRoyaltyConfig::default(),
            auth_config: AuthConfig {
                function_auth: FunctionAuth::AllowAll,
                method_auth: MethodAuthTemplate::StaticRoleDefinition(roles_template!(
                    roles {
                        STORER_ROLE => updaters: [STORER_UPDATER_ROLE];
                        STORER_UPDATER_ROLE => updaters: [STORER_UPDATER_ROLE];
                        RECOVERER_ROLE => updaters: [RECOVERER_UPDATER_ROLE];
                        RECOVERER_UPDATER_ROLE => updaters: [RECOVERER_UPDATER_ROLE];
                    },
                    methods {
                        ACCOUNT_LOCKER_STORE_IDENT => [STORER_ROLE];
                        ACCOUNT_LOCKER_STORE_BATCH_IDENT => [STORER_ROLE];
                        ACCOUNT_LOCKER_SEND_OR_STORE_IDENT => [STORER_ROLE];
                        ACCOUNT_LOCKER_SEND_OR_STORE_BATCH_IDENT => [STORER_ROLE];

                        ACCOUNT_LOCKER_RECOVER_IDENT => [RECOVERER_ROLE];
                        ACCOUNT_LOCKER_RECOVER_NON_FUNGIBLES_IDENT => [RECOVERER_ROLE];

                        ACCOUNT_LOCKER_CLAIM_IDENT => MethodAccessibility::Public;
                        ACCOUNT_LOCKER_CLAIM_NON_FUNGIBLES_IDENT => MethodAccessibility::Public;
                        ACCOUNT_LOCKER_GET_AMOUNT_IDENT => MethodAccessibility::Public;
                        ACCOUNT_LOCKER_GET_NON_FUNGIBLE_LOCAL_IDS_IDENT => MethodAccessibility::Public;
                    }
                )),
            },
        }
    }

    pub fn invoke_export<Y>(
        export_name: &str,
        input: &IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        dispatch! {
            export_name,
            input,
            api,
            AccountLocker,
            [
                instantiate,
                instantiate_simple,
                store,
                store_batch,
                send_or_store,
                send_or_store_batch,
                recover,
                recover_non_fungibles,
                claim,
                claim_non_fungibles,
                get_amount,
                get_non_fungible_local_ids,
            ]
        }
    }

    fn instantiate<Y>(
        AccountLockerInstantiateInput {
            owner_role,
            storer_role,
            storer_updater_role,
            recoverer_role,
            recoverer_updater_role,
            address_reservation,
        }: AccountLockerInstantiateInput,
        api: &mut Y,
    ) -> Result<AccountLockerInstantiateOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Main module
        let object_id = api.new_simple_object(ACCOUNT_LOCKER_BLUEPRINT, indexmap! {})?;

        // Role Assignment Module
        let roles = indexmap! {
            ModuleId::Main => roles2! {
                STORER_ROLE => storer_role, updatable;
                STORER_UPDATER_ROLE => storer_updater_role, updatable;
                RECOVERER_ROLE => recoverer_role, updatable;
                RECOVERER_UPDATER_ROLE => recoverer_updater_role, updatable;
            }
        };
        let role_assignment = RoleAssignment::create(owner_role, roles, api)?.0;

        // Metadata Module
        let metadata = Metadata::create(api)?;

        // Globalize
        let address = api.globalize(
            object_id,
            indexmap!(
                AttachedModuleId::RoleAssignment => role_assignment.0,
                AttachedModuleId::Metadata => metadata.0,
            ),
            address_reservation,
        )?;
        let component_address = ComponentAddress::new_or_panic(address.as_node_id().0);

        Ok(Global::new(component_address))
    }

    fn instantiate_simple<Y>(
        AccountLockerInstantiateSimpleInput {
            allow_forceful_withdraws,
        }: AccountLockerInstantiateSimpleInput,
        api: &mut Y,
    ) -> Result<AccountLockerInstantiateSimpleOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Creating a new resource for the admin badge. We will first allocate a new address for it
        // and then instantiate it to its own address. The admin badge that we create will be its
        // own owner.
        let (badge_reservation, badge_address) = api.allocate_global_address(BlueprintId {
            package_address: RESOURCE_PACKAGE,
            blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.into(),
        })?;
        let badge_address = ResourceAddress::new_or_panic(badge_address.as_node_id().0);

        let (_, badge) = api
            .call_function(
                RESOURCE_PACKAGE,
                FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT,
                scrypto_encode(&FungibleResourceManagerCreateWithInitialSupplyInput {
                    owner_role: OwnerRole::Updatable(rule!(require(badge_address))),
                    track_total_supply: true,
                    divisibility: 0,
                    resource_roles: Default::default(),
                    metadata: metadata! {
                        init {
                            "name" => "Account Locker Admin Badge".to_owned(), locked;
                        }
                    },
                    address_reservation: Some(badge_reservation),
                    initial_supply: dec!(1),
                })
                .unwrap(),
            )
            .map(|rtn| {
                scrypto_decode::<FungibleResourceManagerCreateWithInitialSupplyOutput>(&rtn)
                    .unwrap()
            })?;

        // Preparing all of the roles and rules.
        let rule = rule!(require(badge_address));
        let recoverer_rule = match allow_forceful_withdraws {
            true => rule.clone(),
            false => rule!(deny_all),
        };

        Self::instantiate(
            AccountLockerInstantiateInput {
                owner_role: OwnerRole::Updatable(rule.clone()),
                storer_role: rule.clone(),
                storer_updater_role: rule.clone(),
                recoverer_role: recoverer_rule.clone(),
                recoverer_updater_role: recoverer_rule,
                address_reservation: None,
            },
            api,
        )
        .map(|rtn| (rtn, badge))
    }

    fn store<Y>(
        AccountLockerStoreInput { claimant, bucket }: AccountLockerStoreInput,
        api: &mut Y,
    ) -> Result<AccountLockerStoreOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Bucket info.
        let (resource_address, resource_specifier) = bucket_to_resource_specifier(&bucket, api)?;

        // Store in the vault.
        Self::with_vault_create_on_traversal(
            claimant.0,
            resource_address,
            api,
            |mut vault, api| vault.put(bucket, api),
        )??;

        // Emit an event with the stored resource
        Runtime::emit_event(
            api,
            StoreEvent {
                claimant,
                resource_address,
                resources: resource_specifier,
            },
        )?;

        Ok(())
    }

    fn store_batch<Y>(
        AccountLockerStoreBatchInput { claimants, bucket }: AccountLockerStoreBatchInput,
        api: &mut Y,
    ) -> Result<AccountLockerStoreBatchOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Store in the vaults
        let resource_address = bucket.resource_address(api)?;
        for (account_address, specifier) in claimants.iter() {
            let claim_bucket = match specifier {
                ResourceSpecifier::Fungible(amount) => bucket.take(*amount, api)?,
                ResourceSpecifier::NonFungible(ids) => {
                    bucket.take_non_fungibles(ids.clone(), api)?
                }
            };

            Self::with_vault_create_on_traversal(
                account_address.0,
                resource_address,
                api,
                |mut vault, api| vault.put(claim_bucket, api),
            )??;
        }

        // Emit an event
        Runtime::emit_event(
            api,
            BatchStoreEvent {
                claimants,
                resource_address,
            },
        )?;

        // Return the change
        if bucket.is_empty(api)? {
            bucket.drop_empty(api)?;
            Ok(None)
        } else {
            Ok(Some(bucket))
        }
    }

    fn send_or_store<Y>(
        AccountLockerSendOrStoreInput { claimant, bucket }: AccountLockerSendOrStoreInput,
        api: &mut Y,
    ) -> Result<AccountLockerSendOrStoreOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Getting the node-id of the actor and constructing the non-fungible global id of the
        // global caller.
        let actor_node_id = api.actor_get_node_id(ACTOR_STATE_SELF)?;
        let global_caller_non_fungible_global_id =
            global_caller(GlobalAddress::new_or_panic(actor_node_id.0));

        // Attempting to deposit the resources into the account.
        let bucket = api
            .call_method(
                claimant.0.as_node_id(),
                ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                scrypto_encode(&AccountTryDepositOrRefundInput {
                    bucket,
                    // TODO: Add the global caller badge later on once we change the account
                    // behavior.
                    // authorized_depositor_badge: Some(global_caller_non_fungible_global_id),
                    authorized_depositor_badge: None,
                })
                .unwrap(),
            )
            .map(|rtn| scrypto_decode::<AccountTryDepositOrRefundOutput>(&rtn).unwrap())?;

        // If we got a bucket back then we need to store it.
        if let Some(bucket) = bucket {
            Self::store(AccountLockerStoreInput { claimant, bucket }, api)?;
        }

        Ok(())
    }

    fn send_or_store_batch<Y>(
        AccountLockerSendOrStoreBatchInput { claimants, bucket  }: AccountLockerSendOrStoreBatchInput,
        api: &mut Y,
    ) -> Result<AccountLockerSendOrStoreBatchOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Getting the node-id of the actor and constructing the non-fungible global id of the
        // global caller.
        let actor_node_id = api.actor_get_node_id(ACTOR_STATE_SELF)?;
        let global_caller_non_fungible_global_id =
            global_caller(GlobalAddress::new_or_panic(actor_node_id.0));

        // First attempt to deposit the resources into their accounts
        let resource_address = bucket.resource_address(api)?;
        let mut failed_deposits = indexmap! {};
        for (account_address, specifier) in claimants.into_iter() {
            let claim_bucket = match specifier {
                ResourceSpecifier::Fungible(amount) => bucket.take(amount, api)?,
                ResourceSpecifier::NonFungible(ref ids) => {
                    bucket.take_non_fungibles(ids.clone(), api)?
                }
            };

            let bucket = api
                .call_method(
                    account_address.0.as_node_id(),
                    ACCOUNT_TRY_DEPOSIT_OR_REFUND_IDENT,
                    scrypto_encode(&AccountTryDepositOrRefundInput {
                        bucket: claim_bucket,
                        // TODO: Add the global caller badge later on once we change the account
                        // behavior.
                        // authorized_depositor_badge: Some(global_caller_non_fungible_global_id.clone()),
                        authorized_depositor_badge: None,
                    })
                    .unwrap(),
                )
                .map(|rtn| scrypto_decode::<AccountTryDepositOrRefundOutput>(&rtn).unwrap())?;

            // If the deposit failed then insert it into the failed deposits map and store it.
            if let Some(bucket) = bucket {
                // Store in the vault.
                Self::with_vault_create_on_traversal(
                    account_address.0,
                    resource_address,
                    api,
                    |mut vault, api| vault.put(bucket, api),
                )??;

                // Insert in map
                failed_deposits.insert(account_address, specifier);
            }
        }

        // Emit a batch store event with the ones that we failed to deposit.
        Runtime::emit_event(
            api,
            BatchStoreEvent {
                claimants: failed_deposits,
                resource_address,
            },
        )?;

        if bucket.is_empty(api)? {
            bucket.drop_empty(api)?;
            Ok(None)
        } else {
            Ok(Some(bucket))
        }
    }

    fn recover<Y>(
        AccountLockerRecoverInput {
            claimant,
            resource_address,
            amount,
        }: AccountLockerRecoverInput,
        api: &mut Y,
    ) -> Result<AccountLockerRecoverOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Recover the resources from the vault.
        let bucket = Self::with_vault_create_on_traversal(
            claimant.0,
            resource_address,
            api,
            |mut vault, api| vault.take(amount, api),
        )??;

        // Emitting the event
        let (resource_address, resource_specifier) = bucket_to_resource_specifier(&bucket, api)?;
        Runtime::emit_event(
            api,
            RecoveryEvent {
                claimant,
                resource_address,
                resources: resource_specifier,
            },
        )?;

        // Return
        Ok(bucket)
    }

    fn recover_non_fungibles<Y>(
        AccountLockerRecoverNonFungiblesInput {
            claimant,
            resource_address,
            ids,
        }: AccountLockerRecoverNonFungiblesInput,
        api: &mut Y,
    ) -> Result<AccountLockerRecoverNonFungiblesOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Recover the resources from the vault.
        let bucket = Self::with_vault_create_on_traversal(
            claimant.0,
            resource_address,
            api,
            |mut vault, api| vault.take_non_fungibles(ids, api),
        )??;

        // Emitting the event
        let (resource_address, resource_specifier) = bucket_to_resource_specifier(&bucket, api)?;
        Runtime::emit_event(
            api,
            RecoveryEvent {
                claimant,
                resource_address,
                resources: resource_specifier,
            },
        )?;

        // Return
        Ok(bucket)
    }

    fn claim<Y>(
        AccountLockerClaimInput {
            claimant,
            resource_address,
            amount,
        }: AccountLockerClaimInput,
        api: &mut Y,
    ) -> Result<AccountLockerClaimOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Read and assert against the owner role of the claimant.
        let claimant_owner_role = api
            .call_module_method(
                claimant.0.as_node_id(),
                AttachedModuleId::RoleAssignment,
                ROLE_ASSIGNMENT_GET_OWNER_ROLE_IDENT,
                scrypto_encode(&RoleAssignmentGetOwnerRoleInput).unwrap(),
            )
            .map(|rtn| scrypto_decode::<RoleAssignmentGetOwnerRoleOutput>(&rtn).unwrap())?;
        Runtime::assert_access_rule(claimant_owner_role.rule, api)?;

        // Recover the resources from the vault.
        let bucket = Self::with_vault_create_on_traversal(
            claimant.0,
            resource_address,
            api,
            |mut vault, api| vault.take(amount, api),
        )??;

        // Emitting the event
        let (resource_address, resource_specifier) = bucket_to_resource_specifier(&bucket, api)?;
        Runtime::emit_event(
            api,
            RecoveryEvent {
                claimant,
                resource_address,
                resources: resource_specifier,
            },
        )?;

        // Return
        Ok(bucket)
    }

    fn claim_non_fungibles<Y>(
        AccountLockerClaimNonFungiblesInput {
            claimant,
            resource_address,
            ids,
        }: AccountLockerClaimNonFungiblesInput,
        api: &mut Y,
    ) -> Result<AccountLockerClaimNonFungiblesOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // Read and assert against the owner role of the claimant.
        let claimant_owner_role = api
            .call_module_method(
                claimant.0.as_node_id(),
                AttachedModuleId::RoleAssignment,
                ROLE_ASSIGNMENT_GET_OWNER_ROLE_IDENT,
                scrypto_encode(&RoleAssignmentGetOwnerRoleInput).unwrap(),
            )
            .map(|rtn| scrypto_decode::<RoleAssignmentGetOwnerRoleOutput>(&rtn).unwrap())?;
        Runtime::assert_access_rule(claimant_owner_role.rule, api)?;

        // Recover the resources from the vault.
        let bucket = Self::with_vault_create_on_traversal(
            claimant.0,
            resource_address,
            api,
            |mut vault, api| vault.take_non_fungibles(ids, api),
        )??;

        // Emitting the event
        let (resource_address, resource_specifier) = bucket_to_resource_specifier(&bucket, api)?;
        Runtime::emit_event(
            api,
            RecoveryEvent {
                claimant,
                resource_address,
                resources: resource_specifier,
            },
        )?;

        // Return
        Ok(bucket)
    }

    fn get_amount<Y>(
        AccountLockerGetAmountInput {
            claimant,
            resource_address,
        }: AccountLockerGetAmountInput,
        api: &mut Y,
    ) -> Result<AccountLockerGetAmountOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        Self::with_vault(claimant.0, resource_address, api, |vault, api| {
            vault
                .map(|vault| vault.amount(api))
                .unwrap_or(Ok(Decimal::ZERO))
        })?
    }

    fn get_non_fungible_local_ids<Y>(
        AccountLockerGetNonFungibleLocalIdsInput {
            claimant,
            resource_address,
            limit,
        }: AccountLockerGetNonFungibleLocalIdsInput,
        api: &mut Y,
    ) -> Result<AccountLockerGetNonFungibleLocalIdsOutput, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        Self::with_vault(claimant.0, resource_address, api, |vault, api| {
            vault
                .map(|vault| vault.non_fungible_local_ids(limit, api))
                .unwrap_or(Ok(indexset! {}))
        })?
    }

    fn with_vault_create_on_traversal<Y, F, O>(
        account_address: ComponentAddress,
        resource_address: ResourceAddress,
        api: &mut Y,
        handler: F,
    ) -> Result<O, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
        F: FnOnce(Vault, &mut Y) -> O,
    {
        // The collection on the blueprint maps an account address to a key value store. We read the
        // node id of that key value store.
        let account_claims_handle = api.actor_open_key_value_entry(
            ACTOR_STATE_SELF,
            AccountLockerCollection::AccountClaimsKeyValue.collection_index(),
            &scrypto_encode(&account_address).unwrap(),
            LockFlags::MUTABLE,
        )?;
        let account_claims = api
            .key_value_entry_get_typed::<VersionedAccountLockerAccountClaims>(
                account_claims_handle,
            )?
            .map(|entry| entry.into_latest());

        let account_claims_kv_store = match account_claims {
            Some(account_claims_kv_store) => account_claims_kv_store,
            None => {
                // Create a new kv-store
                let key_value_store = api
                    .key_value_store_new(
                        KeyValueStoreDataSchema::new_local_without_self_package_replacement::<
                            ResourceAddress,
                            Vault,
                        >(true),
                    )
                    .map(Own)?;
                // Write the kv-store's node id to the collection entry.
                api.key_value_entry_set_typed(
                    account_claims_handle,
                    VersionedAccountLockerAccountClaims::V1(key_value_store),
                )?;
                // Return the NodeId of the kv-store.
                key_value_store
            }
        };

        // Lock the entry in the key-value store which contains the vault and attempt to get it. If
        // we're allowed to create the vault.
        let vault_entry_handle = api.key_value_store_open_entry(
            account_claims_kv_store.as_node_id(),
            &scrypto_encode(&resource_address).unwrap(),
            LockFlags::MUTABLE,
        )?;

        let vault_entry = api.key_value_entry_get_typed::<Vault>(vault_entry_handle)?;
        let vault = match vault_entry {
            Some(vault) => vault,
            None => {
                // Creating the vault.
                let vault = Vault::create(resource_address, api)?;
                // Writing it to the kv-entry
                api.key_value_entry_set_typed(vault_entry_handle, Vault(vault.0))?;
                // Return the vault.
                vault
            }
        };

        // Call the callback
        let rtn = handler(vault, api);

        // Close the opened kv-entries.
        api.key_value_entry_close(vault_entry_handle)?;
        api.key_value_entry_close(account_claims_handle)?;

        // Return the rtn result
        Ok(rtn)
    }

    fn with_vault<Y, F, O>(
        account_address: ComponentAddress,
        resource_address: ResourceAddress,
        api: &mut Y,
        handler: F,
    ) -> Result<O, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
        F: FnOnce(Option<Vault>, &mut Y) -> O,
    {
        // The collection on the blueprint maps an account address to a key value store. We read the
        // node id of that key value store.
        let account_claims_handle = api.actor_open_key_value_entry(
            ACTOR_STATE_SELF,
            AccountLockerCollection::AccountClaimsKeyValue.collection_index(),
            &scrypto_encode(&account_address).unwrap(),
            LockFlags::read_only(),
        )?;
        let account_claims = api
            .key_value_entry_get_typed::<VersionedAccountLockerAccountClaims>(
                account_claims_handle,
            )?
            .map(|entry| entry.into_latest());

        let account_claims_kv_store = match account_claims {
            Some(account_claims_kv_store) => account_claims_kv_store,
            None => {
                // Call the callback function.
                let rtn = handler(None, api);
                // Dropping the lock on the collection entry.
                api.key_value_entry_close(account_claims_handle)?;
                // Return the result of the callback.
                return Ok(rtn);
            }
        };

        // Lock the entry in the key-value store which contains the vault and attempt to get it. If
        // we're allowed to create the vault.
        let vault_entry_handle = api.key_value_store_open_entry(
            account_claims_kv_store.as_node_id(),
            &scrypto_encode(&resource_address).unwrap(),
            LockFlags::read_only(),
        )?;

        let vault_entry = api.key_value_entry_get_typed::<Vault>(vault_entry_handle)?;

        // Call the callback
        let rtn = handler(vault_entry, api);

        // Close the opened kv-entries.
        api.key_value_entry_close(vault_entry_handle)?;
        api.key_value_entry_close(account_claims_handle)?;

        // Return the rtn result
        Ok(rtn)
    }
}

fn bucket_to_resource_specifier<Y>(
    bucket: &Bucket,
    api: &mut Y,
) -> Result<(ResourceAddress, ResourceSpecifier), RuntimeError>
where
    Y: ClientApi<RuntimeError>,
{
    let resource_address = bucket.resource_address(api)?;
    if resource_address.is_fungible() {
        let amount = bucket.amount(api)?;
        Ok((resource_address, ResourceSpecifier::Fungible(amount)))
    } else {
        let ids = bucket.non_fungible_local_ids(api)?;
        Ok((resource_address, ResourceSpecifier::NonFungible(ids)))
    }
}