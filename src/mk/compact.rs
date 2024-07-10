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

pub struct CompactMerkleTree {
    pub leaves: Vec<MKNode>,
    pub root_hash: Hash,
}

impl CompactMerkleTree {
    fn create<T: DataToHash>(data: &[T]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        let leaves: Vec<Node<[u8; 64]>> = Self::create_leaves_from(data);
        let root_hash = Self::calculate_root(leaves.clone());
        Some(Self { leaves, root_hash })
    }

    fn create_leaves_from<T: DataToHash>(data: &[T]) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node {
                value: get_hash_from_data(el),
            })
            .collect()
    }

    fn calculate_root(mut leaves: Vec<MKNode>) -> Hash {
        while leaves.len() > 1 {
            leaves = Self::get_parent_nodes(&leaves);
        }

        // there has to be a first, otherwise the while would keep running
        return leaves.first().unwrap().value;
    }

    pub fn get_root_hash(&self) -> Hash {
        self.root_hash
    }

    fn get_parent_nodes(nodes: &[MKNode]) -> Vec<MKNode> {
        nodes
            .chunks(2)
            .map(|leaf| match leaf {
                [a, b] => Node {
                    value: get_combined_hash(a.value, b.value),
                },
                [a] => Node {
                    value: get_combined_hash(a.value, a.value),
                },
                _ => panic!(),
            })
            .collect()
    }

    pub fn get_leaf_by_idx(&self, idx: usize) -> Option<MKNode> {
        self.leaves.get(idx).cloned()
    }

    pub fn get_leaf_by_hash(&self, hash: Hash) -> Option<MKNode> {
        self.leaves.iter().find(|el| el.value == hash).cloned()
    }

    pub fn add_leaf<T: DataToHash>(&mut self, data: T) {
        let hash = get_hash_from_data(data);
        self.leaves.push(Node { value: hash });
        self.rebuild_root();
    }

    pub fn delete_leaf(&mut self, index: usize) {
        if self.leaves.get(index).is_some() {
            self.leaves.remove(index);
            self.rebuild_root();
        }
    }

    pub fn update_leaf<T: DataToHash>(&mut self, index: usize, data: T) {
        if let Some(node) = self.leaves.get_mut(index) {
            node.value = get_hash_from_data(data);
            self.rebuild_root();
        }
    }

    fn rebuild_root(&mut self) {
        let root_hash = Self::calculate_root(self.leaves.clone());
        self.root_hash = root_hash;
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
            let mut sibling = nodes.get(sibling_idx);

            // it needs to hash with itself
            if sibling.is_none() {
                sibling = nodes.get(leaf_idx);
            }

            proof.push(sibling.unwrap().value);
            nodes = Self::get_parent_nodes(&nodes);
            leaf_idx /= 2;
        }

        Ok(proof)
    }

    pub fn verify_proof(&self, mut leaf_hash: Hash, mut leaf_idx: usize, proof: Vec<Hash>) -> bool {
        for hash in proof {
            if is_even(leaf_idx) {
                leaf_hash = get_combined_hash(leaf_hash, hash);
            } else {
                leaf_hash = get_combined_hash(hash, leaf_hash);
            }
            leaf_idx /= 2;
        }
        leaf_hash == self.root_hash
    }

    pub fn contains_hash(&self, hash: Hash) -> Option<(usize, Vec<Hash>)> {
        let leaf = self
            .leaves
            .iter()
            .enumerate()
            .find(|(_, el)| el.value == hash);
        let leaf_idx = leaf?.0;
        // if the leaf exists then it must have a proof
        return Some((leaf_idx, self.gen_proof(leaf_idx).unwrap()));
    }
}

impl<T: AsRef<[u8]>> TryFrom<&[T]> for CompactMerkleTree {
    type Error = &'static str;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        match CompactMerkleTree::create(value) {
            Some(mk) => Ok(mk),
            None => Err("data can't be empty"),
        }
    }
}
