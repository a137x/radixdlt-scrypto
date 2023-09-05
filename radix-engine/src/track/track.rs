use crate::kernel::call_frame::TransientSubstates;
use crate::track::interface::{
    CommitableSubstateStore, IOAccess, NodeSubstates, TrackedSubstateInfo,
};
use crate::track::utils::OverlayingResultIterator;
use crate::track::LegacyStateUpdates;
use crate::types::*;
use radix_engine_interface::types::*;
use radix_engine_store_interface::db_key_mapper::SubstateKeyContent;
use radix_engine_store_interface::interface::{DatabaseUpdates, DbPartitionKey, DbSubstateValue};
use radix_engine_store_interface::{
    db_key_mapper::DatabaseKeyMapper,
    interface::{DatabaseUpdate, DbSortKey, PartitionEntry, SubstateDatabase},
};
use sbor::rust::collections::btree_map::Entry;
use sbor::rust::iter::empty;
use sbor::rust::mem;

use super::interface::{CanonicalPartition, CanonicalSubstateKey, StoreCommit, StoreCommitInfo};

/// A tree-like description of all updates that happened to a stored state, to be included as a part
/// of a transaction receipt.
/// This structure is indexed (i.e. uses [`IndexMap`]s where [`Vec`]s could be used) for convenience
/// and performance, since both the source (i.e. Track) and the sink (i.e. Database and API) operate
/// on indexed structures too.
/// This structure maintains partial information on the order of operations (please see individual
/// fields for details), since the end users care about it. Please note that this means multiple
/// instances of [`StateUpdates`] can represent the same transform of state store (i.e. differing
/// only by order of some operations), and hence it is not 100% "canonical form".
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor, Default)]
pub struct StateUpdates {
    /// Indexed Node-level updates, captured in the order of first update operation to a Node.
    pub by_node: IndexMap<NodeId, NodeStateUpdates>,
}

impl StateUpdates {
    /// Starts a Node-level update.
    pub fn of_node(&mut self, node_id: NodeId) -> &mut NodeStateUpdates {
        self.by_node
            .entry(node_id)
            .or_insert_with(|| NodeStateUpdates::Delta {
                by_partition: index_map_new(),
            })
    }

    pub fn into_legacy(self) -> LegacyStateUpdates {
        self.into()
    }
}

/// A description of all updates that happened to a state of a single Node.
/// Note: currently, we do not support any Node-wide changes (e.g. deleting entire Node); however,
/// we use an enum for potential future development.
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum NodeStateUpdates {
    /// A "delta" update to a Node, touching only selected Partitions.
    /// Contains indexed Partition-level updates, captured in the order of first update operation to
    /// a Partition.
    Delta {
        by_partition: IndexMap<PartitionNumber, PartitionStateUpdates>,
    },
}

impl Default for NodeStateUpdates {
    fn default() -> Self {
        NodeStateUpdates::Delta {
            by_partition: index_map_new(),
        }
    }
}

impl NodeStateUpdates {
    /// Starts a Partition-level update.
    pub fn of_partition(&mut self, partition_num: PartitionNumber) -> &mut PartitionStateUpdates {
        match self {
            NodeStateUpdates::Delta { by_partition } => {
                by_partition.entry(partition_num).or_default()
            }
        }
    }
}

/// A description of all updates that happened to a state of a single Partition.
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum PartitionStateUpdates {
    /// A "delta" update to a Partition, touching only selected Substates.
    /// Contains indexed Substate-level updates, captured in the order of first update operation to
    /// a Substate.
    Delta {
        by_substate: IndexMap<SubstateKey, DatabaseUpdate>,
    },
    /// A batch update.
    Batch(BatchPartitionUpdate),
}

impl Default for PartitionStateUpdates {
    fn default() -> Self {
        PartitionStateUpdates::Delta {
            by_substate: index_map_new(),
        }
    }
}

impl PartitionStateUpdates {
    /// Resets the partition to an empty state.
    pub fn delete(&mut self) {
        *self = PartitionStateUpdates::Batch(BatchPartitionUpdate::Reset {
            new_substate_values: index_map_new(),
        });
    }

