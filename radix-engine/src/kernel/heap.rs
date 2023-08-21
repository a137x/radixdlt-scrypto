use crate::blueprints::resource::*;
use crate::track::interface::NodeSubstates;
use crate::types::*;
use radix_engine_interface::blueprints::resource::{
    LiquidFungibleResource, LiquidNonFungibleResource, LockedFungibleResource,
    LockedNonFungibleResource,
};

pub struct Heap {
    nodes: NonIterMap<NodeId, NodeSubstates>,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum HeapRemovePartitionError {
    NodeNotFound(NodeId),
    ModuleNotFound(PartitionNumber),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum HeapRemoveNodeError {
    NodeNotFound(NodeId),
}

impl Heap {
    pub fn new() -> Self {
        Self {
            nodes: NonIterMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Checks if the given node is in this heap.
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        self.nodes.contains_key(node_id)
    }

    pub fn remove_module(
        &mut self,
        node_id: &NodeId,
        partition_number: PartitionNumber,
    ) -> Result<BTreeMap<SubstateKey, IndexedScryptoValue>, HeapRemovePartitionError> {
        if let Some(modules) = self.nodes.get_mut(node_id) {
            let module = modules
                .remove(&partition_number)
                .ok_or(HeapRemovePartitionError::ModuleNotFound(partition_number))?;
            Ok(module)
        } else {
            Err(HeapRemovePartitionError::NodeNotFound(node_id.clone()))
        }
    }

    /// Reads a substate
    pub fn get_substate(
        &self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
    ) -> Option<&IndexedScryptoValue> {
        self.nodes
            .get(node_id)
            .and_then(|node| node.get(&partition_num))
            .and_then(|module_substates| module_substates.get(substate_key))
    }

    /// Inserts or overwrites a substate
    pub fn set_substate(
        &mut self,
        node_id: NodeId,
        partition_num: PartitionNumber,
        substate_key: SubstateKey,
        substate_value: IndexedScryptoValue,
    ) {
        self.nodes
            .entry(node_id)
            .or_insert_with(|| NodeSubstates::default())
            .entry(partition_num)
            .or_default()
            .insert(substate_key, substate_value);
    }

    pub fn remove_substate(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
    ) -> Option<IndexedScryptoValue> {
        self.nodes
            .get_mut(node_id)
            .and_then(|n| n.get_mut(&partition_num))
            .and_then(|s| s.remove(substate_key))
    }

    /// Scans the keys of a node's partition. On an non-existing node/partition, this
    /// will return an empty vector
    pub fn scan_keys(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        count: u32,
    ) -> Vec<SubstateKey> {
        let node_substates = self
            .nodes
            .get_mut(node_id)
            .and_then(|n| n.get_mut(&partition_num));
        if let Some(substates) = node_substates {
            let substates: Vec<SubstateKey> = substates
                .iter()
                .map(|(key, _value)| key.clone())
                .take(count.try_into().unwrap())
                .collect();

            substates
        } else {
            vec![]
        }
    }

    /// Drains the substates from a node's partition. On an non-existing node/partition, this
    /// will return an empty vector
    pub fn drain_substates(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        count: u32,
    ) -> Vec<(SubstateKey, IndexedScryptoValue)> {
        let node_substates = self
            .nodes
            .get_mut(node_id)
            .and_then(|n| n.get_mut(&partition_num));
        if let Some(substates) = node_substates {
            let keys: Vec<SubstateKey> = substates
                .iter()
                .map(|(key, _)| key.clone())
                .take(count.try_into().unwrap())
                .collect();

            let mut items = Vec::new();

            for key in keys {
                let value = substates.remove(&key).unwrap();
                items.push((key, value));
            }

            items
        } else {
            vec![]
        }
    }

    /// Inserts a new node to heap.
    pub fn create_node(&mut self, node_id: NodeId, substates: NodeSubstates) {
        self.nodes.insert(node_id, substates);
    }

    /// Removes node.
    pub fn remove_node(&mut self, node_id: &NodeId) -> Result<NodeSubstates, HeapRemoveNodeError> {
        match self.nodes.remove(node_id) {
            Some(node_substates) => Ok(node_substates),
            None => Err(HeapRemoveNodeError::NodeNotFound(node_id.clone())),
        }
    }
}

#[derive(Debug)]
pub struct DroppedFungibleBucket {
    pub liquid: LiquidFungibleResource,
    pub locked: LockedFungibleResource,
}

#[derive(Debug)]
pub struct DroppedNonFungibleBucket {
    pub liquid: LiquidNonFungibleResource,
    pub locked: LockedNonFungibleResource,
}

impl Into<DroppedFungibleBucket> for Vec<Vec<u8>> {
    fn into(self) -> DroppedFungibleBucket {
        let liquid: LiquidFungibleResource =
            scrypto_decode(&self[FungibleBucketField::Liquid as usize]).unwrap();
        let locked: LockedFungibleResource =
            scrypto_decode(&self[FungibleBucketField::Locked as usize]).unwrap();

        DroppedFungibleBucket { liquid, locked }
    }
}

impl Into<DroppedNonFungibleBucket> for Vec<Vec<u8>> {
    fn into(self) -> DroppedNonFungibleBucket {
        let liquid: LiquidNonFungibleResource =
            scrypto_decode(&self[NonFungibleBucketField::Liquid as usize]).unwrap();
        let locked: LockedNonFungibleResource =
            scrypto_decode(&self[NonFungibleBucketField::Locked as usize]).unwrap();

        DroppedNonFungibleBucket { liquid, locked }
    }
}

pub struct DroppedFungibleProof {
    pub moveable: ProofMoveableSubstate,
    pub fungible_proof: FungibleProofSubstate,
}

pub struct DroppedNonFungibleProof {
    pub moveable: ProofMoveableSubstate,
    pub non_fungible_proof: NonFungibleProofSubstate,
}

impl Into<DroppedFungibleProof> for Vec<Vec<u8>> {
    fn into(self) -> DroppedFungibleProof {
        let moveable: ProofMoveableSubstate =
            scrypto_decode(&self[FungibleProofField::Moveable as usize]).unwrap();
        let fungible_proof: FungibleProofSubstate =
            scrypto_decode(&self[FungibleProofField::ProofRefs as usize]).unwrap();

        DroppedFungibleProof {
            moveable,
            fungible_proof,
        }
    }
}

impl Into<DroppedNonFungibleProof> for Vec<Vec<u8>> {
    fn into(self) -> DroppedNonFungibleProof {
        let moveable: ProofMoveableSubstate =
            scrypto_decode(&self[FungibleProofField::Moveable as usize]).unwrap();
        let non_fungible_proof: NonFungibleProofSubstate =
            scrypto_decode(&self[FungibleProofField::ProofRefs as usize]).unwrap();

        DroppedNonFungibleProof {
            moveable,
            non_fungible_proof,
        }
    }
}
