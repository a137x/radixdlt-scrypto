//! This file contains a large part of Aptos' jellyfish-merkle/src/lib.rs source
//! file (which implements a core of a JMT logic) with a couple of manual
//! adjustments (aimed at reducing dependency on Aptos and adjusting the JMT for
//! our purposes - still a work in progress).
//! Each non-trivial adjustment is annotated with an "ADJUSTMENT: ..." comment.
//! Please see types.rs for other copied-from-Aptos dependencies of this file.

use super::types::{
    Child, Children, InternalNode, IteratedHash, LeafNode, Nibble, NibblePath, Node, NodeKey,
    SparseMerkleProof, SparseMerkleProofExt, SparseMerkleRangeProof, TreeReader, Version,
    SPARSE_MERKLE_PLACEHOLDER_HASH,
};
use radix_engine_interface::crypto::Hash;
use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;

// ADJUSTMENT: the original used a known key size (32) as a limit
const SANITY_NIBBLE_LIMIT: usize = 1000;

/// The Jellyfish Merkle tree data structure. See [`crate`] for description.
pub struct JellyfishMerkleTree<'a, R, K> {
    reader: &'a R,
    phantom_value: PhantomData<K>,
}

impl<'a, R: 'a + TreeReader<K>, K: Clone> JellyfishMerkleTree<'a, R, K> {
    /// Creates a `JellyfishMerkleTree` backed by the given [`TreeReader`](trait.TreeReader.html).
    pub fn new(reader: &'a R) -> Self {
        Self {
            reader,
            phantom_value: PhantomData,
        }
    }

    /// Get the node hash from the cache if cache is provided, otherwise (for test only) compute it.
    fn get_hash(
        node_key: &NodeKey,
        node: &Node<K>,
        hash_cache: &Option<&HashMap<NibblePath, Hash>>,
    ) -> Hash {
        if let Some(cache) = hash_cache {
            match cache.get(node_key.nibble_path()) {
                Some(hash) => *hash,
                None => unreachable!("{:?} can not be found in hash cache", node_key),
            }
        } else {
            node.hash()
        }
    }

    /// For each value set:
    /// Returns the new nodes and values in a batch after applying `value_set`. For
    /// example, if after transaction `T_i` the committed state of tree in the persistent storage
    /// looks like the following structure:
    ///
    /// ```text
    ///              S_i
    ///             /   \
    ///            .     .
    ///           .       .
    ///          /         \
    ///         o           x
    ///        / \
    ///       A   B
    ///        storage (disk)
    /// ```
    ///
    /// where `A` and `B` denote the states of two adjacent accounts, and `x` is a sibling subtree
    /// of the path from root to A and B in the tree. Then a `value_set` produced by the next
    /// transaction `T_{i+1}` modifies other accounts `C` and `D` exist in the subtree under `x`, a
    /// new partial tree will be constructed in memory and the structure will be:
    ///
    /// ```text
    ///                 S_i      |      S_{i+1}
    ///                /   \     |     /       \
    ///               .     .    |    .         .
    ///              .       .   |   .           .
    ///             /         \  |  /             \
    ///            /           x | /               x'
    ///           o<-------------+-               / \
    ///          / \             |               C   D
    ///         A   B            |
    ///           storage (disk) |    cache (memory)
    /// ```
    ///
    /// With this design, we are able to query the global state in persistent storage and
    /// generate the proposed tree delta based on a specific root hash and `value_set`. For
    /// example, if we want to execute another transaction `T_{i+1}'`, we can use the tree `S_i` in
    /// storage and apply the `value_set` of transaction `T_{i+1}`. Then if the storage commits
    /// the returned batch, the state `S_{i+1}` is ready to be read from the tree by calling
    /// [`get_with_proof`](struct.JellyfishMerkleTree.html#method.get_with_proof). Anything inside
    /// the batch is not reachable from public interfaces before being committed.
    pub fn batch_put_value_set(
        &self,
        value_set: Vec<(Hash, Option<&(Hash, K)>)>,
        node_hashes: Option<&HashMap<NibblePath, Hash>>,
        persisted_version: Option<Version>,
        version: Version,
    ) -> Result<(Hash, TreeUpdateBatch<K>), Error> {
        let deduped_and_sorted_kvs = value_set
            .into_iter()
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect::<Vec<_>>();

        let mut batch = TreeUpdateBatch::new();
        let root_node_opt = if let Some(persisted_version) = persisted_version {
            self.batch_insert_at(
                &NodeKey::new_empty_path(persisted_version),
                version,
                deduped_and_sorted_kvs.as_slice(),
                0,
                &node_hashes,
                &mut batch,
            )
        } else {
            self.batch_update_subtree(
                &NodeKey::new_empty_path(version),
                version,
                deduped_and_sorted_kvs.as_slice(),
                0,
                &node_hashes,
                &mut batch,
            )
        }?;

        let node_key = NodeKey::new_empty_path(version);
        let root_hash = if let Some(root_node) = root_node_opt {
            let hash = root_node.hash();
            batch.put_node(node_key, root_node);
            hash
        } else {
            batch.put_node(node_key, Node::Null);
            SPARSE_MERKLE_PLACEHOLDER_HASH
        };

        Ok((root_hash, batch))
    }

    fn batch_insert_at(
        &self,
        node_key: &NodeKey,
        version: Version,
        kvs: &[(Hash, Option<&(Hash, K)>)],
        depth: usize,
        hash_cache: &Option<&HashMap<NibblePath, Hash>>,
        batch: &mut TreeUpdateBatch<K>,
    ) -> Result<Option<Node<K>>, Error> {
        let node = self.reader.get_node(node_key)?;
        batch.put_stale_node(node_key.clone(), version, &node);

        match node {
            Node::Internal(internal_node) => {
                // There is a small possibility that the old internal node is intact.
                // Traverse all the path touched by `kvs` from this internal node.
                let range_iter = NibbleRangeIterator::new(kvs, depth);
                // ADJUSTMENT: there was a par_iter (conditionally) used here
                let new_children: Vec<_> = range_iter
                    .map(|(left, right)| {
                        self.insert_at_child(
                            node_key,
                            &internal_node,
                            version,
                            kvs,
                            left,
                            right,
                            depth,
                            hash_cache,
                            batch,
                        )
                    })
                    .collect::<Result<_, Error>>()?;

                // Reuse the current `InternalNode` in memory to create a new internal node.
                let mut old_children: Children = internal_node.into();
                let mut new_created_children: HashMap<Nibble, Node<K>> = HashMap::new();
                for (child_nibble, child_option) in new_children {
                    if let Some(child) = child_option {
                        new_created_children.insert(child_nibble, child);
                    } else {
                        old_children.remove(&child_nibble);
                    }
                }

                if old_children.is_empty() && new_created_children.is_empty() {
                    return Ok(None);
                } else if old_children.len() <= 1 && new_created_children.len() <= 1 {
                    if let Some((new_nibble, new_child)) = new_created_children.iter().next() {
                        if let Some((old_nibble, _old_child)) = old_children.iter().next() {
                            if old_nibble == new_nibble && new_child.is_leaf() {
                                return Ok(Some(new_child.clone()));
                            }
                        } else if new_child.is_leaf() {
                            return Ok(Some(new_child.clone()));
                        }
                    } else {
                        let (old_child_nibble, old_child) =
                            old_children.iter().next().expect("must exist");
                        if old_child.is_leaf() {
                            let old_child_node_key =
                                node_key.gen_child_node_key(old_child.version, *old_child_nibble);
                            let old_child_node = self.reader.get_node(&old_child_node_key)?;
                            batch.put_stale_node(old_child_node_key, version, &old_child_node);
                            return Ok(Some(old_child_node));
                        }
                    }
                }

                let mut new_children = old_children;
                for (child_index, new_child_node) in new_created_children {
                    let new_child_node_key = node_key.gen_child_node_key(version, child_index);
                    new_children.insert(
                        child_index,
                        Child::new(
                            Self::get_hash(&new_child_node_key, &new_child_node, hash_cache),
                            version,
                            new_child_node.node_type(),
                        ),
                    );
                    batch.put_node(new_child_node_key, new_child_node);
                }
                let new_internal_node = InternalNode::new(new_children);
                Ok(Some(new_internal_node.into()))
            }
            Node::Leaf(leaf_node) => self.batch_update_subtree_with_existing_leaf(
                node_key, version, leaf_node, kvs, depth, hash_cache, batch,
            ),
            Node::Null => {
                assert_eq!(depth, 0, "Null node can only exist at depth 0");
                self.batch_update_subtree(node_key, version, kvs, 0, hash_cache, batch)
            }
        }
    }

    fn insert_at_child(
        &self,
        node_key: &NodeKey,
        internal_node: &InternalNode,
        version: Version,
        kvs: &[(Hash, Option<&(Hash, K)>)],
        left: usize,
        right: usize,
        depth: usize,
        hash_cache: &Option<&HashMap<NibblePath, Hash>>,
        batch: &mut TreeUpdateBatch<K>,
    ) -> Result<(Nibble, Option<Node<K>>), Error> {
        let child_index = kvs[left].0.get_nibble(depth);
        let child = internal_node.child(child_index);

        let new_child_node_option = match child {
            Some(child) => self.batch_insert_at(
                &node_key.gen_child_node_key(child.version, child_index),
                version,
                &kvs[left..=right],
                depth + 1,
                hash_cache,
                batch,
            )?,
            None => self.batch_update_subtree(
                &node_key.gen_child_node_key(version, child_index),
                version,
                &kvs[left..=right],
                depth + 1,
                hash_cache,
                batch,
            )?,
        };

        Ok((child_index, new_child_node_option))
    }

    fn batch_update_subtree_with_existing_leaf(
        &self,
        node_key: &NodeKey,
        version: Version,
        existing_leaf_node: LeafNode<K>,
        kvs: &[(Hash, Option<&(Hash, K)>)],
        depth: usize,
        hash_cache: &Option<&HashMap<NibblePath, Hash>>,
        batch: &mut TreeUpdateBatch<K>,
    ) -> Result<Option<Node<K>>, Error> {
        let existing_leaf_key = existing_leaf_node.account_key();

        if kvs.len() == 1 && kvs[0].0 == existing_leaf_key {
            if let (key, Some((value_hash, state_key))) = kvs[0] {
                let new_leaf_node = Node::new_leaf(key, *value_hash, (state_key.clone(), version));
                Ok(Some(new_leaf_node))
            } else {
                Ok(None)
            }
        } else {
            let existing_leaf_bucket = existing_leaf_key.get_nibble(depth);
            let mut isolated_existing_leaf = true;
            let mut children = vec![];
            for (left, right) in NibbleRangeIterator::new(kvs, depth) {
                let child_index = kvs[left].0.get_nibble(depth);
                let child_node_key = node_key.gen_child_node_key(version, child_index);
                if let Some(new_child_node) = if existing_leaf_bucket == child_index {
                    isolated_existing_leaf = false;
                    self.batch_update_subtree_with_existing_leaf(
                        &child_node_key,
                        version,
                        existing_leaf_node.clone(),
                        &kvs[left..=right],
                        depth + 1,
                        hash_cache,
                        batch,
                    )?
                } else {
                    self.batch_update_subtree(
                        &child_node_key,
                        version,
                        &kvs[left..=right],
                        depth + 1,
                        hash_cache,
                        batch,
                    )?
                } {
                    children.push((child_index, new_child_node));
                }
            }
            if isolated_existing_leaf {
                children.push((existing_leaf_bucket, existing_leaf_node.into()));
            }

            if children.is_empty() {
                Ok(None)
            } else if children.len() == 1 && children[0].1.is_leaf() {
                let (_, child) = children.pop().expect("Must exist");
                Ok(Some(child))
            } else {
                let new_internal_node = InternalNode::new(
                    children
                        .into_iter()
                        .map(|(child_index, new_child_node)| {
                            let new_child_node_key =
                                node_key.gen_child_node_key(version, child_index);
                            let result = (
                                child_index,
                                Child::new(
                                    Self::get_hash(
                                        &new_child_node_key,
                                        &new_child_node,
                                        hash_cache,
                                    ),
                                    version,
                                    new_child_node.node_type(),
                                ),
                            );
                            batch.put_node(new_child_node_key, new_child_node);
                            result
                        })
                        .collect(),
                );
                Ok(Some(new_internal_node.into()))
            }
        }
    }

    fn batch_update_subtree(
        &self,
        node_key: &NodeKey,
        version: Version,
        kvs: &[(Hash, Option<&(Hash, K)>)],
        depth: usize,
        hash_cache: &Option<&HashMap<NibblePath, Hash>>,
        batch: &mut TreeUpdateBatch<K>,
    ) -> Result<Option<Node<K>>, Error> {
        if kvs.len() == 1 {
            if let (key, Some((value_hash, state_key))) = kvs[0] {
                let new_leaf_node = Node::new_leaf(key, *value_hash, (state_key.clone(), version));
                Ok(Some(new_leaf_node))
            } else {
                Ok(None)
            }
        } else {
            let mut children = vec![];
            for (left, right) in NibbleRangeIterator::new(kvs, depth) {
                let child_index = kvs[left].0.get_nibble(depth);
                let child_node_key = node_key.gen_child_node_key(version, child_index);
                if let Some(new_child_node) = self.batch_update_subtree(
                    &child_node_key,
                    version,
                    &kvs[left..=right],
                    depth + 1,
                    hash_cache,
                    batch,
                )? {
                    children.push((child_index, new_child_node))
                }
            }
            if children.is_empty() {
                Ok(None)
            } else if children.len() == 1 && children[0].1.is_leaf() {
                let (_, child) = children.pop().expect("Must exist");
                Ok(Some(child))
            } else {
                let new_internal_node = InternalNode::new(
                    children
                        .into_iter()
                        .map(|(child_index, new_child_node)| {
                            let new_child_node_key =
                                node_key.gen_child_node_key(version, child_index);
                            let result = (
                                child_index,
                                Child::new(
                                    Self::get_hash(
                                        &new_child_node_key,
                                        &new_child_node,
                                        hash_cache,
                                    ),
                                    version,
                                    new_child_node.node_type(),
                                ),
                            );
                            batch.put_node(new_child_node_key, new_child_node);
                            result
                        })
                        .collect(),
                );
                Ok(Some(new_internal_node.into()))
            }
        }
    }

    /// Returns the value (if applicable) and the corresponding merkle proof.
    pub fn get_with_proof(
        &self,
        key: Hash,
        version: Version,
    ) -> Result<(Option<(Hash, (K, Version))>, SparseMerkleProof), Error> {
        self.get_with_proof_ext(key, version)
            .map(|(value, proof_ext)| (value, proof_ext.into()))
    }

    pub fn get_with_proof_ext(
        &self,
        key: Hash,
        version: Version,
    ) -> Result<(Option<(Hash, (K, Version))>, SparseMerkleProofExt), Error> {
        // Empty tree just returns proof with no sibling hash.
        let mut next_node_key = NodeKey::new_empty_path(version);
        let mut siblings = vec![];
        let nibble_path = NibblePath::new_even(key.to_vec());
        let mut nibble_iter = nibble_path.nibbles();

        for _nibble_depth in 0..SANITY_NIBBLE_LIMIT {
            let next_node = self.reader.get_node(&next_node_key)?;
            match next_node {
                Node::Internal(internal_node) => {
                    let queried_child_index = nibble_iter
                        .next()
                        .ok_or_else(|| Error::new(ErrorKind::NotFound, "ran out of nibbles"))?;
                    let (child_node_key, mut siblings_in_internal) = internal_node
                        .get_child_with_siblings(
                            &next_node_key,
                            queried_child_index,
                            Some(self.reader),
                        )?;
                    siblings.append(&mut siblings_in_internal);
                    next_node_key = match child_node_key {
                        Some(node_key) => node_key,
                        None => {
                            return Ok((
                                None,
                                SparseMerkleProofExt::new(None, {
                                    siblings.reverse();
                                    siblings
                                }),
                            ))
                        }
                    };
                }
                Node::Leaf(leaf_node) => {
                    return Ok((
                        if leaf_node.account_key() == key {
                            Some((leaf_node.value_hash(), leaf_node.value_index().clone()))
                        } else {
                            None
                        },
                        SparseMerkleProofExt::new(Some(leaf_node.into()), {
                            siblings.reverse();
                            siblings
                        }),
                    ));
                }
                Node::Null => {
                    return Ok((None, SparseMerkleProofExt::new(None, vec![])));
                }
            }
        }
        Err(Error::new(ErrorKind::InvalidData, "cycle in the tree"))
    }

    /// Gets the proof that shows a list of keys up to `rightmost_key_to_prove` exist at `version`.
    pub fn get_range_proof(
        &self,
        rightmost_key_to_prove: Hash,
        version: Version,
    ) -> Result<SparseMerkleRangeProof, Error> {
        let (account, proof) = self.get_with_proof(rightmost_key_to_prove, version)?;
        assert!(account.is_some(), "rightmost_key_to_prove must exist.");

        let siblings = proof
            .siblings()
            .iter()
            .rev()
            .zip(rightmost_key_to_prove.iter_bits())
            .filter_map(|(sibling, bit)| {
                // We only need to keep the siblings on the right.
                if !bit {
                    Some(*sibling)
                } else {
                    None
                }
            })
            .rev()
            .collect();
        Ok(SparseMerkleRangeProof::new(siblings))
    }

    fn get_root_node(&self, version: Version) -> Result<Node<K>, Error> {
        self.get_root_node_option(version)?.ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                format!("Root node not found for version {}", version),
            )
        })
    }

    fn get_root_node_option(&self, version: Version) -> Result<Option<Node<K>>, Error> {
        let root_node_key = NodeKey::new_empty_path(version);
        self.reader.get_node_option(&root_node_key)
    }

    pub fn get_root_hash(&self, version: Version) -> Result<Hash, Error> {
        self.get_root_node(version).map(|n| n.hash())
    }

    pub fn get_leaf_count(&self, version: Version) -> Result<usize, Error> {
        self.get_root_node(version).map(|n| n.leaf_count())
    }

    pub fn get_all_nodes_referenced(&self, version: Version) -> Result<Vec<NodeKey>, Error> {
        let mut out_keys = vec![];
        self.get_all_nodes_referenced_impl(NodeKey::new_empty_path(version), &mut out_keys)?;
        Ok(out_keys)
    }

    fn get_all_nodes_referenced_impl(
        &self,
        key: NodeKey,
        out_keys: &mut Vec<NodeKey>,
    ) -> Result<(), Error> {
        match self.reader.get_node(&key)? {
            Node::Internal(internal_node) => {
                for (child_nibble, child) in internal_node.children_sorted() {
                    self.get_all_nodes_referenced_impl(
                        key.gen_child_node_key(child.version, *child_nibble),
                        out_keys,
                    )?;
                }
            }
            Node::Leaf(_) | Node::Null => {}
        };

        out_keys.push(key);
        Ok(())
    }
}

