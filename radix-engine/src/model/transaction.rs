use sbor::*;
use scrypto::rust::string::String;
use scrypto::rust::vec::Vec;
use scrypto::types::*;

/// Represents an unvalidated transaction.
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq)]
pub struct Transaction {
    pub instructions: Vec<Instruction>,
}

/// Represents an unvalidated instruction in transaction
#[derive(Debug, Clone, TypeId, Encode, Decode, PartialEq, Eq)]
pub enum Instruction {
    /// Takes resource from worktop and return a bucket.
    TakeFromWorktop {
        amount: Decimal,
        resource_address: Address,
    },

    /// Takes all resource from worktop and return a bucket.
    TakeAllFromWorktop { resource_address: Address },

    /// Returns resource to worktop.
    ReturnToWorktop { bid: Bid },

    /// Asserts worktop contains at least this amount.
    AssertWorktopContains {
        amount: Decimal,
        resource_address: Address,
    },

    /// Creates a temporary bucket ref.
    CreateBucketRef { bid: Bid },

    /// Clones a temporary bucket ref.
    CloneBucketRef { rid: Rid },

    /// Drops a temporary bucket ref.
    DropBucketRef { rid: Rid },

    /// Calls a blueprint function.
    ///
    /// Buckets and bucket refs in arguments moves from transaction context to the callee.
    CallFunction {
        package_address: Address,
        blueprint_name: String,
        function: String,
        args: Vec<Vec<u8>>,
    },

    /// Calls a component method.
    ///
    /// Buckets and bucket refs in arguments moves from transaction context to the callee.
    CallMethod {
        component_address: Address,
        method: String,
        args: Vec<Vec<u8>>,
    },

    /// With method with all resources from transaction context.
    CallMethodWithAllResources {
        component_address: Address,
        method: String,
    },

    /// Marks the end of transaction with signatures.
    /// TODO: replace public key address with signature.
    End { signatures: Vec<Address> },
}
