use crate::utils::{crypto::*, num::is_even};

pub struct Node<T> {
    pub value: T,
}

impl<T> Clone for Node<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
        }
    }
}

type MKNode = Node<Hash>;

pub struct CompactMerkleTree<H: Hasher> {
    pub leaves: Vec<MKNode>,
    pub root_hash: Hash,
    pub hasher: H,
}

impl<H: Hasher> CompactMerkleTree<H> {
    pub fn create<T: HashableData>(data: &[T], hasher: H) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let leaves = CompactMerkleTree::create_leaves_from(data, &hasher);
        let root_hash = CompactMerkleTree::calculate_root(&leaves, &hasher);

        Some(Self {
            leaves,
            root_hash,
            hasher,
        })
    }

    fn create_leaves_from<T: HashableData>(data: &[T], hasher: &H) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node {
                value: hasher.get_hash_from_data(el),
            })
            .collect()
    }

    fn calculate_root(leaves: &[MKNode], hasher: &H) -> Hash {
        let mut nodes = leaves.to_vec();

        while nodes.len() > 1 {
            nodes = CompactMerkleTree::get_parent_nodes(&nodes, hasher);
        }

        nodes[0].value.clone()
    }

    fn get_parent_nodes(nodes: &[MKNode], hasher: &H) -> Vec<MKNode> {
        nodes
            .chunks(2)
            .map(|chunk| match chunk {
                [a, b] => Node {
                    value: hasher.get_combined_hash(&a.value, &b.value),
                },
                [a] => Node {
                    value: hasher.get_combined_hash(&a.value, &a.value),
                },
                _ => panic!("Unexpected chunk size in get_parent_nodes"),
            })
            .collect()
    }

    pub fn get_leaf_by_idx(&self, idx: usize) -> Option<MKNode> {
        self.leaves.get(idx).cloned()
    }

    pub fn get_leaf_by_hash(&self, hash: Hash) -> Option<MKNode> {
        self.leaves.iter().find(|el| el.value == hash).cloned()
    }

    pub fn add_leaf<T: HashableData>(&mut self, data: T) {
        let hash = self.hasher.get_hash_from_data(data);
        self.leaves.push(Node { value: hash });
        self.rebuild_root();
    }

    pub fn delete_leaf(&mut self, index: usize) {
        if self.leaves.get(index).is_some() {
            self.leaves.remove(index);
            self.rebuild_root();
        }
    }

    pub fn update_leaf<T: HashableData>(&mut self, index: usize, data: T) {
        if let Some(node) = self.leaves.get_mut(index) {
            node.value = self.hasher.get_hash_from_data(data);
            self.rebuild_root();
        }
    }

    fn rebuild_root(&mut self) {
        self.root_hash = CompactMerkleTree::calculate_root(&self.leaves, &self.hasher);
    }

    pub fn gen_proof(&self, mut leaf_idx: usize) -> Result<Vec<Hash>, &str> {
        let mut proof: Vec<Hash> = Vec::new();

        if self.leaves.get(leaf_idx).is_none() {
            return Err("No leaf exists with the given index");
        }

        let mut nodes = self.leaves.clone();

        while nodes.len() > 1 {
            let sibling_idx = if is_even(leaf_idx) {
                leaf_idx + 1
            } else {
                leaf_idx - 1
            };

            let sibling_value = nodes
                .get(sibling_idx)
                .map(|node| node.value.clone())
                .unwrap_or(nodes[leaf_idx].value.clone());

            proof.push(sibling_value);
            nodes = CompactMerkleTree::get_parent_nodes(&nodes, &self.hasher);
            leaf_idx /= 2;
        }

        Ok(proof)
    }

    pub fn verify_proof(&self, leaf_hash: &Hash, mut leaf_idx: usize, proof: Vec<Hash>) -> bool {
        let mut leaf_hash = leaf_hash.clone();
        for hash in proof {
            if is_even(leaf_idx) {
                leaf_hash = self.hasher.get_combined_hash(&leaf_hash, &hash);
            } else {
                leaf_hash = self.hasher.get_combined_hash(&hash, &leaf_hash);
            }
            leaf_idx /= 2;
        }
        leaf_hash == self.root_hash
    }

    pub fn contains_hash(&self, hash: &Hash) -> Option<(usize, Vec<Hash>)> {
        let leaf_index = self.leaves.iter().position(|el| el.value == *hash)?;

        let proof = self.gen_proof(leaf_index).unwrap();
        Some((leaf_index, proof))
    }
}
