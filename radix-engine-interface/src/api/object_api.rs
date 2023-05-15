use crate::api::node_modules::auth::ACCESS_RULES_BLUEPRINT;
use crate::api::node_modules::metadata::METADATA_BLUEPRINT;
use crate::constants::{
    ACCESS_RULES_MODULE_PACKAGE, METADATA_MODULE_PACKAGE, ROYALTY_MODULE_PACKAGE,
};
use crate::types::*;
#[cfg(feature = "radix_engine_fuzzing")]
use arbitrary::Arbitrary;
use radix_engine_common::types::*;
use radix_engine_derive::{ManifestSbor, ScryptoSbor};
use radix_engine_interface::api::node_modules::royalty::COMPONENT_ROYALTY_BLUEPRINT;
use sbor::rust::collections::*;
use sbor::rust::prelude::*;
use sbor::rust::vec::Vec;
use scrypto_schema::InstanceSchema;

#[repr(u8)]
#[cfg_attr(feature = "radix_engine_fuzzing", derive(Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ScryptoSbor,
    ManifestSbor,
    FromRepr,
    EnumIter,
)]
pub enum ObjectModuleId {
    Main,
    Metadata,
    Royalty,
    AccessRules,
}

impl ObjectModuleId {
    pub fn base_partition_num(&self) -> PartitionNumber {
        match self {
            ObjectModuleId::Metadata => METADATA_KV_STORE_PARTITION,
            ObjectModuleId::Royalty => ROYALTY_FIELD_PARTITION,
            ObjectModuleId::AccessRules => ACCESS_RULES_FIELD_PARTITION,
            ObjectModuleId::Main => OBJECT_BASE_PARTITION,
        }
    }

    pub fn static_blueprint(&self) -> Option<Blueprint> {
        match self {
            ObjectModuleId::Metadata => {
                Some(Blueprint::new(&METADATA_MODULE_PACKAGE, METADATA_BLUEPRINT))
            }
            ObjectModuleId::Royalty => Some(Blueprint::new(
                &ROYALTY_MODULE_PACKAGE,
                COMPONENT_ROYALTY_BLUEPRINT,
            )),
            ObjectModuleId::AccessRules => Some(Blueprint::new(
                &ACCESS_RULES_MODULE_PACKAGE,
                ACCESS_RULES_BLUEPRINT,
            )),
            ObjectModuleId::Main => None,
        }
    }
}

/// A high level interface to manipulate objects in the actor's call frame
pub trait ClientObjectApi<E> {
    /// Creates a new simple blueprint object of a given blueprint type
    fn new_simple_object(
        &mut self,
        blueprint_ident: &str,
        fields: Vec<Vec<u8>>,
    ) -> Result<NodeId, E> {
        self.new_object(blueprint_ident, None, fields, btreemap![])
    }

    /// Creates a new object of a given blueprint type
    fn new_object(
        &mut self,
        blueprint_ident: &str,
        schema: Option<InstanceSchema>,
        fields: Vec<Vec<u8>>,
        kv_entries: BTreeMap<u8, BTreeMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<NodeId, E>;

    /// Drops an object, returns the fields of the object
    fn drop_object(&mut self, node_id: &NodeId) -> Result<Vec<Vec<u8>>, E>;

    /// Get info regarding a visible object
    fn get_object_info(&mut self, node_id: &NodeId) -> Result<ObjectInfo, E>;

    /// Pre-allocates a global address, for a future globalization.
    fn preallocate_global_address(&mut self) -> Result<GlobalAddress, E>;

    /// Moves an object currently in the heap into the global space making
    /// it accessible to all. A global address is automatically created and returned.
    fn globalize(&mut self, modules: BTreeMap<ObjectModuleId, NodeId>) -> Result<GlobalAddress, E>;

    /// Moves an object currently in the heap into the global space making
    /// it accessible to all with the provided global address.
    fn globalize_with_address(
        &mut self,
        modules: BTreeMap<ObjectModuleId, NodeId>,
        address: GlobalAddress,
    ) -> Result<(), E>;

    fn globalize_with_address_and_create_inner_object(
        &mut self,
        modules: BTreeMap<ObjectModuleId, NodeId>,
        address: GlobalAddress,
        inner_object_blueprint: &str,
        inner_object_fields: Vec<Vec<u8>>,
    ) -> Result<NodeId, E>;

    /// Calls a method on an object
    fn call_method(
        &mut self,
        receiver: &NodeId,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E> {
        self.call_method_advanced(receiver, false, ObjectModuleId::Main, method_name, args)
    }

    fn call_direct_access_method(
        &mut self,
        receiver: &NodeId,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E> {
        self.call_method_advanced(receiver, true, ObjectModuleId::Main, method_name, args)
    }

    // TODO: Add Object Module logic
    /// Calls a method on an object module
    fn call_method_advanced(
        &mut self,
        receiver: &NodeId,
        direct_access: bool, // May change to enum for other types of reference in future
        module_id: ObjectModuleId,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E>;
}
