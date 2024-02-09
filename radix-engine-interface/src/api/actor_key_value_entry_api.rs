use radix_engine_common::data::scrypto::{scrypto_decode, ScryptoDecode};
use radix_engine_common::prelude::*;
use sbor::rust::fmt::Debug;
use sbor::rust::vec::Vec;

use super::{KeyValueEntryHandle, LockFlags};

pub trait ClientActorKeyValueEntryApi<E: Debug> {
    /// If the key value entry doesn't exist, it uses the default "Option::None"
    fn actor_open_key_value_entry(
        &mut self,
        object_handle: ActorStateHandle,
        collection_index: CollectionIndex,
        key: &Vec<u8>,
        flags: LockFlags,
    ) -> Result<KeyValueEntryHandle, E>;

    fn actor_remove_key_value_entry(
        &mut self,
        object_handle: ActorStateHandle,
        collection_index: CollectionIndex,
        key: &Vec<u8>,
    ) -> Result<Vec<u8>, E>;

    fn actor_remove_key_value_entry_typed<V: ScryptoDecode>(
        &mut self,
        object_handle: ActorStateHandle,
        collection_index: CollectionIndex,
        key: &Vec<u8>,
    ) -> Result<Option<V>, E> {
        let removed = self.actor_remove_key_value_entry(object_handle, collection_index, key)?;
        let rtn = scrypto_decode(&removed).unwrap();
        Ok(rtn)
    }
}
