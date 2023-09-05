use crate::engine::scrypto_env::ScryptoVmV1Api;

use crate::modules::ModuleHandle;
use crate::prelude::Attachable;
use radix_engine_derive::*;
use radix_engine_interface::api::node_modules::auth::{
    RoleAssignmentCreateInput, RoleAssignmentGetInput, RoleAssignmentLockOwnerInput,
    RoleAssignmentSetInput, RoleAssignmentSetOwnerInput, ROLE_ASSIGNMENT_BLUEPRINT,
    ROLE_ASSIGNMENT_CREATE_IDENT, ROLE_ASSIGNMENT_GET_IDENT, ROLE_ASSIGNMENT_LOCK_OWNER_IDENT,
    ROLE_ASSIGNMENT_SET_IDENT, ROLE_ASSIGNMENT_SET_OWNER_IDENT,
};
use radix_engine_interface::api::*;
use radix_engine_interface::blueprints::resource::{
    OwnerRoleEntry, RoleAssignmentInit, RoleKey, Rule,
};
use radix_engine_interface::constants::ROLE_ASSIGNMENT_MODULE_PACKAGE;
use radix_engine_interface::data::scrypto::model::*;
use radix_engine_interface::data::scrypto::{scrypto_decode, scrypto_encode};
use radix_engine_interface::*;
use sbor::rust::prelude::*;

pub trait HasRoleAssignment {
    fn set_owner_role<A: Into<Rule>>(&self, rule: A);
    fn lock_owner_role<A: Into<Rule>>(&self);
    fn set_role<A: Into<Rule>>(&self, name: &str, rule: A);
    fn get_role(&self, name: &str) -> Option<Rule>;
    fn set_metadata_role<A: Into<Rule>>(&self, name: &str, rule: A);
    fn set_component_royalties_role<A: Into<Rule>>(&self, name: &str, rule: A);
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

    pub fn set_owner_role<A: Into<Rule>>(&self, rule: A) {
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

    fn internal_set_role<A: Into<Rule>>(&self, module: ModuleId, name: &str, rule: A) {
        self.call_ignore_rtn(
            ROLE_ASSIGNMENT_SET_IDENT,
            &RoleAssignmentSetInput {
                module,
                role_key: RoleKey::new(name),
                rule: rule.into(),
            },
        );
    }

    fn internal_get_role(&self, module: ModuleId, name: &str) -> Option<Rule> {
        self.call(
            ROLE_ASSIGNMENT_GET_IDENT,
            &RoleAssignmentGetInput {
                module,
                role_key: RoleKey::new(name),
            },
        )
    }

    pub fn set_role<A: Into<Rule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::Main, name, rule);
    }

    pub fn get_role(&self, name: &str) -> Option<Rule> {
        self.internal_get_role(ModuleId::Main, name)
    }

    pub fn set_metadata_role<A: Into<Rule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::Metadata, name, rule);
    }

    pub fn get_metadata_role(&self, name: &str) -> Option<Rule> {
        self.internal_get_role(ModuleId::Metadata, name)
    }

    pub fn set_component_royalties_role<A: Into<Rule>>(&self, name: &str, rule: A) {
        self.internal_set_role(ModuleId::Royalty, name, rule);
    }

    pub fn get_component_royalties_role(&self, name: &str) -> Option<Rule> {
        self.internal_get_role(ModuleId::Royalty, name)
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
    MUTABLE(Rule),
}

impl From<Mutability> for Rule {
    fn from(val: Mutability) -> Self {
        match val {
            Mutability::LOCKED => Rule::DenyAll,
            Mutability::MUTABLE(rule) => rule,
        }
    }
}
