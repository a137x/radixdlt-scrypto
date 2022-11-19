use crate::resource::{ComponentAuthZone, NonFungible, ScryptoProof};
use radix_engine_interface::api::api::SysNativeInvokable;
use radix_engine_interface::data::{ScryptoDecode, ScryptoTypeId};
use radix_engine_interface::math::Decimal;
use radix_engine_interface::model::*;
use sbor::rust::collections::BTreeSet;
use sbor::rust::fmt::Debug;
use sbor::rust::vec::Vec;
use scrypto::engine::scrypto_env::ScryptoEnv;
use scrypto::scrypto_env_native_fn;

pub trait SysBucket {
    fn sys_new<Y, E: Debug + ScryptoTypeId + ScryptoDecode>(
        receiver: ResourceAddress,
        sys_calls: &mut Y,
    ) -> Result<Bucket, E>
    where
        Y: SysNativeInvokable<ResourceManagerCreateBucketInvocation, E>;

    fn sys_burn<Y, E: Debug + ScryptoTypeId + ScryptoDecode>(self, env: &mut Y) -> Result<(), E>
    where
        Y: SysNativeInvokable<ResourceManagerBurnInvocation, E>
            + SysNativeInvokable<BucketGetResourceAddressInvocation, E>;

    fn sys_resource_address<Y, E>(&self, env: &mut Y) -> Result<ResourceAddress, E>
    where
        Y: SysNativeInvokable<BucketGetResourceAddressInvocation, E>,
        E: Debug + ScryptoTypeId + ScryptoDecode;

    fn sys_create_proof<Y, E: Debug + ScryptoTypeId + ScryptoDecode>(
        &self,
        sys_calls: &mut Y,
    ) -> Result<Proof, E>
    where
        Y: SysNativeInvokable<BucketCreateProofInvocation, E>;
}

impl SysBucket for Bucket {
    fn sys_new<Y, E: Debug + ScryptoTypeId + ScryptoDecode>(
        receiver: ResourceAddress,
        sys_calls: &mut Y,
    ) -> Result<Bucket, E>
    where
        Y: SysNativeInvokable<ResourceManagerCreateBucketInvocation, E>,
    {
        sys_calls.sys_invoke(ResourceManagerCreateBucketInvocation { receiver })
    }

    fn sys_burn<Y, E: Debug + ScryptoTypeId + ScryptoDecode>(self, env: &mut Y) -> Result<(), E>
    where
        Y: SysNativeInvokable<ResourceManagerBurnInvocation, E>
            + SysNativeInvokable<BucketGetResourceAddressInvocation, E>,
    {
        let receiver = self.sys_resource_address(env)?;
        env.sys_invoke(ResourceManagerBurnInvocation {
            receiver,
            bucket: Bucket(self.0),
        })
    }

    fn sys_resource_address<Y, E>(&self, env: &mut Y) -> Result<ResourceAddress, E>
    where
        Y: SysNativeInvokable<BucketGetResourceAddressInvocation, E>,
        E: Debug + ScryptoTypeId + ScryptoDecode,
    {
        env.sys_invoke(BucketGetResourceAddressInvocation { receiver: self.0 })
    }

    fn sys_create_proof<Y, E: Debug + ScryptoTypeId + ScryptoDecode>(
        &self,
        sys_calls: &mut Y,
    ) -> Result<Proof, E>
    where
        Y: SysNativeInvokable<BucketCreateProofInvocation, E>,
    {
        sys_calls.sys_invoke(BucketCreateProofInvocation { receiver: self.0 })
    }
}

pub trait ScryptoBucket {
    fn new(resource_address: ResourceAddress) -> Self;
    fn burn(self);
    fn create_proof(&self) -> Proof;
    fn resource_address(&self) -> ResourceAddress;
    fn take_internal(&mut self, amount: Decimal) -> Bucket;
    fn take_non_fungibles(&mut self, non_fungible_ids: &BTreeSet<NonFungibleId>) -> Bucket;
    fn put(&mut self, other: Self) -> ();
    fn non_fungible_ids(&self) -> BTreeSet<NonFungibleId>;
    fn amount(&self) -> Decimal;
    fn take<A: Into<Decimal>>(&mut self, amount: A) -> Self;
    fn take_non_fungible(&mut self, non_fungible_id: &NonFungibleId) -> Self;
    fn is_empty(&self) -> bool;
    fn authorize<F: FnOnce() -> O, O>(&self, f: F) -> O;
    fn non_fungibles<T: NonFungibleData>(&self) -> Vec<NonFungible<T>>;
    fn non_fungible_id(&self) -> NonFungibleId;
    fn non_fungible<T: NonFungibleData>(&self) -> NonFungible<T>;
}

