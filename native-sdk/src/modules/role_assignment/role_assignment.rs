use radix_engine_interface::api::node_modules::auth::{
    RoleAssignmentCreateInput, RoleAssignmentLockOwnerInput, RoleAssignmentSetInput,
    RoleAssignmentSetOwnerInput, ROLE_ASSIGNMENT_BLUEPRINT, ROLE_ASSIGNMENT_CREATE_IDENT,
    ROLE_ASSIGNMENT_SET_IDENT, ROLE_ASSIGNMENT_SET_OWNER_IDENT,
};
use radix_engine_interface::api::object_api::ObjectModuleId;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::resource::{
    AccessRule, OwnerRoleEntry, RoleKey, RolesInit,
};
use radix_engine_interface::constants::ROLE_ASSIGNMENT_MODULE_PACKAGE;
use radix_engine_interface::data::scrypto::model::Own;
use radix_engine_interface::data::scrypto::*;
use radix_engine_interface::types::NodeId;
use sbor::rust::fmt::Debug;
use sbor::rust::prelude::*;

pub struct RoleAssignment(pub Own);

impl RoleAssignment {
    pub fn create<Y, R: Into<OwnerRoleEntry>, E: Debug + ScryptoDecode>(
        owner_role: R,
        roles: BTreeMap<ObjectModuleId, RolesInit>,
        api: &mut Y,
    ) -> Result<Self, E>
    where
        Y: ClientApi<E>,
    {
        let rtn = api.call_function(
            ROLE_ASSIGNMENT_MODULE_PACKAGE,
            ROLE_ASSIGNMENT_BLUEPRINT,
            ROLE_ASSIGNMENT_CREATE_IDENT,
            scrypto_encode(&RoleAssignmentCreateInput {
                owner_role: owner_role.into(),
                roles,
            })
            .unwrap(),
        )?;

        let role_assignment: Own = scrypto_decode(&rtn).unwrap();

        Ok(Self(role_assignment))
    }
}

impl RoleAssignmentObject for RoleAssignment {
    fn self_id(&self) -> (&NodeId, ObjectModuleId) {
        (&self.0 .0, ObjectModuleId::Main)
    }
}

pub struct AttachedRoleAssignment(pub NodeId);

impl RoleAssignmentObject for AttachedRoleAssignment {
    fn self_id(&self) -> (&NodeId, ObjectModuleId) {
        (&self.0, ObjectModuleId::RoleAssignment)
    }
}

pub trait RoleAssignmentObject {
    fn self_id(&self) -> (&NodeId, ObjectModuleId);

    fn set_owner_role<Y: ClientApi<E>, E: Debug + ScryptoDecode, A: Into<AccessRule>>(
        &self,
        rule: A,
        api: &mut Y,
    ) -> Result<(), E> {
        let (node_id, module_id) = self.self_id();
        let _rtn = api.call_method_advanced(
            node_id,
            module_id,
            false,
            ROLE_ASSIGNMENT_SET_OWNER_IDENT,
            scrypto_encode(&RoleAssignmentSetOwnerInput { rule: rule.into() }).unwrap(),
        )?;

        Ok(())
    }

    fn lock_owner_role<Y: ClientApi<E>, E: Debug + ScryptoDecode, A: Into<AccessRule>>(
        &self,
        api: &mut Y,
    ) -> Result<(), E> {
        let (node_id, module_id) = self.self_id();
        let _rtn = api.call_method_advanced(
            node_id,
            module_id,
            false,
            ROLE_ASSIGNMENT_SET_OWNER_IDENT,
            scrypto_encode(&RoleAssignmentLockOwnerInput {}).unwrap(),
        )?;

        Ok(())
    }

    fn set_role<
        Y: ClientApi<E>,
        E: Debug + ScryptoDecode,
        R: Into<RoleKey>,
        A: Into<AccessRule>,
    >(
        &self,
        module: ObjectModuleId,
        role_key: R,
        rule: A,
        api: &mut Y,
    ) -> Result<(), E> {
        let (node_id, module_id) = self.self_id();
        let _rtn = api.call_method_advanced(
            node_id,
            module_id,
            false,
            ROLE_ASSIGNMENT_SET_IDENT,
            scrypto_encode(&RoleAssignmentSetInput {
                module,
                role_key: role_key.into(),
                rule: rule.into(),
            })
            .unwrap(),
        )?;

        Ok(())
    }
}
