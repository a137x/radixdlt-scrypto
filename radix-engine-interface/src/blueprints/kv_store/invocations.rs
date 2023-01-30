use crate::api::types::*;
use crate::data::types::Own;
use crate::*;
use sbor::rust::fmt::Debug;

//==================================================
//  KeyValueStore::create() -> Own<KeyValueStore>
//==================================================

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct KeyValueStoreCreateInvocation {}

impl Invocation for KeyValueStoreCreateInvocation {
    type Output = Own;

    fn fn_identifier(&self) -> FnIdentifier {
        FnIdentifier::Native(NativeFn::KeyValueStore(KeyValueStoreFn::Create))
    }
}

impl SerializableInvocation for KeyValueStoreCreateInvocation {
    type ScryptoOutput = Own;
}

impl Into<CallTableInvocation> for KeyValueStoreCreateInvocation {
    fn into(self) -> CallTableInvocation {
        NativeInvocation::KeyValueStore(KeyValueStoreInvocation::Create(self)).into()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct KeyValueStoreGetMethodArgs {
    pub key: Vec<u8>,
}

//=====================================================
// KeyValueStore::get(&self, key: Vec<u8>) -> LockHandle
//=====================================================

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct KeyValueStoreGetInvocation {
    pub receiver: KeyValueStoreId,
    pub key: Vec<u8>,
}

impl Invocation for KeyValueStoreGetInvocation {
    type Output = LockHandle;

    fn fn_identifier(&self) -> FnIdentifier {
        FnIdentifier::Native(NativeFn::KeyValueStore(KeyValueStoreFn::Get))
    }
}

impl SerializableInvocation for KeyValueStoreGetInvocation {
    type ScryptoOutput = LockHandle;
}

impl Into<CallTableInvocation> for KeyValueStoreGetInvocation {
    fn into(self) -> CallTableInvocation {
        NativeInvocation::KeyValueStore(KeyValueStoreInvocation::Get(self)).into()
    }
}

//=======================================================================
// KeyValueStore::get_mut(&self, key: Vec<u8>) -> LockHandle
//=======================================================================

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct KeyValueStoreGetMutInvocation {
    pub receiver: KeyValueStoreId,
    pub key: Vec<u8>,
}

impl Invocation for KeyValueStoreGetMutInvocation {
    type Output = LockHandle;

    fn fn_identifier(&self) -> FnIdentifier {
        FnIdentifier::Native(NativeFn::KeyValueStore(KeyValueStoreFn::GetMut))
    }
}

impl SerializableInvocation for KeyValueStoreGetMutInvocation {
    type ScryptoOutput = LockHandle;
}

impl Into<CallTableInvocation> for KeyValueStoreGetMutInvocation {
    fn into(self) -> CallTableInvocation {
        NativeInvocation::KeyValueStore(KeyValueStoreInvocation::GetMut(self)).into()
    }
}

//=============================================================
// KeyValueStore::insert(&self, key: Vec<u8>, value: Vec<u8>)
//=============================================================

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct KeyValueStoreInsertMethodArgs {
    pub current_time_ms: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct KeyValueStoreInsertInvocation {
    pub receiver: KeyValueStoreId,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Invocation for KeyValueStoreInsertInvocation {
    type Output = ();

    fn fn_identifier(&self) -> FnIdentifier {
        FnIdentifier::Native(NativeFn::KeyValueStore(KeyValueStoreFn::Insert))
    }
}

impl SerializableInvocation for KeyValueStoreInsertInvocation {
    type ScryptoOutput = ();
}

impl Into<CallTableInvocation> for KeyValueStoreInsertInvocation {
    fn into(self) -> CallTableInvocation {
        NativeInvocation::KeyValueStore(KeyValueStoreInvocation::Insert(self)).into()
    }
}
