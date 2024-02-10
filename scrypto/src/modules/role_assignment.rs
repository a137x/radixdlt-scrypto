use super::{Attachable, ModuleHandle};
use crate::engine::scrypto_env::ScryptoVmV1Api;
use module_blueprints_interface::auth::{
    RoleAssignmentCreateInput, RoleAssignmentGetInput, RoleAssignmentLockOwnerInput,
    RoleAssignmentSetInput, RoleAssignmentSetOwnerInput, ROLE_ASSIGNMENT_CREATE_IDENT,
    ROLE_ASSIGNMENT_GET_IDENT, ROLE_ASSIGNMENT_LOCK_OWNER_IDENT, ROLE_ASSIGNMENT_SET_IDENT,
    ROLE_ASSIGNMENT_SET_OWNER_IDENT,
};
use radix_engine_common::constants::ROLE_ASSIGNMENT_MODULE_PACKAGE;
use radix_engine_common::data::scrypto::model::*;
use radix_engine_common::data::scrypto::{scrypto_decode, scrypto_encode};
use radix_engine_common::prelude::*;
use radix_engine_common::*;

pub trait HasRoleAssignment {
    fn set_owner_role<A: Into<AccessRule>>(&self, rule: A);
    fn lock_owner_role(&self);
    fn set_role<A: Into<AccessRule>>(&self, name: &str, rule: A);
    fn get_role(&self, name: &str) -> Option<AccessRule>;
    fn set_metadata_role<A: Into<AccessRule>>(&self, name: &str, rule: A);
    fn set_component_royalties_role<A: Into<AccessRule>>(&self, name: &str, rule: A);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleAssignment(pub ModuleHandle);

impl RoleAssignment {
    pub fn new<R: Into<OwnerRoleEntry>>(
        owner_role: R,
        roles: IndexMap<ModuleId, RoleAssignmentInit>,
    ) -> Self {
        let rtn = ScryptoVmV1Api::blueprint_call(
            ROLE_ASSIGNMENT_MODULE_PACKAGE,
            ROLE_ASSIGNMENT_BLUEPRINT,
            ROLE_ASSIGNMENT_CREATE_IDENT,
            scrypto_encode(&RoleAssignmentCreateInput {
                owner_role: owner_role.into(),
                roles,
            })
            .unwrap(),
        );
        let role_assignment: Own = scrypto_decode(&rtn).unwrap();
        Self(ModuleHandle::Own(role_assignment))
    }

    pub fn set_owner_role<A: Into<AccessRule>>(&self, rule: A) {
        self.call_ignore_rtn(
            ROLE_ASSIGNMENT_SET_OWNER_IDENT,
            &RoleAssignmentSetOwnerInput { rule: rule.into() },
        );
    }

    pub fn lock_owner_role(&self) {
        self.call_ignore_rtn(
            ROLE_ASSIGNMENT_LOCK_OWNER_IDENT,
            &RoleAssignmentLockOwnerInput {},
        );
    }

    fn internal_set_role<A: Into<AccessRule>>(&self, module: ModuleId, name: &str, rule: A) {
        self.call_ignore_rtn(
            ROLE_ASSIGNMENT_SET_IDENT,
            &RoleAssignmentSetInput {
                module,
                role_key: RoleKey::new(name),
                rule: rule.into(),
            },
        );
    }

    fn internal_get_role(&self, module: ModuleId, name: &str) -> Option<AccessRule> {
        self.call(
            ROLE_ASSIGNMENT_GET_IDENT,
            &RoleAssignmentGetInput {
                module,
                role_key: RoleKey::new(name),
            },
        )
    }

    pub fn set_role<A: Into<AccessRule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::Main, name, rule);
    }

    pub fn get_role(&self, name: &str) -> Option<AccessRule> {
        self.internal_get_role(ModuleId::Main, name)
    }

    pub fn set_metadata_role<A: Into<AccessRule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::Metadata, name, rule);
    }

    pub fn get_metadata_role(&self, name: &str) -> Option<AccessRule> {
        self.internal_get_role(ModuleId::Metadata, name)
    }

    pub fn set_component_royalties_role<A: Into<AccessRule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::Royalty, name, rule);
    }

    pub fn get_component_royalties_role(&self, name: &str) -> Option<AccessRule> {
        self.internal_get_role(ModuleId::Royalty, name)
    }

    pub fn set_role_assignment_role<A: Into<AccessRule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::RoleAssignment, name, rule);
    }

    pub fn get_role_assignment_role(&self, name: &str) -> Option<AccessRule> {
        self.internal_get_role(ModuleId::RoleAssignment, name)
    }
}

impl Attachable for RoleAssignment {
    const MODULE_ID: AttachedModuleId = AttachedModuleId::RoleAssignment;

    fn new(handle: ModuleHandle) -> Self {
        Self(handle)
    }

    fn handle(&self) -> &ModuleHandle {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScryptoSbor)]
pub enum Mutability {
    LOCKED,
    MUTABLE(AccessRule),
}

impl From<Mutability> for AccessRule {
    fn from(val: Mutability) -> Self {
        match val {
            Mutability::LOCKED => AccessRule::DenyAll,
            Mutability::MUTABLE(rule) => rule,
        }
    }
}
