use scrypto::api::*;
use scrypto::prelude::*;

#[derive(Debug, PartialEq, Eq, ScryptoSbor, NonFungibleData)]
pub struct Sandwich {
    pub name: String,
    #[mutable]
    pub available: bool,
    pub tastes_great: bool,
    #[mutable]
    pub reference: Option<ComponentAddress>,
    #[mutable]
    pub own: Option<Own>,
}

#[blueprint]
mod non_fungible_test {
    struct NonFungibleTest {
        vault: Vault,
    }

    impl NonFungibleTest {
        pub fn create_non_fungible_mutable() -> (Bucket, ResourceManager, Bucket) {
            // Create a mint badge
            let mint_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();

            // Create non-fungible resource with mutable supply
            let resource_manager = ResourceBuilder::new_integer_non_fungible::<Sandwich>(
                OwnerRole::Fixed(rule!(require(mint_badge.resource_address()))),
            )
            .metadata(metadata! {
                init {
                    "name" => "Katz's Sandwiches".to_owned(), locked;
                }
            })
            .mint_roles(mint_roles! {
                minter => OWNER;
                minter_updater => rule!(deny_all);
            })
            .burn_roles(burn_roles! {
                burner => rule!(allow_all);
                burner_updater => rule!(deny_all);
            })
            .non_fungible_data_update_roles(non_fungible_data_update_roles! {
                non_fungible_data_updater => OWNER;
                non_fungible_data_updater_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            // Mint a non-fungible
            let non_fungible = mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                resource_manager.mint_non_fungible(
                    &NonFungibleLocalId::integer(0),
                    Sandwich {
                        name: "Test".to_owned(),
                        available: false,
                        tastes_great: true,
                        reference: None,
                        own: None,
                    },
                )
            });

            (mint_badge, resource_manager, non_fungible)
        }

        pub fn update_nft(mint_badge: Bucket, proof: Proof) -> Bucket {
            let proof = proof.skip_checking();
            mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                let resource_manager = proof.resource_manager();
                resource_manager.update_non_fungible_data(
                    &proof.as_non_fungible().non_fungible_local_id(),
                    "available",
                    true,
                )
            });

