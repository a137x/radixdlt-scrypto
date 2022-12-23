use sbor::rust::fmt;
use sbor::rust::string::String;
use sbor::rust::vec::Vec;
use sbor::*;
use utils::{copy_u8_array, ContextualDisplay};

use crate::abi::*;
use crate::address::*;
use crate::data::ScryptoCustomTypeId;
use crate::scrypto_type;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SystemAddress {
    EpochManager([u8; 26]),
    Clock([u8; 26]),
}

//========
// binary
//========

impl TryFrom<&[u8]> for SystemAddress {
    type Error = AddressError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.len() {
            27 => match EntityType::try_from(slice[0])
                .map_err(|_| AddressError::InvalidEntityTypeId(slice[0]))?
            {
                EntityType::EpochManager => Ok(Self::EpochManager(copy_u8_array(&slice[1..]))),
                EntityType::Clock => Ok(Self::Clock(copy_u8_array(&slice[1..]))),
                _ => Err(AddressError::InvalidEntityTypeId(slice[0])),
            },
            _ => Err(AddressError::InvalidLength(slice.len())),
        }
    }
}

impl SystemAddress {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(EntityType::system(self).id());
        match self {
            Self::EpochManager(v) => buf.extend(v),
            Self::Clock(v) => buf.extend(v),
        }
        buf
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_vec())
    }

    pub fn try_from_hex(hex_str: &str) -> Result<Self, AddressError> {
        let bytes = hex::decode(hex_str).map_err(|_| AddressError::HexDecodingError)?;

        Self::try_from(bytes.as_ref())
    }
}

scrypto_type!(
    SystemAddress,
    ScryptoCustomTypeId::SystemAddress,
    Type::SystemAddress,
    27
);

//======
// text
//======

impl fmt::Debug for SystemAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.display(NO_NETWORK))
    }
}

impl<'a> ContextualDisplay<AddressDisplayContext<'a>> for SystemAddress {
    type Error = AddressError;

    fn contextual_format<F: fmt::Write>(
        &self,
        f: &mut F,
        context: &AddressDisplayContext<'a>,
    ) -> Result<(), Self::Error> {
        if let Some(encoder) = context.encoder {
            return encoder.encode_system_address_to_fmt(f, self);
        }

        // This could be made more performant by streaming the hex into the formatter
        match self {
            SystemAddress::EpochManager(_) => write!(f, "EpochManagerSystem[{}]", self.to_hex()),
            SystemAddress::Clock(_) => write!(f, "ClockSystem[{}]", self.to_hex()),
        }
        .map_err(|err| AddressError::FormatError(err))
    }
}
