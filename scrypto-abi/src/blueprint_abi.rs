#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::schema_type::Type;
use sbor::{Decode, Encode, TypeId};

/// Represents a blueprint.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, TypeId, Encode, Decode)]
pub struct Blueprint {
    pub package_address: String,
    pub blueprint_name: String,
    pub abi: BlueprintAbi,
}

/// Represents the ABI of a blueprint.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq)]
pub struct BlueprintAbi {
    pub structure: Type,
    pub fns: Vec<Fn>,
}

impl BlueprintAbi {
    pub fn get_fn_abi(&self, fn_ident: &str) -> Option<&Fn> {
        for func in &self.fns {
            if func.ident.eq(fn_ident) {
                return Option::Some(func);
            }
        }
        Option::None
    }

    pub fn contains_fn(&self, fn_ident: &str) -> bool {
        self.get_fn_abi(fn_ident).is_some()
    }
}

/// Represents a method/function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq)]
pub struct Fn {
    pub ident: String,
    pub mutability: Option<SelfMutability>,
    pub input: Type,
    pub output: Type,
    pub export_name: String,
}

/// Whether a method is going to change the component state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq)]
pub enum SelfMutability {
    /// An immutable method requires an immutable reference to component state.
    Immutable,

    /// A mutable method requires a mutable reference to component state.
    Mutable,
}
