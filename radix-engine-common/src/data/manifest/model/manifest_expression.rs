#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;
use sbor::rust::convert::TryFrom;
#[cfg(not(feature = "alloc"))]
use sbor::rust::fmt;
use sbor::rust::vec::Vec;
use sbor::*;

use crate::data::manifest::*;
use crate::*;

#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestExpression {
    EntireWorktop,
    EntireAuthZone,
    Owned(u8),
}

//========
// error
//========

/// Represents an error when parsing ManifestExpression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseManifestExpressionError {
    InvalidLength,
    UnknownExpression,
}

#[cfg(not(feature = "alloc"))]
impl std::error::Error for ParseManifestExpressionError {}

#[cfg(not(feature = "alloc"))]
impl fmt::Display for ParseManifestExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//========
// binary
//========

impl TryFrom<&[u8]> for ManifestExpression {
    type Error = ParseManifestExpressionError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match (slice.get(0), slice.len()) {
            (Some(0), 1) => Ok(Self::EntireWorktop),
            (Some(1), 1) => Ok(Self::EntireAuthZone),
            (Some(2), 2) => Ok(Self::Owned(slice[1])),
            _ => Err(Self::Error::InvalidLength),
        }
    }
}

impl ManifestExpression {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            ManifestExpression::EntireWorktop => {
                bytes.push(0);
            }
            ManifestExpression::EntireAuthZone => {
                bytes.push(1);
            }
            ManifestExpression::Owned(i) => {
                bytes.push(2);
                bytes.push(*i);
            }
        };
        bytes
    }
}

manifest_type!(ManifestExpression, ManifestCustomValueKind::Expression, 1);