    /// Applies the given updates on top of the current updates to the partition.
    pub fn update_substates(
        &mut self,
        updates: impl IntoIterator<Item = (SubstateKey, DatabaseUpdate)>,
    ) {
        match self {
            PartitionStateUpdates::Delta { by_substate } => by_substate.extend(updates),
            PartitionStateUpdates::Batch(BatchPartitionUpdate::Reset {
                new_substate_values,
            }) => {
                for (substate_key, database_update) in updates {
                    match database_update {
                        DatabaseUpdate::Set(new_value) => {
                            new_substate_values.insert(substate_key, new_value);
                        }
                        DatabaseUpdate::Delete => {
                            let existed = new_substate_values.remove(&substate_key).is_some();
                            if !existed {
                                panic!("inconsistent update: delete of substate {:?} not existing in reset partition", substate_key);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A description of a batch update affecting an entire Partition.
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum BatchPartitionUpdate {
    /// A reset, dropping all Substates of a partition and replacing them with a new set.
    /// Contains indexed new Substate values, captured in the order of creation of a Substate.
    Reset {
        new_substate_values: IndexMap<SubstateKey, DbSubstateValue>,
    },
}

impl StateUpdates {
    /// Uses the given [`DatabaseKeyMapper`] to express the contained [`StateUpdate`]s using
    /// database-level key encoding.
    /// Note: We are still missing support for "delete partition" operation in database layer, so we
    /// effectively let deleted partition live, assuming they are not touched again. For this
    /// reason, this method panics if it detects a write to a partition that was deleted.
    pub fn create_database_updates<M: DatabaseKeyMapper>(&self) -> DatabaseUpdates {
        // TODO(after RCNet-v3.1): Expand the `DatabaseUpdates` definition in order to capture the
        // `partition_deletions` there as well (preferably using a "canonicalized" representation).
        let mut database_updates = DatabaseUpdates::default();
        for (node_id, node_state_updates) in &self.by_node {
            match node_state_updates {
                NodeStateUpdates::Delta { by_partition } => {
                    for (partition_num, partition_state_updates) in by_partition {
                        match partition_state_updates {
                            PartitionStateUpdates::Delta { by_substate } => {
                                let partition_updates = database_updates
                                    .entry(M::to_db_partition_key(node_id, *partition_num))
                                    .or_default();
                                for (substate_key, update) in by_substate {
                                    partition_updates
                                        .insert(M::to_db_sort_key(substate_key), update.clone());
                                }
                            }
                            PartitionStateUpdates::Batch(BatchPartitionUpdate::Reset {
                                new_substate_values,
                            }) => {
                                if !new_substate_values.is_empty() {
                                    panic!("no support for writing to a deleted partition");
                                }
                            }
                        };
                    }
                }
            }
        }
        database_updates
    }
}

#[derive(Clone, Debug)]
pub struct RuntimeSubstate {
    pub value: IndexedScryptoValue,
}

impl RuntimeSubstate {
    fn new(value: IndexedScryptoValue) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug)]
pub enum ReadOnly {
    NonExistent,
    Existent(RuntimeSubstate),
}

#[derive(Clone, Debug)]
pub enum Write {
    Update(RuntimeSubstate),
    Delete,
}

impl Write {
    fn into_value(self) -> Option<IndexedScryptoValue> {
        match self {
            Write::Update(substate) => Some(substate.value),
            Write::Delete => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TrackedSubstate {
    pub substate_key: SubstateKey,
    pub substate_value: TrackedSubstateValue,
}

// TODO: Add new virtualized
#[derive(Clone, Debug)]
pub enum TrackedSubstateValue {
    New(RuntimeSubstate),
    ReadOnly(ReadOnly),
    ReadExistAndWrite(IndexedScryptoValue, Write),
    ReadNonExistAndWrite(RuntimeSubstate),
    WriteOnly(Write),
    Garbage,
}

impl TrackedSubstate {
    fn size(&self) -> usize {
        // `substate_key` is accounted as part of the CanonicalSubstateKey
        self.substate_value.size()
    }
}

impl TrackedSubstateValue {
    fn size(&self) -> usize {
        match self {
            TrackedSubstateValue::New(x) => x.value.len(),
            TrackedSubstateValue::ReadOnly(r) => match r {
                ReadOnly::NonExistent => 0,
                ReadOnly::Existent(x) => x.value.len(),
            },
            TrackedSubstateValue::ReadExistAndWrite(e, w) => {
                e.len()
                    + match w {
                        Write::Update(x) => x.value.len(),
                        Write::Delete => 0,
                    }
            }
            TrackedSubstateValue::ReadNonExistAndWrite(x) => x.value.len(),
            TrackedSubstateValue::WriteOnly(w) => match w {
                Write::Update(x) => x.value.len(),
                Write::Delete => 0,
            },
            TrackedSubstateValue::Garbage => 0,
        }
    }

    fn get_runtime_substate_mut(&mut self) -> Option<&mut RuntimeSubstate> {
        match self {
            TrackedSubstateValue::New(substate)
            | TrackedSubstateValue::WriteOnly(Write::Update(substate))
            | TrackedSubstateValue::ReadOnly(ReadOnly::Existent(substate))
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Update(substate))
            | TrackedSubstateValue::ReadNonExistAndWrite(substate) => Some(substate),

            TrackedSubstateValue::WriteOnly(Write::Delete)
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Delete)
            | TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent)
            | TrackedSubstateValue::Garbage => None,
        }
    }

    pub fn get(&self) -> Option<&IndexedScryptoValue> {
        match self {
            TrackedSubstateValue::New(substate)
            | TrackedSubstateValue::WriteOnly(Write::Update(substate))
            | TrackedSubstateValue::ReadOnly(ReadOnly::Existent(substate))
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Update(substate))
            | TrackedSubstateValue::ReadNonExistAndWrite(substate) => Some(&substate.value),
            TrackedSubstateValue::WriteOnly(Write::Delete)
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Delete)
            | TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent)
            | TrackedSubstateValue::Garbage => None,
        }
    }

    pub fn set(&mut self, value: IndexedScryptoValue) {
        match self {
            TrackedSubstateValue::Garbage => {
                *self = TrackedSubstateValue::WriteOnly(Write::Update(RuntimeSubstate::new(value)));
            }
            TrackedSubstateValue::New(substate)
            | TrackedSubstateValue::WriteOnly(Write::Update(substate))
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Update(substate))
            | TrackedSubstateValue::ReadNonExistAndWrite(substate) => {
                substate.value = value;
            }
            TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent) => {
                let new_tracked =
                    TrackedSubstateValue::ReadNonExistAndWrite(RuntimeSubstate::new(value));
                *self = new_tracked;
            }
            TrackedSubstateValue::ReadOnly(ReadOnly::Existent(old)) => {
                let new_tracked = TrackedSubstateValue::ReadExistAndWrite(
                    old.value.clone(),
                    Write::Update(RuntimeSubstate::new(value)),
                );
                *self = new_tracked;
            }
            TrackedSubstateValue::ReadExistAndWrite(_, write @ Write::Delete)
            | TrackedSubstateValue::WriteOnly(write @ Write::Delete) => {
                *write = Write::Update(RuntimeSubstate::new(value));
            }
        };
    }

    pub fn take(&mut self) -> Option<IndexedScryptoValue> {
        match self {
            TrackedSubstateValue::Garbage => None,
            TrackedSubstateValue::New(..) => {
                let old = mem::replace(self, TrackedSubstateValue::Garbage);
                old.into_value()
            }
            TrackedSubstateValue::WriteOnly(_) => {
                let old = mem::replace(self, TrackedSubstateValue::WriteOnly(Write::Delete));
                old.into_value()
            }
            TrackedSubstateValue::ReadExistAndWrite(_, write) => {
                let write = mem::replace(write, Write::Delete);
                write.into_value()
            }
            TrackedSubstateValue::ReadNonExistAndWrite(..) => {
                let old = mem::replace(self, TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent));
                old.into_value()
            }
            TrackedSubstateValue::ReadOnly(ReadOnly::Existent(v)) => {
                let new_tracked =
                    TrackedSubstateValue::ReadExistAndWrite(v.value.clone(), Write::Delete);
                let old = mem::replace(self, new_tracked);
                old.into_value()
            }
            TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent) => None,
        }
    }

