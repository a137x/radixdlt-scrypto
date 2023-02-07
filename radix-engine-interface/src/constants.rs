use crate::address;
use crate::api::component::ComponentAddress;
use crate::api::package::PackageAddress;
use crate::blueprints::resource::ResourceAddress;
use crate::construct_address;

/// The XRD resource address.
pub const RADIX_TOKEN: ResourceAddress = address!(EntityType::Resource, 0);

/// The ECDSA virtual resource address.
pub const ECDSA_SECP256K1_TOKEN: ResourceAddress = address!(EntityType::Resource, 1);

/// The ED25519 virtual resource address.
pub const EDDSA_ED25519_TOKEN: ResourceAddress = address!(EntityType::Resource, 2);

/// The system token which allows access to system resources (e.g. setting epoch)
pub const SYSTEM_TOKEN: ResourceAddress = address!(EntityType::Resource, 3);

pub const PACKAGE_TOKEN: ResourceAddress = address!(EntityType::Resource, 4);

pub const OLYMPIA_VALIDATOR_TOKEN: ResourceAddress = address!(EntityType::Resource, 5);

/// The address of the faucet package.
pub const IDENTITY_PACKAGE: PackageAddress = address!(EntityType::Package, 0);
pub const IDENTITY_BLUEPRINT: &str = "Identity";

pub const EPOCH_MANAGER_PACKAGE: PackageAddress = address!(EntityType::Package, 1);
pub const EPOCH_MANAGER_BLUEPRINT: &str = "EpochManager";

pub const CLOCK_PACKAGE: PackageAddress = address!(EntityType::Package, 2);
pub const CLOCK_BLUEPRINT: &str = "Clock";

pub const FAUCET_PACKAGE: PackageAddress = address!(EntityType::Package, 3);
pub const FAUCET_BLUEPRINT: &str = "Faucet";

/// The address of the faucet component, test network only.
pub const FAUCET_COMPONENT: ComponentAddress = construct_address!(
    EntityType::NormalComponent,
    51,
    112,
    129,
    183,
    184,
    244,
    163,
    95,
    218,
    117,
    244,
    128,
    134,
    100,
    153,
    207,
    215,
    243,
    188,
    209,
    242,
    31,
    200,
    35,
    100,
    163
);

pub const CLOCK: ComponentAddress = address!(EntityType::Clock, 0);
pub const EPOCH_MANAGER: ComponentAddress = address!(EntityType::EpochManager, 0);