impl ScryptoBucket for Bucket {
    fn new(resource_address: ResourceAddress) -> Self {
        Self::sys_new(resource_address, &mut ScryptoEnv).unwrap()
    }

    fn burn(self) {
        self.sys_burn(&mut ScryptoEnv).unwrap()
    }

    fn create_proof(&self) -> Proof {
        self.sys_create_proof(&mut ScryptoEnv).unwrap()
    }

    fn resource_address(&self) -> ResourceAddress {
        self.sys_resource_address(&mut ScryptoEnv).unwrap()
    }

    scrypto_env_native_fn! {
        fn take_internal(&mut self, amount: Decimal) -> Bucket {
            BucketTakeInvocation {
                receiver: self.0,
                amount,
            }
        }

        fn take_non_fungibles(&mut self, non_fungible_ids: &BTreeSet<NonFungibleId>) -> Bucket {
            BucketTakeNonFungiblesInvocation {
                receiver: self.0,
                ids: non_fungible_ids.clone()
            }
        }

        fn put(&mut self, other: Self) -> () {
            BucketPutInvocation {
                receiver: self.0,
                bucket: Bucket(other.0),
            }
        }

        fn non_fungible_ids(&self) -> BTreeSet<NonFungibleId> {
            BucketGetNonFungibleIdsInvocation {
                receiver: self.0,
            }
        }

        fn amount(&self) -> Decimal {
            BucketGetAmountInvocation {
                receiver: self.0,
            }
        }
    }

    /// Takes some amount of resources from this bucket.
    fn take<A: Into<Decimal>>(&mut self, amount: A) -> Self {
        self.take_internal(amount.into())
    }

    /// Takes a specific non-fungible from this bucket.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible bucket or the specified non-fungible resource is not found.
    fn take_non_fungible(&mut self, non_fungible_id: &NonFungibleId) -> Self {
        self.take_non_fungibles(&BTreeSet::from([non_fungible_id.clone()]))
    }

    /// Uses resources in this bucket as authorization for an operation.
    fn authorize<F: FnOnce() -> O, O>(&self, f: F) -> O {
        ComponentAuthZone::push(self.create_proof());
        let output = f();
        ComponentAuthZone::pop().drop();
        output
    }

    /// Checks if this bucket is empty.
    fn is_empty(&self) -> bool {
        self.amount() == 0.into()
    }

    /// Returns all the non-fungible units contained.
    ///
    /// # Panics
    /// Panics if this is not a non-fungible bucket.
    fn non_fungibles<T: NonFungibleData>(&self) -> Vec<NonFungible<T>> {
        let resource_address = self.resource_address();
        self.non_fungible_ids()
            .iter()
            .map(|id| NonFungible::from(NonFungibleAddress::new(resource_address, id.clone())))
            .collect()
    }

    /// Returns a singleton non-fungible id
    ///
    /// # Panics
    /// Panics if this is not a singleton bucket
    fn non_fungible_id(&self) -> NonFungibleId {
        let non_fungible_ids = self.non_fungible_ids();
        if non_fungible_ids.len() != 1 {
            panic!("Expecting singleton NFT vault");
        }
        self.non_fungible_ids().into_iter().next().unwrap()
    }

    /// Returns a singleton non-fungible.
    ///
    /// # Panics
    /// Panics if this is not a singleton bucket
    fn non_fungible<T: NonFungibleData>(&self) -> NonFungible<T> {
        let non_fungibles = self.non_fungibles();
        if non_fungibles.len() != 1 {
            panic!("Expecting singleton NFT bucket");
        }
        non_fungibles.into_iter().next().unwrap()
    }
}
