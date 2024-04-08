use radix_common::constants::*;
use radix_common::types::*;
use radix_common::*;
use radix_engine_interface::*;

/// Defines the set of Nodes that all test [`CallFrame`]s have visibility to when they're first
/// created. This contains all of the well-known addresses of nodes.
pub(super) const GLOBAL_VISIBLE_NODES: [NodeId; 28] = [
    XRD.into_node_id(),
    SECP256K1_SIGNATURE_VIRTUAL_BADGE.into_node_id(),
    ED25519_SIGNATURE_VIRTUAL_BADGE.into_node_id(),
    PACKAGE_OF_DIRECT_CALLER_VIRTUAL_BADGE.into_node_id(),
    GLOBAL_CALLER_VIRTUAL_BADGE.into_node_id(),
    SYSTEM_TRANSACTION_BADGE.into_node_id(),
    PACKAGE_OWNER_BADGE.into_node_id(),
    VALIDATOR_OWNER_BADGE.into_node_id(),
    ACCOUNT_OWNER_BADGE.into_node_id(),
    IDENTITY_OWNER_BADGE.into_node_id(),
    PACKAGE_PACKAGE.into_node_id(),
    RESOURCE_PACKAGE.into_node_id(),
    ACCOUNT_PACKAGE.into_node_id(),
    IDENTITY_PACKAGE.into_node_id(),
    CONSENSUS_MANAGER_PACKAGE.into_node_id(),
    ACCESS_CONTROLLER_PACKAGE.into_node_id(),
    POOL_PACKAGE.into_node_id(),
    TRANSACTION_PROCESSOR_PACKAGE.into_node_id(),
    METADATA_MODULE_PACKAGE.into_node_id(),
    ROYALTY_MODULE_PACKAGE.into_node_id(),
    ROLE_ASSIGNMENT_MODULE_PACKAGE.into_node_id(),
    GENESIS_HELPER_PACKAGE.into_node_id(),
    FAUCET_PACKAGE.into_node_id(),
    TRANSACTION_TRACKER_PACKAGE.into_node_id(),
    CONSENSUS_MANAGER.into_node_id(),
    GENESIS_HELPER.into_node_id(),
    FAUCET.into_node_id(),
    TRANSACTION_TRACKER.into_node_id(),
];
