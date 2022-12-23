use crate::model::*;

/// A unique identifier used in the addressing of Resource Addresses.
pub const RESOURCE_ADDRESS_ENTITY_ID: u8 = 0x00;

/// A unique identifier used in the addressing of Package Addresses.
pub const PACKAGE_ADDRESS_ENTITY_ID: u8 = 0x01;

/// A unique identifier used in the addressing of Generic Component Addresses.
pub const NORMAL_COMPONENT_ADDRESS_ENTITY_ID: u8 = 0x02;

/// A unique identifier used in the addressing of Account Component Addresses.
pub const ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID: u8 = 0x03;

/// A unique identifier used in the addressing of Epoch Manager System Addresses.
pub const EPOCH_MANAGER_SYSTEM_ADDRESS_ENTITY_ID: u8 = 0x04;

/// A unique identifier used in the addressing of Clock System Addresses.
pub const CLOCK_SYSTEM_ADDRESS_ENTITY_ID: u8 = 0x05;

/// A unique identifier used in the addressing of a virtual Account Component Addresses.
pub const ECDSA_SECP_256K1_VIRTUAL_ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID: u8 = 0x06;

/// A unique identifier used in the addressing of a virtual Account Component Addresses.
pub const EDDSA_ED_25519_VIRTUAL_ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID: u8 = 0x07;

/// An enum which represents the different addressable entities.
#[derive(PartialEq, Eq)]
pub enum EntityType {
    Resource,
    Package,
    NormalComponent,
    AccountComponent,
    EcdsaSecp256k1VirtualAccountComponent,
    EddsaEd25519VirtualAccountComponent,
    EpochManager,
    Clock,
}

impl EntityType {
    pub fn package(_address: &PackageAddress) -> Self {
        Self::Package
    }
    pub fn resource(_address: &ResourceAddress) -> Self {
        Self::Resource
    }
    pub fn component(address: &ComponentAddress) -> Self {
        match address {
            ComponentAddress::Normal(_) => Self::NormalComponent,
            ComponentAddress::Account(_) => Self::AccountComponent,
            ComponentAddress::EcdsaSecp256k1VirtualAccount(_) => {
                Self::EcdsaSecp256k1VirtualAccountComponent
            }
            ComponentAddress::EddsaEd25519VirtualAccount(_) => {
                Self::EddsaEd25519VirtualAccountComponent
            }
        }
    }

    pub fn system(address: &SystemAddress) -> Self {
        match address {
            SystemAddress::EpochManager(_) => Self::EpochManager,
            SystemAddress::Clock(_) => Self::Clock,
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            Self::Resource => RESOURCE_ADDRESS_ENTITY_ID,
            Self::Package => PACKAGE_ADDRESS_ENTITY_ID,
            Self::NormalComponent => NORMAL_COMPONENT_ADDRESS_ENTITY_ID,
            Self::AccountComponent => ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID,
            Self::EcdsaSecp256k1VirtualAccountComponent => {
                ECDSA_SECP_256K1_VIRTUAL_ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID
            }
            Self::EddsaEd25519VirtualAccountComponent => {
                EDDSA_ED_25519_VIRTUAL_ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID
            }
            Self::EpochManager => EPOCH_MANAGER_SYSTEM_ADDRESS_ENTITY_ID,
            Self::Clock => CLOCK_SYSTEM_ADDRESS_ENTITY_ID,
        }
    }
}

impl TryFrom<u8> for EntityType {
    type Error = EntityTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            RESOURCE_ADDRESS_ENTITY_ID => Ok(Self::Resource),
            PACKAGE_ADDRESS_ENTITY_ID => Ok(Self::Package),
            NORMAL_COMPONENT_ADDRESS_ENTITY_ID => Ok(Self::NormalComponent),
            ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID => Ok(Self::AccountComponent),
            ECDSA_SECP_256K1_VIRTUAL_ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID => {
                Ok(Self::EcdsaSecp256k1VirtualAccountComponent)
            }
            EDDSA_ED_25519_VIRTUAL_ACCOUNT_COMPONENT_ADDRESS_ENTITY_ID => {
                Ok(Self::EddsaEd25519VirtualAccountComponent)
            }
            EPOCH_MANAGER_SYSTEM_ADDRESS_ENTITY_ID => Ok(Self::EpochManager),
            CLOCK_SYSTEM_ADDRESS_ENTITY_ID => Ok(Self::Clock),
            _ => Err(EntityTypeError::InvalidEntityTypeId(value)),
        }
    }
}

pub enum EntityTypeError {
    InvalidEntityTypeId(u8),
}
