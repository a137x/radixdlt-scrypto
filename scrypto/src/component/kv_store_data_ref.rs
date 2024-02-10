use crate::engine::scrypto_env::ScryptoVmV1Api;
use radix_engine_common::data::scrypto::*;
use radix_engine_common::types::SubstateHandle;
use radix_engine_system_interface::key_value_entry_api::KeyValueEntryHandle;
use sbor::rust::fmt;
use sbor::rust::marker::PhantomData;
use sbor::rust::ops::{Deref, DerefMut};

pub struct KeyValueEntryRef<'a, V: ScryptoEncode> {
    lock_handle: KeyValueEntryHandle,
    value: V,
    phantom: PhantomData<&'a ()>,
}

impl<'a, V: fmt::Display + ScryptoEncode> fmt::Display for KeyValueEntryRef<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a, V: ScryptoEncode> KeyValueEntryRef<'a, V> {
    pub fn new(lock_handle: KeyValueEntryHandle, value: V) -> KeyValueEntryRef<'a, V> {
        KeyValueEntryRef {
            lock_handle,
            value,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, V: ScryptoEncode> Deref for KeyValueEntryRef<'a, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, V: ScryptoEncode> Drop for KeyValueEntryRef<'a, V> {
    fn drop(&mut self) {
        ScryptoVmV1Api::kv_entry_close(self.lock_handle);
    }
}

pub struct KeyValueEntryRefMut<'a, V: ScryptoEncode> {
    handle: KeyValueEntryHandle,
    value: V,
    phantom: PhantomData<&'a ()>,
}

impl<V: fmt::Display + ScryptoEncode> fmt::Display for KeyValueEntryRefMut<'_, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a, V: ScryptoEncode> KeyValueEntryRefMut<'a, V> {
    pub fn new(lock_handle: SubstateHandle, value: V) -> KeyValueEntryRefMut<'a, V> {
        KeyValueEntryRefMut {
            handle: lock_handle,
            value,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a, V: ScryptoEncode> Drop for KeyValueEntryRefMut<'a, V> {
    fn drop(&mut self) {
        let value = scrypto_encode(&self.value).unwrap();
        ScryptoVmV1Api::kv_entry_write(self.handle, value);
        ScryptoVmV1Api::kv_entry_close(self.handle);
    }
}

impl<'a, V: ScryptoEncode> Deref for KeyValueEntryRefMut<'a, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, V: ScryptoEncode> DerefMut for KeyValueEntryRefMut<'a, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
