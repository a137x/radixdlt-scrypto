use crate::hash_tree::tree_store::{
    NodeKey, ReadableTreeStore, StaleTreePart, TreeNode, WriteableTreeStore,
};
use crate::hash_tree::{put_at_next_version, BatchChange, HashChange, SubstateHashChange};
use radix_engine_common::crypto::hash;
use radix_engine_store_interface::interface::{DatabaseUpdate, DatabaseUpdates};

struct CollectingTreeStore<'s, S> {
    readable_delegate: &'s S,
    diff: StateHashTreeDiff,
}

impl<'s, S: ReadableTreeStore> CollectingTreeStore<'s, S> {
    pub fn new(readable_delegate: &'s S) -> Self {
        Self {
            readable_delegate,
            diff: StateHashTreeDiff::new(),
        }
    }

    pub fn into_diff(self) -> StateHashTreeDiff {
        self.diff
    }
}

impl<'s, S: ReadableTreeStore> ReadableTreeStore for CollectingTreeStore<'s, S> {
    fn get_node(&self, key: &NodeKey) -> Option<TreeNode> {
        self.readable_delegate.get_node(key)
    }
}

impl<'s, S> WriteableTreeStore for CollectingTreeStore<'s, S> {
    fn insert_node(&mut self, key: NodeKey, node: TreeNode) {
        self.diff.new_nodes.push((key, node));
    }

    fn record_stale_tree_part(&mut self, part: StaleTreePart) {
        self.diff.stale_parts.push(part);
    }
}

#[derive(Clone)]
pub struct StateHashTreeDiff {
    pub new_nodes: Vec<(NodeKey, TreeNode)>,
    pub stale_parts: Vec<StaleTreePart>,
}

impl StateHashTreeDiff {
    pub fn new() -> Self {
        Self {
            new_nodes: Vec::new(),
            stale_parts: Vec::new(),
        }
    }
}

pub fn compute_state_tree_update<S: ReadableTreeStore>(
    store: &S,
    parent_state_version: u64,
    database_updates: &DatabaseUpdates,
    partition_deletions: &IndexSet<DbPartitionKey>,
) -> StateHashTreeDiff {
    let mut hash_changes = Vec::new();
    for db_partition_key in partition_deletions {
        hash_changes.push(HashChange::Batch(BatchChange::DeletePartition(
            db_partition_key.clone(),
        )));
    }
    for (db_partition_key, partition_updates) in database_updates {
        for (db_sort_key, database_update) in partition_updates {
            match database_update {
                DatabaseUpdate::Set(value) => {
                    hash_changes.push(HashChange::Single(SubstateHashChange::new(
                        (db_partition_key.clone(), db_sort_key.clone()),
                        Some(hash(value)),
                    )))
                }
                DatabaseUpdate::Delete => hash_changes.push(HashChange::Single(
                    SubstateHashChange::new((db_partition_key.clone(), db_sort_key.clone()), None),
                )),
            }
        }
    }

    let mut collector = CollectingTreeStore::new(store);
    put_at_next_version(
        &mut collector,
        Some(parent_state_version).filter(|v| *v > 0),
        hash_changes,
    );
    collector.into_diff()
}