    fn revert_writes(&mut self) {
        match self {
            TrackedSubstateValue::ReadOnly(..) | TrackedSubstateValue::Garbage => {}
            TrackedSubstateValue::New(..) | TrackedSubstateValue::WriteOnly(_) => {
                *self = TrackedSubstateValue::Garbage;
            }
            TrackedSubstateValue::ReadExistAndWrite(read, _) => {
                *self = TrackedSubstateValue::ReadOnly(ReadOnly::Existent(RuntimeSubstate::new(
                    read.clone(),
                )));
            }
            TrackedSubstateValue::ReadNonExistAndWrite(..) => {
                *self = TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent);
            }
        }
    }

    pub fn into_value(self) -> Option<IndexedScryptoValue> {
        match self {
            TrackedSubstateValue::New(substate)
            | TrackedSubstateValue::WriteOnly(Write::Update(substate))
            | TrackedSubstateValue::ReadOnly(ReadOnly::Existent(substate))
            | TrackedSubstateValue::ReadNonExistAndWrite(substate)
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Update(substate)) => {
                Some(substate.value)
            }
            TrackedSubstateValue::WriteOnly(Write::Delete)
            | TrackedSubstateValue::ReadExistAndWrite(_, Write::Delete)
            | TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent)
            | TrackedSubstateValue::Garbage => None,
        }
    }
}

#[derive(Debug)]
pub struct TrackedPartition {
    pub substates: BTreeMap<DbSortKey, TrackedSubstate>,
    pub range_read: u32,
}

impl TrackedPartition {
    pub fn new() -> Self {
        Self {
            substates: BTreeMap::new(),
            range_read: 0,
        }
    }

    pub fn new_with_substates(substates: BTreeMap<DbSortKey, TrackedSubstate>) -> Self {
        Self {
            substates,
            range_read: 0,
        }
    }

    pub fn revert_writes(&mut self) {
        for substate in &mut self.substates.values_mut() {
            substate.substate_value.revert_writes();
        }
    }
}

#[derive(Debug)]
pub struct TrackedNode {
    pub tracked_partitions: IndexMap<PartitionNumber, TrackedPartition>,
    // If true, then all SubstateUpdates under this NodeUpdate must be inserts
    // The extra information, though awkward structurally, makes for a much
    // simpler iteration implementation as long as the invariant is maintained
    pub is_new: bool,
}

impl TrackedNode {
    pub fn new(is_new: bool) -> Self {
        Self {
            tracked_partitions: index_map_new(),
            is_new,
        }
    }

    pub fn revert_writes(&mut self) {
        for (_, tracked_partition) in &mut self.tracked_partitions {
            tracked_partition.revert_writes();
        }
    }
}

pub fn to_state_updates<M: DatabaseKeyMapper + 'static>(
    index: IndexMap<NodeId, TrackedNode>,
    partition_deletions: IndexSet<(NodeId, PartitionNumber)>,
) -> StateUpdates {
    let mut system_updates = index_map_new();
    for (node_id, tracked_node) in index {
        for (partition_num, tracked_partition) in tracked_node.tracked_partitions {
            let mut partition_updates = index_map_new();
            for tracked in tracked_partition.substates.into_values() {
                let update = match tracked.substate_value {
                    TrackedSubstateValue::ReadOnly(..) | TrackedSubstateValue::Garbage => None,
                    TrackedSubstateValue::ReadNonExistAndWrite(substate)
                    | TrackedSubstateValue::New(substate) => {
                        Some(DatabaseUpdate::Set(substate.value.into()))
                    }
                    TrackedSubstateValue::ReadExistAndWrite(_, write)
                    | TrackedSubstateValue::WriteOnly(write) => match write {
                        Write::Delete => Some(DatabaseUpdate::Delete),
                        Write::Update(substate) => Some(DatabaseUpdate::Set(substate.value.into())),
                    },
                };
                if let Some(update) = update {
                    partition_updates.insert(tracked.substate_key, update);
                }
            }
            system_updates.insert((node_id.clone(), partition_num), partition_updates);
        }
    }

    StateUpdates::from(LegacyStateUpdates {
        partition_deletions,
        system_updates,
    })
}

