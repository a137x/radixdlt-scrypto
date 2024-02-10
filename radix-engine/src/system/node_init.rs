use crate::system::type_info::TypeInfoSubstate;
use crate::types::*;
use radix_engine_system_interface::types::SubstateKey;

pub fn type_info_partition(info: TypeInfoSubstate) -> BTreeMap<SubstateKey, IndexedScryptoValue> {
    BTreeMap::from([(
        TypeInfoField::TypeInfo.into(),
        IndexedScryptoValue::from_typed(&info),
    )])
}