            mint_badge
        }

        pub fn create_burnable_non_fungible() -> Bucket {
            ResourceBuilder::new_ruid_non_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Katz's Sandwiches".to_owned(), locked;
                    }
                })
                .burn_roles(burn_roles! {
                    burner => rule!(allow_all);
                    burner_updater => rule!(deny_all);
                })
                .mint_initial_supply([
                    Sandwich {
                        name: "Zero".to_owned(),
                        available: true,
                        tastes_great: true,
                        reference: None,
                        own: None,
                    },
                    Sandwich {
                        name: "One".to_owned(),
                        available: true,
                        tastes_great: true,
                        reference: None,
                        own: None,
                    },
                ])
                .into()
        }

        pub fn create_non_fungible_fixed() -> Bucket {
            ResourceBuilder::new_integer_non_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Katz's Sandwiches".to_owned(), locked;
                    }
                })
                .mint_initial_supply([
                    (
                        1u64.into(),
                        Sandwich {
                            name: "One".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                    (
                        2u64.into(),
                        Sandwich {
                            name: "Two".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                    (
                        3u64.into(),
                        Sandwich {
                            name: "Three".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                ])
                .into()
        }

        pub fn verify_does_not_exist(non_fungible_global_id: NonFungibleGlobalId) {
            let manager: ResourceManager = non_fungible_global_id.resource_address().into();
            assert_eq!(
                manager.non_fungible_exists(&non_fungible_global_id.local_id()),
                false
            );
        }

        pub fn create_non_fungible_reference(address: ComponentAddress) -> (Bucket, Bucket) {
            let (mint_badge, resource_manager, bucket) = Self::create_non_fungible_mutable();

            mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                resource_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(0),
                    "reference",
                    Some(address),
                );
            });

            (mint_badge, bucket)
        }

        pub fn call_non_fungible_reference(resource_manager: ResourceManager) -> String {
            let data: Sandwich =
                resource_manager.get_non_fungible_data(&NonFungibleLocalId::integer(0));
            let address = data.reference.unwrap();

            let some_component: Global<AnyComponent> = address.into();
            some_component.get_metadata("test_key").unwrap()
        }

        pub fn update_non_fungible_with_ownership() -> Bucket {
            let (mint_badge, resource_manager, bucket) = Self::create_non_fungible_mutable();

            let vault = Vault::with_bucket(bucket);

            mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                resource_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(0),
                    "own",
                    Some(vault.0),
                );
            });

            mint_badge
        }

        pub fn update_non_fungible(field: String, value: bool) -> (Bucket, Bucket) {
            let (mint_badge, resource_manager, bucket) = Self::create_non_fungible_mutable();

            mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                resource_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(0),
                    &field,
                    value,
                );
            });

            (mint_badge, bucket)
        }

        pub fn update_and_get_non_fungible_reference(
            reference: ComponentAddress,
        ) -> (Bucket, Bucket) {
            let (mint_badge, resource_manager, bucket) = Self::create_non_fungible_mutable();

            mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                resource_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(0),
                    "reference",
                    Some(reference),
                );
            });

            let data: Sandwich =
                resource_manager.get_non_fungible_data(&NonFungibleLocalId::integer(0));
            assert_eq!(data.reference, Some(reference));
            (mint_badge, bucket)
        }

        pub fn update_and_get_non_fungible() -> (Bucket, Bucket) {
            let (mint_badge, resource_manager, bucket) = Self::create_non_fungible_mutable();
            let data: Sandwich =
                resource_manager.get_non_fungible_data(&NonFungibleLocalId::integer(0));
            assert_eq!(data.available, false);

            mint_badge.as_fungible().authorize_with_amount(dec!(1), || {
                resource_manager.update_non_fungible_data(
                    &NonFungibleLocalId::integer(0),
                    "available",
                    true,
                );
            });

            let data: Sandwich =
                resource_manager.get_non_fungible_data(&NonFungibleLocalId::integer(0));
            assert_eq!(data.available, true);
            (mint_badge, bucket)
        }

        pub fn get_total_supply() {
            let resource_manager =
                ResourceBuilder::new_integer_non_fungible::<Sandwich>(OwnerRole::None)
                    .metadata(metadata! {
                        init {
                            "name" => "Katz's Sandwiches".to_owned(), locked;
                        }
                    })
                    .create_with_no_initial_supply();

            assert_eq!(resource_manager.total_supply().unwrap(), Decimal::zero(),);
        }

        pub fn non_fungible_exists() -> (Bucket, Bucket) {
            let (mint_badge, resource_manager, bucket) = Self::create_non_fungible_mutable();
            assert_eq!(
                resource_manager.non_fungible_exists(&NonFungibleLocalId::integer(0)),
                true
            );
            assert_eq!(
                resource_manager.non_fungible_exists(&NonFungibleLocalId::integer(1)),
                false
            );
            (mint_badge, bucket)
        }

        pub fn take_and_put_bucket() -> Bucket {
            let mut bucket = Self::create_non_fungible_fixed();
            assert_eq!(bucket.amount(), 3.into());

            let non_fungible = bucket.take(1);
            assert_eq!(bucket.amount(), 2.into());
            assert_eq!(non_fungible.amount(), 1.into());

            bucket.put(non_fungible);
            bucket
        }

        pub fn take_non_fungible_and_put_bucket() -> Bucket {
            let mut bucket = Self::create_non_fungible_fixed().as_non_fungible();
            assert_eq!(bucket.amount(), 3.into());

            let non_fungible = bucket.take_non_fungible(&NonFungibleLocalId::integer(1));
            assert_eq!(bucket.amount(), 2.into());
            assert_eq!(non_fungible.amount(), 1.into());

            bucket.put(non_fungible);
            bucket.into()
        }

        pub fn take_non_fungibles_and_put_bucket() -> Bucket {
            let mut bucket = Self::create_non_fungible_fixed();
            assert_eq!(bucket.amount(), 3.into());

            let mut non_fungibles = BTreeSet::new();
            non_fungibles.insert(NonFungibleLocalId::integer(1));

            let non_fungible = bucket.as_non_fungible().take_non_fungibles(&non_fungibles);
            assert_eq!(bucket.amount(), 2.into());
            assert_eq!(non_fungible.amount(), 1.into());

            bucket.put(non_fungible.into());
            bucket
        }

        pub fn take_and_put_vault() -> Bucket {
            let mut vault = Vault::with_bucket(Self::create_non_fungible_fixed());
            assert_eq!(vault.amount(), 3.into());

            let non_fungible = vault.take(1);
            assert_eq!(vault.amount(), 2.into());
            assert_eq!(non_fungible.amount(), 1.into());

            NonFungibleTest { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();

            non_fungible
        }

        pub fn get_non_fungible_local_ids_bucket() -> (Bucket, Bucket) {
            let mut bucket = Self::create_non_fungible_fixed();
            let non_fungible_bucket = bucket.take(1);
            assert_eq!(
                non_fungible_bucket
                    .as_non_fungible()
                    .non_fungible_local_ids(),
                BTreeSet::from([NonFungibleLocalId::integer(1)])
            );
            assert_eq!(
                bucket.as_non_fungible().non_fungible_local_ids(),
                BTreeSet::from([
                    NonFungibleLocalId::integer(2),
                    NonFungibleLocalId::integer(3)
                ])
            );
            (bucket, non_fungible_bucket)
        }

        pub fn get_non_fungible_local_id_bucket() -> (Bucket, Bucket) {
            let mut bucket = Self::create_non_fungible_fixed();
            let non_fungible_bucket = bucket.take(1);
            assert_eq!(
                non_fungible_bucket
                    .as_non_fungible()
                    .non_fungible_local_id(),
                NonFungibleLocalId::integer(1)
            );
            assert_eq!(
                bucket.as_non_fungible().non_fungible_local_id(),
                NonFungibleLocalId::integer(2)
            );
            (bucket, non_fungible_bucket)
        }

        pub fn get_non_fungible_local_ids_vault() -> Bucket {
            let mut vault = Vault::with_bucket(Self::create_non_fungible_fixed());
            let non_fungible_bucket = vault.take(1);
            assert_eq!(
                non_fungible_bucket
                    .as_non_fungible()
                    .non_fungible_local_ids(),
                BTreeSet::from([NonFungibleLocalId::integer(1)])
            );
            assert_eq!(
                vault.as_non_fungible().non_fungible_local_ids(100),
                BTreeSet::from([
                    NonFungibleLocalId::integer(2),
                    NonFungibleLocalId::integer(3)
                ])
            );

            NonFungibleTest { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();

            non_fungible_bucket
        }

        pub fn contains_non_fungible_vault() {
            let vault = Vault::with_bucket(Self::create_non_fungible_fixed());
            let vault = vault.as_non_fungible();
            assert!(vault.contains_non_fungible(&NonFungibleLocalId::integer(1)));
            assert!(vault.contains_non_fungible(&NonFungibleLocalId::integer(2)));
            assert!(vault.contains_non_fungible(&NonFungibleLocalId::integer(3)));
            assert!(!vault.contains_non_fungible(&NonFungibleLocalId::integer(4)));

            NonFungibleTest {
                vault: vault.into(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();
        }

        pub fn contains_non_fungible_bucket() {
            let bucket = Self::create_non_fungible_fixed();
            let bucket = bucket.as_non_fungible();
            assert!(bucket.contains_non_fungible(&NonFungibleLocalId::integer(1)));
            assert!(bucket.contains_non_fungible(&NonFungibleLocalId::integer(2)));
            assert!(bucket.contains_non_fungible(&NonFungibleLocalId::integer(3)));
            assert!(!bucket.contains_non_fungible(&NonFungibleLocalId::integer(4)));

            NonFungibleTest {
                vault: Vault::with_bucket(bucket.into()),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize();
        }

        pub fn get_non_fungible_local_id_vault() -> Bucket {
            let mut vault = Vault::with_bucket(Self::create_non_fungible_fixed());
            let non_fungible_bucket = vault.take(1);
            assert_eq!(
                non_fungible_bucket
                    .as_non_fungible()
                    .non_fungible_local_id(),
                NonFungibleLocalId::integer(1)
            );
            assert_eq!(
                vault.as_non_fungible().non_fungible_local_id(),
                NonFungibleLocalId::integer(2)
            );

            NonFungibleTest { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();

            non_fungible_bucket
        }

        pub fn singleton_non_fungible() {
            let mut bucket = Self::create_non_fungible_fixed();
            assert_eq!(bucket.amount(), 3.into());

            // read singleton bucket
            let singleton = bucket.take(1);
            let _: Sandwich = singleton.as_non_fungible().non_fungible().data();

            // read singleton vault
            let mut vault = Vault::with_bucket(singleton);
            let _: Sandwich = vault.as_non_fungible().non_fungible().data();

            // read singleton proof
            let proof = vault
                .as_non_fungible()
                .create_proof_of_non_fungibles(&btreeset!(NonFungibleLocalId::integer(1)))
                .skip_checking();
            assert_eq!(proof.resource_address(), vault.resource_address());
            let _: Sandwich = proof.as_non_fungible().non_fungible().data();
            proof.drop();

            // clean up
            vault.put(bucket);
            NonFungibleTest { vault }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
        }

        pub fn create_wrong_non_fungible_local_id_type() -> Bucket {
            let mut entries = BTreeMap::new();
            entries.insert(
                NonFungibleLocalId::integer(0),
                (scrypto_decode(&scrypto_encode(&()).unwrap()).unwrap(),),
            );

            // creating non-fungible id with id type set to default (RUID)
            let rtn = ScryptoEnv
                .call_function(
                    RESOURCE_PACKAGE,
                    NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT,
                    NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT,
                    scrypto_encode(&NonFungibleResourceManagerCreateWithInitialSupplyInput {
                        owner_role: OwnerRole::None,
                        id_type: NonFungibleIdType::RUID,
                        track_total_supply: false,
                        resource_roles: NonFungibleResourceRoles::default(),
                        metadata: metadata! {},
                        non_fungible_schema: NonFungibleDataSchema::new_schema::<()>(),
                        entries,
                        address_reservation: None,
                    })
                    .unwrap(),
                )
                .unwrap();
            let (_resource_address, bucket): (ResourceAddress, Bucket) =
                scrypto_decode(&rtn).unwrap();

            bucket
        }

        pub fn create_string_non_fungible() -> Bucket {
            // creating non-fungible id with id type set to default (RUID)
            ResourceBuilder::new_string_non_fungible::<Sandwich>(OwnerRole::None)
                .mint_initial_supply([
                    (
                        "1".try_into().unwrap(),
                        Sandwich {
                            name: "One".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                    (
                        "2".try_into().unwrap(),
                        Sandwich {
                            name: "Two".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                ])
                .into()
        }

        pub fn create_bytes_non_fungible() -> Bucket {
            ResourceBuilder::new_bytes_non_fungible::<Sandwich>(OwnerRole::None)
                .mint_initial_supply([
                    (
                        1u32.to_le_bytes().to_vec().try_into().unwrap(),
                        Sandwich {
                            name: "One".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                    (
                        2u32.to_le_bytes().to_vec().try_into().unwrap(),
                        Sandwich {
                            name: "Two".to_owned(),
                            available: true,
                            tastes_great: true,
                            reference: None,
                            own: None,
                        },
                    ),
                ])
                .into()
        }

        pub fn create_ruid_non_fungible() -> Bucket {
            ResourceBuilder::new_ruid_non_fungible::<Sandwich>(OwnerRole::None)
                .mint_initial_supply([Sandwich {
                    name: "Zero".to_owned(),
                    available: true,
                    tastes_great: true,
                    reference: None,
                    own: None,
                }])
                .into()
        }

        pub fn create_mintable_ruid_non_fungible() -> ResourceManager {
            ResourceBuilder::new_ruid_non_fungible::<Sandwich>(OwnerRole::None)
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply()
        }

        pub fn create_ruid_non_fungible_and_mint() -> Bucket {
            // creating non-fungible id with id type set to default (RUID)
            let resource_manager =
                ResourceBuilder::new_ruid_non_fungible::<Sandwich>(OwnerRole::None)
                    .mint_roles(mint_roles! {
                        minter => rule!(allow_all);
                        minter_updater => rule!(deny_all);
                    })
                    .metadata(metadata! {
                        init {
                            "name" => "Katz's Sandwiches".to_owned(), locked;
                        }
                    })
                    .create_with_no_initial_supply();

            resource_manager.mint_ruid_non_fungible(Sandwich {
                name: "Test".to_owned(),
                available: false,
                tastes_great: true,
                reference: None,
                own: None,
            })
        }
    }
}