struct IterationCountedIter<'a, E> {
    iter: Box<dyn Iterator<Item = Result<(DbSortKey, (SubstateKey, IndexedScryptoValue)), E>> + 'a>,
    num_iterations: u32,
}

impl<'a, E> IterationCountedIter<'a, E> {
    fn new(
        iter: Box<
            dyn Iterator<Item = Result<(DbSortKey, (SubstateKey, IndexedScryptoValue)), E>> + 'a,
        >,
    ) -> Self {
        Self {
            iter,
            num_iterations: 0u32,
        }
    }
}

impl<'a, E> Iterator for IterationCountedIter<'a, E> {
    type Item = Result<(DbSortKey, (SubstateKey, IndexedScryptoValue)), E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.num_iterations = self.num_iterations + 1;
        self.iter.next()
    }
}

/// Transaction-wide states and side effects
pub struct Track<'s, S: SubstateDatabase, M: DatabaseKeyMapper + 'static> {
    /// Substate database, use `get_substate_from_db` and `list_entries_from_db` for access
    substate_db: &'s S,

    tracked_nodes: IndexMap<NodeId, TrackedNode>,
    force_write_tracked_nodes: IndexMap<NodeId, TrackedNode>,
    /// TODO: if time allows, consider merging into tracked nodes.
    deleted_partitions: IndexSet<(NodeId, PartitionNumber)>,

    transient_substates: TransientSubstates,

    phantom_data: PhantomData<M>,
}

