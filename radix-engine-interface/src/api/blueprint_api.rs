use radix_common::prelude::VersionedScryptoSchema;
use radix_common::types::*;
use radix_rust::rust::prelude::*;
use radix_rust::rust::rc::*;
use radix_rust::rust::vec::Vec;

pub trait ClientBlueprintApi<E> {
    /// Calls a function on a blueprint
    fn call_function(
        &mut self,
        package_address: PackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E>;

    fn resolve_blueprint_type(
        &mut self,
        blueprint_type_id: &BlueprintTypeIdentifier,
    ) -> Result<(Rc<VersionedScryptoSchema>, ScopedTypeId), E>;
}