/// An iterator that iterates the index range (inclusive) of each different nibble at given
/// `nibble_idx` of all the keys in a sorted key-value pairs which have the identical Hash
/// prefix (up to nibble_idx).
struct NibbleRangeIterator<'a, K> {
    sorted_kvs: &'a [(Hash, K)],
    nibble_idx: usize,
    pos: usize,
}

impl<'a, K> NibbleRangeIterator<'a, K> {
    fn new(sorted_kvs: &'a [(Hash, K)], nibble_idx: usize) -> Self {
        NibbleRangeIterator {
            sorted_kvs,
            nibble_idx,
            pos: 0,
        }
    }
}

impl<'a, K> Iterator for NibbleRangeIterator<'a, K> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.pos;
        if self.pos < self.sorted_kvs.len() {
            let cur_nibble = self.sorted_kvs[left].0.get_nibble(self.nibble_idx);
            let (mut i, mut j) = (left, self.sorted_kvs.len() - 1);
            // Find the last index of the cur_nibble.
            while i < j {
                let mid = j - (j - i) / 2;
                if self.sorted_kvs[mid].0.get_nibble(self.nibble_idx) > cur_nibble {
                    j = mid - 1;
                } else {
                    i = mid;
                }
            }
            self.pos = i + 1;
            Some((left, i))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TreeUpdateBatch<K> {
    pub node_batch: Vec<Vec<(NodeKey, Node<K>)>>,
    pub stale_node_index_batch: Vec<Vec<StaleNodeIndex>>,
    pub num_new_leaves: usize,
    pub num_stale_leaves: usize,
}

impl<K: Clone> TreeUpdateBatch<K> {
    pub fn new() -> Self {
        Self {
            node_batch: vec![vec![]],
            stale_node_index_batch: vec![vec![]],
            num_new_leaves: 0,
            num_stale_leaves: 0,
        }
    }

    fn inc_num_new_leaves(&mut self) {
        self.num_new_leaves += 1;
    }

    fn inc_num_stale_leaves(&mut self) {
        self.num_stale_leaves += 1;
    }

    pub fn put_node(&mut self, node_key: NodeKey, node: Node<K>) {
        if node.is_leaf() {
            self.inc_num_new_leaves();
        }
        self.node_batch[0].push((node_key, node))
    }

    pub fn put_stale_node(
        &mut self,
        node_key: NodeKey,
        stale_since_version: Version,
        node: &Node<K>,
    ) {
        if node.is_leaf() {
            self.inc_num_stale_leaves();
        }
        self.stale_node_index_batch[0].push(StaleNodeIndex {
            node_key,
            stale_since_version,
        });
    }
}

/// Indicates a node becomes stale since `stale_since_version`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StaleNodeIndex {
    /// The version since when the node is overwritten and becomes stale.
    pub stale_since_version: Version,
    /// The [`NodeKey`](node_type/struct.NodeKey.html) identifying the node associated with this
    /// record.
    pub node_key: NodeKey,
}