impl<'s, S: SubstateDatabase, M: DatabaseKeyMapper + 'static> Track<'s, S, M> {
    pub fn new(substate_db: &'s S) -> Self {
        Self {
            substate_db,
            force_write_tracked_nodes: index_map_new(),
            tracked_nodes: index_map_new(),
            deleted_partitions: index_set_new(),
            transient_substates: TransientSubstates::new(),
            phantom_data: PhantomData::default(),
        }
    }

    // TODO cleanup interface to avoid redundant information
    fn get_substate_from_db<E, F: FnMut(IOAccess) -> Result<(), E>>(
        substate_db: &'s S,
        partition_key: &DbPartitionKey,
        sort_key: &DbSortKey,
        on_io_access: &mut F,
        canonical_substate_key: CanonicalSubstateKey,
    ) -> Result<Option<IndexedScryptoValue>, E> {
        let result = substate_db
            .get_substate(partition_key, sort_key)
            .map(|e| IndexedScryptoValue::from_vec(e).expect("Failed to decode substate"));
        if let Some(x) = &result {
            on_io_access(IOAccess::ReadFromDb(canonical_substate_key, x.len()))?;
        } else {
            on_io_access(IOAccess::ReadFromDbNotFound(canonical_substate_key))?;
        }
        Ok(result)
    }

    // TODO cleanup interface to avoid redundant information
    fn list_entries_from_db<
        'x,
        E: 'x,
        F: FnMut(IOAccess) -> Result<(), E> + 'x,
        K: SubstateKeyContent + 'static,
    >(
        substate_db: &'x S,
        partition_key: &DbPartitionKey,
        on_io_access: &'x mut F,
        canonical_partition: CanonicalPartition,
    ) -> Box<dyn Iterator<Item = Result<(DbSortKey, (SubstateKey, IndexedScryptoValue)), E>> + 'x>
    {
        struct TracedIterator<
            'a,
            E,
            F: FnMut(IOAccess) -> Result<(), E>,
            M: DatabaseKeyMapper + 'static,
            K: SubstateKeyContent + 'static,
        > {
            iterator: Box<dyn Iterator<Item = PartitionEntry> + 'a>,
            on_io_access: &'a mut F,
            canonical_partition: CanonicalPartition,
            errored_out: bool,
            phantom1: PhantomData<M>,
            phantom2: PhantomData<K>,
        }

        impl<
                'a,
                E,
                F: FnMut(IOAccess) -> Result<(), E>,
                M: DatabaseKeyMapper + 'static,
                K: SubstateKeyContent + 'static,
            > Iterator for TracedIterator<'a, E, F, M, K>
        {
            type Item = Result<(DbSortKey, (SubstateKey, IndexedScryptoValue)), E>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.errored_out {
                    return None;
                }

                let result = self.iterator.next();
                if let Some(x) = result {
                    let substate_key = M::from_db_sort_key::<K>(&x.0);
                    let substate_value =
                        IndexedScryptoValue::from_vec(x.1).expect("Failed to decode substate");
                    let io_access = IOAccess::ReadFromDb(
                        CanonicalSubstateKey::of(self.canonical_partition, substate_key.clone()),
                        substate_value.len(),
                    );
                    let result = (self.on_io_access)(io_access);
                    match result {
                        Ok(()) => Some(Ok((x.0, (substate_key, substate_value)))),
                        Err(e) => {
                            self.errored_out = true;
                            Some(Err(e))
                        }
                    }
                } else {
                    None
                }
            }
        }

        Box::new(TracedIterator {
            iterator: substate_db.list_entries(partition_key),
            on_io_access,
            canonical_partition,
            errored_out: false,
            phantom1: PhantomData::<M>,
            phantom2: PhantomData::<K>,
        })
    }

    /// Reverts all non force write changes.
    ///
    /// Note that dependencies will never be reverted.
    pub fn revert_non_force_write_changes(&mut self) {
        self.tracked_nodes
            .retain(|_, tracked_node| !tracked_node.is_new);
        for (_, tracked_node) in &mut self.tracked_nodes {
            tracked_node.revert_writes();
        }

        let force_writes = mem::take(&mut self.force_write_tracked_nodes);

        for (node_id, force_track_node) in force_writes {
            for (partition_num, force_track_partition) in force_track_node.tracked_partitions {
                for (db_sort_key, force_track_key) in force_track_partition.substates {
                    let tracked_node = self.tracked_nodes.get_mut(&node_id).unwrap();
                    let tracked_partition = tracked_node
                        .tracked_partitions
                        .get_mut(&partition_num)
                        .unwrap();
                    let tracked = &mut tracked_partition
                        .substates
                        .get_mut(&db_sort_key)
                        .unwrap()
                        .substate_value;
                    *tracked = force_track_key.substate_value;
                }
            }
        }
    }

    /// Finalizes changes captured by this substate store.
    ///
    ///  Returns the state changes and dependencies.
    pub fn finalize(
        self,
    ) -> (
        IndexMap<NodeId, TrackedNode>,
        IndexSet<(NodeId, PartitionNumber)>,
    ) {
        (self.tracked_nodes, self.deleted_partitions)
    }

    fn get_tracked_partition(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
    ) -> &mut TrackedPartition {
        self.tracked_nodes
            .entry(*node_id)
            .or_insert(TrackedNode::new(false))
            .tracked_partitions
            .entry(partition_num)
            .or_insert(TrackedPartition::new())
    }

    fn get_tracked_substate<E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_number: PartitionNumber,
        substate_key: SubstateKey,
        on_io_access: &mut F,
    ) -> Result<&mut TrackedSubstateValue, E> {
        let db_sort_key = M::to_db_sort_key(&substate_key);
        let partition = &mut self
            .tracked_nodes
            .entry(*node_id)
            .or_insert(TrackedNode::new(false))
            .tracked_partitions
            .entry(partition_number)
            .or_insert(TrackedPartition::new())
            .substates;
        let entry = partition.entry(db_sort_key.clone());

        match entry {
            Entry::Vacant(e) => {
                if self
                    .transient_substates
                    .is_transient(node_id, partition_number, &substate_key)
                {
                    let tracked = TrackedSubstate {
                        substate_key: substate_key.clone(),
                        substate_value: TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent),
                    };
                    let new_size = Some(tracked.size());
                    e.insert(tracked);

                    on_io_access(IOAccess::TrackSubstateUpdated {
                        canonical_substate_key: CanonicalSubstateKey {
                            node_id: *node_id,
                            partition_number,
                            substate_key,
                        },
                        old_size: None,
                        new_size,
                    })?;
                } else {
                    let db_partition_key = M::to_db_partition_key(node_id, partition_number);
                    let substate_value = Self::get_substate_from_db(
                        self.substate_db,
                        &db_partition_key,
                        &M::to_db_sort_key(&substate_key),
                        on_io_access,
                        CanonicalSubstateKey {
                            node_id: *node_id,
                            partition_number,
                            substate_key: substate_key.clone(),
                        },
                    )?;

                    let new_size;
                    if let Some(value) = substate_value {
                        let tracked = TrackedSubstate {
                            substate_key: substate_key.clone(),
                            substate_value: TrackedSubstateValue::ReadOnly(ReadOnly::Existent(
                                RuntimeSubstate::new(value),
                            )),
                        };
                        new_size = Some(tracked.size());
                        e.insert(tracked);
                    } else {
                        let tracked = TrackedSubstate {
                            substate_key: substate_key.clone(),
                            substate_value: TrackedSubstateValue::ReadOnly(ReadOnly::NonExistent),
                        };
                        new_size = Some(tracked.size());
                        e.insert(tracked);
                    };

                    // Notify upper layer
                    on_io_access(IOAccess::TrackSubstateUpdated {
                        canonical_substate_key: CanonicalSubstateKey {
                            node_id: *node_id,
                            partition_number,
                            substate_key,
                        },
                        old_size: None,
                        new_size,
                    })?;
                }
            }
            Entry::Occupied(..) => {}
        }

        Ok(&mut partition.get_mut(&db_sort_key).unwrap().substate_value)
    }
}

