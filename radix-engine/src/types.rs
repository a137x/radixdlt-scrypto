pub use radix_engine_interface::abi::{BlueprintAbi, Fields, Fn, Type, Variant};
pub use radix_engine_interface::address::{AddressError, Bech32Decoder, Bech32Encoder};
pub use radix_engine_interface::api::types::*;
pub use radix_engine_interface::constants::*;
pub use radix_engine_interface::crypto::*;
pub use radix_engine_interface::data::types::*;
pub use radix_engine_interface::data::{
    scrypto_decode, scrypto_encode, IndexedScryptoValue, ScryptoCategorize, ScryptoDecode,
    ScryptoEncode,
};
pub use radix_engine_interface::dec;
pub use radix_engine_interface::math::{BnumI256, Decimal, RoundingMode};
pub use radix_engine_interface::*;
pub use sbor::rust::borrow::ToOwned;
pub use sbor::rust::boxed::Box;
pub use sbor::rust::cell::{Ref, RefCell, RefMut};
pub use sbor::rust::collections::*;
pub use sbor::rust::fmt;
pub use sbor::rust::fmt::Debug;
pub use sbor::rust::format;
pub use sbor::rust::marker::PhantomData;
pub use sbor::rust::num::NonZeroU32;
pub use sbor::rust::num::NonZeroUsize;
pub use sbor::rust::ops::AddAssign;
pub use sbor::rust::ptr;
pub use sbor::rust::rc::Rc;
pub use sbor::rust::str::FromStr;
pub use sbor::rust::string::String;
pub use sbor::rust::string::ToString;
pub use sbor::rust::sync;
pub use sbor::rust::vec;
pub use sbor::rust::vec::Vec;
pub use sbor::{Categorize, Decode, DecodeError, Encode, SborPath, SborPathBuf, Value, ValueKind};
#[cfg(feature = "std")]
pub use std::alloc;

use bitflags::bitflags;

bitflags! {
    #[derive(Encode, Decode, Categorize)]
    pub struct LockFlags: u32 {
        /// Allows the locked substate to be mutated
        const MUTABLE = 0b00000001;
        /// Checks that the substate locked is unmodified from the beginning of
        /// the transaction. This is used mainly for locking fees in vaults which
        /// requires this in order to be able to support rollbacks
        const UNMODIFIED_BASE = 0b00000010;
        /// Forces a write of a substate even on a transaction failure
        /// Currently used for vault fees.
        const FORCE_WRITE = 0b00000100;
    }
}

impl LockFlags {
    pub fn read_only() -> Self {
        LockFlags::empty()
    }
}

pub struct LockInfo {
    pub offset: SubstateOffset,
}
