use super::HasTypeInfo;
use crate::prelude::{Global, HasStub, ObjectStub, ObjectStubHandle};
use native_blueprints_interface::package::{
    PackageClaimRoyaltiesInput, PACKAGE_BLUEPRINT, PACKAGE_CLAIM_ROYALTIES_IDENT,
};
use native_blueprints_interface::resource::Bucket;
use radix_engine_common::prelude::PACKAGE_PACKAGE;
use radix_engine_common::prelude::*;
use radix_engine_common::*;

pub type Package = Global<PackageStub>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PackageStub(pub ObjectStubHandle);

impl HasStub for PackageStub {
    type Stub = Self;
}

impl HasTypeInfo for PackageStub {
    const PACKAGE_ADDRESS: Option<PackageAddress> = Some(PACKAGE_PACKAGE);

    const BLUEPRINT_NAME: &'static str = PACKAGE_BLUEPRINT;

    const OWNED_TYPE_NAME: &'static str = "OwnedPackage";

    const GLOBAL_TYPE_NAME: &'static str = "GlobalPackage";
}

impl ObjectStub for PackageStub {
    type AddressType = PackageAddress;

    fn new(handle: ObjectStubHandle) -> Self {
        Self(handle)
    }

    fn handle(&self) -> &ObjectStubHandle {
        &self.0
    }
}

impl PackageStub {
    pub fn claim_royalties(&self) -> Bucket {
        self.call(
            PACKAGE_CLAIM_ROYALTIES_IDENT,
            &PackageClaimRoyaltiesInput {},
        )
    }
}

impl From<PackageAddress> for Package {
    fn from(value: PackageAddress) -> Self {
        Global(ObjectStub::new(ObjectStubHandle::Global(value.into())))
    }
}