impl<'s, S: SubstateDatabase, M: DatabaseKeyMapper + 'static> CommitableSubstateStore
    for Track<'s, S, M>
{
    fn mark_as_transient(
        &mut self,
        node_id: NodeId,
        partition_num: PartitionNumber,
        substate_key: SubstateKey,
    ) {
        self.transient_substates
            .mark_as_transient(node_id, partition_num, substate_key);
    }

    fn create_node<E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: NodeId,
        node_substates: NodeSubstates,
        on_io_access: &mut F,
    ) -> Result<(), E> {
        let mut tracked_partitions = index_map_new();

        for (partition_number, partition) in node_substates {
            let mut partition_substates = BTreeMap::new();
            for (substate_key, substate_value) in partition {
                let db_sort_key = M::to_db_sort_key(&substate_key);

                let tracked = TrackedSubstate {
                    substate_key: substate_key.clone(),
                    substate_value: TrackedSubstateValue::New(RuntimeSubstate::new(substate_value)),
                };
                let new_size = Some(tracked.size());
                let old_tracked = partition_substates.insert(db_sort_key, tracked);
                assert!(old_tracked.is_none());

                // Notify upper layer
                on_io_access(IOAccess::TrackSubstateUpdated {
                    canonical_substate_key: CanonicalSubstateKey {
                        node_id,
                        partition_number,
                        substate_key,
                    },
                    old_size: None,
                    new_size,
                })?;
            }
            let tracked_partition = TrackedPartition::new_with_substates(partition_substates);
            tracked_partitions.insert(partition_number, tracked_partition);
        }

        self.tracked_nodes.insert(
            node_id,
            TrackedNode {
                tracked_partitions,
                is_new: true,
            },
        );

        Ok(())
    }

    fn get_tracked_substate_info(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
    ) -> TrackedSubstateInfo {
        let db_sort_key = M::to_db_sort_key(substate_key);
        let info = self
            .tracked_nodes
            .get(node_id)
            .and_then(|n| n.tracked_partitions.get(&partition_num))
            .and_then(|p| p.substates.get(&db_sort_key))
            .map(|s| match s.substate_value {
                TrackedSubstateValue::New(..) | TrackedSubstateValue::Garbage => {
                    TrackedSubstateInfo::New
                }
                TrackedSubstateValue::WriteOnly(..)
                | TrackedSubstateValue::ReadExistAndWrite(..)
                | TrackedSubstateValue::ReadNonExistAndWrite(..) => TrackedSubstateInfo::Updated,
                TrackedSubstateValue::ReadOnly(..) => TrackedSubstateInfo::Unmodified,
            })
            .unwrap_or(TrackedSubstateInfo::Unmodified);

        info
    }

    fn get_substate<E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
        on_io_access: &mut F,
    ) -> Result<Option<&IndexedScryptoValue>, E> {
        // Load the substate from state track
        let tracked =
            self.get_tracked_substate(node_id, partition_num, substate_key.clone(), on_io_access)?;

        let value = tracked.get_runtime_substate_mut().map(|v| &v.value);

        Ok(value)
    }

    fn set_substate<E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: NodeId,
        partition_number: PartitionNumber,
        substate_key: SubstateKey,
        substate_value: IndexedScryptoValue,
        on_io_access: &mut F,
    ) -> Result<(), E> {
        let tracked_partition = self
            .tracked_nodes
            .entry(node_id)
            .or_insert(TrackedNode::new(false))
            .tracked_partitions
            .entry(partition_number)
            .or_insert(TrackedPartition::new());
        let db_sort_key = M::to_db_sort_key(&substate_key);
        let entry = tracked_partition.substates.entry(db_sort_key);

        match entry {
            Entry::Vacant(e) => {
                let tracked = TrackedSubstate {
                    substate_key: substate_key.clone(),
                    substate_value: TrackedSubstateValue::WriteOnly(Write::Update(
                        RuntimeSubstate::new(substate_value),
                    )),
                };
                let new_size = Some(tracked.size());
                e.insert(tracked);

                // Notify upper layer
                on_io_access(IOAccess::TrackSubstateUpdated {
                    canonical_substate_key: CanonicalSubstateKey {
                        node_id,
                        partition_number,
                        substate_key,
                    },
                    old_size: None,
                    new_size,
                })?;
            }
            Entry::Occupied(mut e) => {
                let tracked = e.get_mut();

                let old_size = Some(tracked.size());
                tracked.substate_value.set(substate_value);
                let new_size = Some(tracked.size());

                // Notify upper layer
                on_io_access(IOAccess::TrackSubstateUpdated {
                    canonical_substate_key: CanonicalSubstateKey {
                        node_id,
                        partition_number,
                        substate_key,
                    },
                    old_size,
                    new_size,
                })?;
            }
        }

        Ok(())
    }

    fn force_write(
        &mut self,
        node_id: &NodeId,
        partition_num: &PartitionNumber,
        substate_key: &SubstateKey,
    ) {
        let tracked = self
            .get_tracked_substate(
                node_id,
                *partition_num,
                substate_key.clone(),
                &mut |_| -> Result<(), ()> { Err(()) },
            )
            .expect("Should not need to go into store on close substate.");
        let cloned_track = tracked.clone();

        self.force_write_tracked_nodes
            .entry(*node_id)
            .or_insert(TrackedNode {
                tracked_partitions: index_map_new(),
                is_new: false,
            })
            .tracked_partitions
            .entry(*partition_num)
            .or_insert(TrackedPartition::new())
            .substates
            .insert(
                M::to_db_sort_key(&substate_key),
                TrackedSubstate {
                    substate_key: substate_key.clone(),
                    substate_value: cloned_track,
                },
            );
    }

    // Should not use on virtualized substates
    fn remove_substate<E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_number: PartitionNumber,
        substate_key: &SubstateKey,
        on_io_access: &mut F,
    ) -> Result<Option<IndexedScryptoValue>, E> {
        let tracked = self.get_tracked_substate(
            node_id,
            partition_number,
            substate_key.clone(),
            on_io_access,
        )?;

        let old_size = Some(tracked.size());
        let taken = tracked.take();
        let new_size = Some(tracked.size());

        // Notify upper layer
        on_io_access(IOAccess::TrackSubstateUpdated {
            canonical_substate_key: CanonicalSubstateKey {
                node_id: *node_id,
                partition_number,
                substate_key: substate_key.clone(),
            },
            old_size,
            new_size,
        })?;

        Ok(taken)
    }

    fn scan_keys<K: SubstateKeyContent + 'static, E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_number: PartitionNumber,
        limit: u32,
        on_io_access: &mut F,
    ) -> Result<Vec<SubstateKey>, E> {
        let limit: usize = limit.try_into().unwrap();
        let mut items = Vec::new();

        let node_updates = self.tracked_nodes.get(node_id);
        let is_new = node_updates
            .map(|tracked_node| tracked_node.is_new)
            .unwrap_or(false);
        let tracked_partition =
            node_updates.and_then(|n| n.tracked_partitions.get(&partition_number));

        if let Some(tracked_partition) = tracked_partition {
            for (_db_sort_key, tracked_substate) in &tracked_partition.substates {
                if items.len() == limit {
                    return Ok(items);
                }

                // TODO: Check that substate is not write locked, before use outside of native blueprints
                if let Some(_substate) = tracked_substate.substate_value.get() {
                    items.push(tracked_substate.substate_key.clone());
                }
            }
        }

        // Optimization, no need to go into database if the node is just created
        if items.len() == limit || is_new {
            return Ok(items);
        }

        let db_partition_key = M::to_db_partition_key(node_id, partition_number);
        let mut tracked_iter = IterationCountedIter::new(Self::list_entries_from_db::<E, F, K>(
            self.substate_db,
            &db_partition_key,
            on_io_access,
            CanonicalPartition {
                node_id: *node_id,
                partition_number,
            },
        ));

        for result in &mut tracked_iter {
            let (db_sort_key, (substate_key, _substate_value)) = result?;

            if items.len() == limit {
                break;
            }

            if tracked_partition
                .map(|tracked_partition| tracked_partition.substates.contains_key(&db_sort_key))
                .unwrap_or(false)
            {
                continue;
            }

            // TODO: cache read substates in Track (and notify upper layer)

            items.push(substate_key);
        }

        // Update track
        let num_iterations = tracked_iter.num_iterations;
        let tracked_partition = self.get_tracked_partition(node_id, partition_number);
        tracked_partition.range_read = u32::max(tracked_partition.range_read, num_iterations);

        Ok(items)
    }

    fn drain_substates<K: SubstateKeyContent + 'static, E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_number: PartitionNumber,
        limit: u32,
        on_io_access: &mut F,
    ) -> Result<Vec<(SubstateKey, IndexedScryptoValue)>, E> {
        let limit: usize = limit.try_into().unwrap();
        let mut items = Vec::new();

        let node_updates = self.tracked_nodes.get_mut(node_id);
        let is_new = node_updates
            .as_ref()
            .map(|tracked_node| tracked_node.is_new)
            .unwrap_or(false);

        // Check what we've currently got so far without going into database
        let mut tracked_partition =
            node_updates.and_then(|n| n.tracked_partitions.get_mut(&partition_number));
        if let Some(tracked_partition) = tracked_partition.as_mut() {
            for (_db_sort_key, tracked_substate) in tracked_partition.substates.iter_mut() {
                if items.len() == limit {
                    return Ok(items);
                }

                let old_size = Some(tracked_substate.size());
                if let Some(value) = tracked_substate.substate_value.take() {
                    items.push((tracked_substate.substate_key.clone(), value));
                }
                let new_size = Some(tracked_substate.size());

                // Notify upper layer
                on_io_access(IOAccess::TrackSubstateUpdated {
                    canonical_substate_key: CanonicalSubstateKey {
                        node_id: *node_id,
                        partition_number,
                        substate_key: tracked_substate.substate_key.clone(),
                    },
                    old_size,
                    new_size,
                })?;
            }
        }

        // Optimization, no need to go into database if the node is just created
        if items.len() == limit || is_new {
            return Ok(items);
        }

        // Read from database
        let db_partition_key = M::to_db_partition_key(node_id, partition_number);

        let (new_updates, num_iterations) = {
            let mut tracked_iter =
                IterationCountedIter::new(Self::list_entries_from_db::<E, F, K>(
                    self.substate_db,
                    &db_partition_key,
                    on_io_access,
                    CanonicalPartition {
                        node_id: *node_id,
                        partition_number,
                    },
                ));
            let new_updates = {
                let mut new_updates = Vec::new();
                for result in &mut tracked_iter {
                    let (db_sort_key, (substate_key, substate_value)) = result?;

                    if items.len() == limit {
                        break;
                    }

                    if tracked_partition
                        .as_ref()
                        .map(|tracked_partition| {
                            tracked_partition.substates.contains_key(&db_sort_key)
                        })
                        .unwrap_or(false)
                    {
                        continue;
                    }

                    let tracked = TrackedSubstate {
                        substate_key: substate_key.clone(),
                        substate_value: TrackedSubstateValue::ReadExistAndWrite(
                            substate_value.clone(),
                            Write::Delete,
                        ),
                    };
                    new_updates.push((db_sort_key, tracked));
                    items.push((substate_key, substate_value));
                }
                new_updates
            };

            (new_updates, tracked_iter.num_iterations)
        };

        // Update track
        {
            let tracked_partition = self.get_tracked_partition(node_id, partition_number);
            tracked_partition.range_read = u32::max(tracked_partition.range_read, num_iterations);

            for (db_sort_key, tracked_substate) in new_updates {
                let substate_key = tracked_substate.substate_key.clone();
                let new_size = Some(tracked_substate.size());
                let old_size = tracked_partition
                    .substates
                    .insert(db_sort_key, tracked_substate)
                    .map(|x| x.size());

                // Notify upper layer
                on_io_access(IOAccess::TrackSubstateUpdated {
                    canonical_substate_key: CanonicalSubstateKey {
                        node_id: *node_id,
                        partition_number,
                        substate_key,
                    },
                    old_size,
                    new_size,
                })?;
            }
        }

        Ok(items)
    }

    fn scan_sorted_substates<E, F: FnMut(IOAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_number: PartitionNumber,
        limit: u32,
        on_io_access: &mut F,
    ) -> Result<Vec<(SortedKey, IndexedScryptoValue)>, E> {
        // TODO: ensure we abort if any substates are write locked.
        let limit: usize = limit.try_into().unwrap();

        // initialize the track partition, since we will definitely need it: either to read values from it OR to update the `range_read` on it
        let tracked_node = self
            .tracked_nodes
            .entry(node_id.clone())
            .or_insert(TrackedNode::new(false));
        let tracked_partition = tracked_node
            .tracked_partitions
            .entry(partition_number)
            .or_insert(TrackedPartition::new());

        // initialize the "from db" iterator: use `dyn`, since we want to skip it altogether if the node is marked as `is_new` in our track
        let mut db_values_count = 0u32;
        let raw_db_entries: Box<
            dyn Iterator<Item = Result<(DbSortKey, (SubstateKey, IndexedScryptoValue)), E>>,
        > = if tracked_node.is_new {
            Box::new(empty()) // optimization: avoid touching the database altogether
        } else {
            let partition_key = M::to_db_partition_key(node_id, partition_number);
            Box::new(Self::list_entries_from_db::<E, F, SortedKey>(
                self.substate_db,
                &partition_key,
                on_io_access,
                CanonicalPartition {
                    node_id: *node_id,
                    partition_number,
                },
            ))
        };
        let db_read_entries = raw_db_entries.inspect(|_| {
            db_values_count += 1;
        });

        // initialize the "from track" iterator
        let tracked_entry_changes =
            tracked_partition
                .substates
                .iter()
                .map(|(db_sort_key, tracked_substate)| {
                    // TODO: ensure we abort if any substates are write locked.
                    if let Some(value) = tracked_substate.substate_value.get() {
                        (
                            db_sort_key.clone(),
                            Some((tracked_substate.substate_key.clone(), value.clone())),
                        )
                    } else {
                        (db_sort_key.clone(), None)
                    }
                });

        let mut items = Vec::new();
        // construct the composite iterator, which applies changes read from our track on top of db values
        for result in
            OverlayingResultIterator::new(db_read_entries, tracked_entry_changes).take(limit)
        {
            let (_db_sort_key, (substate_key, substate_value)) = result?;
            let sorted_key = match substate_key {
                SubstateKey::Sorted(sorted) => sorted,
                _ => panic!("Should be a sorted key"),
            };
            items.push((sorted_key, substate_value));
        }

        // Use the statistics (gathered by the `.inspect()`s above) to update the track's metadata and to return costing info
        tracked_partition.range_read = u32::max(tracked_partition.range_read, db_values_count);

        // TODO: cache read substates in Track (and notify upper layer)

        Ok(items)
    }

    fn delete_partition(&mut self, node_id: &NodeId, partition_num: PartitionNumber) {
        // This is used for transaction tracker only, for which we don't account for store access.

        self.deleted_partitions.insert((*node_id, partition_num));
    }

    fn get_commit_info(&mut self) -> StoreCommitInfo {
        let mut store_commit = Vec::new();

        for (node_id, node) in &self.tracked_nodes {
            for (partition_number, partition) in &node.tracked_partitions {
                for (db_sort_key, substate) in &partition.substates {
                    if self.transient_substates.is_transient(
                        node_id,
                        *partition_number,
                        &substate.substate_key,
                    ) {
                        continue;
                    }

                    let canonical_substate_key = CanonicalSubstateKey {
                        node_id: *node_id,
                        partition_number: *partition_number,
                        substate_key: substate.substate_key.clone(),
                    };

                    match &substate.substate_value {
                        TrackedSubstateValue::New(v) => {
                            store_commit.push(StoreCommit::Insert {
                                canonical_substate_key,
                                size: v.value.len(),
                            });
                        }
                        TrackedSubstateValue::ReadOnly(_) => {
                            // No op
                        }
                        TrackedSubstateValue::ReadExistAndWrite(old_value, write) => match write {
                            Write::Update(x) => {
                                store_commit.push(StoreCommit::Update {
                                    canonical_substate_key,
                                    size: x.value.len(),
                                    old_size: old_value.len(),
                                });
                            }
                            Write::Delete => {
                                store_commit.push(StoreCommit::Delete {
                                    canonical_substate_key,
                                    old_size: old_value.len(),
                                });
                            }
                        },
                        TrackedSubstateValue::ReadNonExistAndWrite(value) => {
                            store_commit.push(StoreCommit::Insert {
                                canonical_substate_key,
                                size: value.value.len(),
                            });
                        }
                        TrackedSubstateValue::WriteOnly(write) => {
                            let old_size = self
                                .substate_db
                                .get_substate(
                                    &M::to_db_partition_key(node_id, *partition_number),
                                    db_sort_key,
                                )
                                .map(|x| x.len());

                            match (old_size, write) {
                                (Some(old_size), Write::Update(x)) => {
                                    store_commit.push(StoreCommit::Update {
                                        canonical_substate_key,
                                        size: x.value.len(),
                                        old_size,
                                    });
                                }
                                (Some(old_size), Write::Delete) => {
                                    store_commit.push(StoreCommit::Delete {
                                        canonical_substate_key,
                                        old_size,
                                    });
                                }
                                (None, Write::Update(x)) => {
                                    store_commit.push(StoreCommit::Insert {
                                        canonical_substate_key,
                                        size: x.value.len(),
                                    });
                                }
                                (None, Write::Delete) => {
                                    // TODO: this should never happen?
                                }
                            }
                        }
                        TrackedSubstateValue::Garbage => {
                            // No op
                        }
                    }
                }
            }
        }

        store_commit
    }
}
