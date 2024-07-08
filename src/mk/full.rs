use crate::tree::*;
use crate::utils::crypto::*;
use crate::utils::num;
use std::rc::Rc;

type MKNode = TreeNode<Hash>;
pub struct FullMerkleTree {
    pub tree: MKNode,
    pub leaves: Vec<MKNode>,
    pub root_hash: Hash,
}

impl FullMerkleTree {
    pub fn create<T: DataToHash>(data: &[T]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        let leaves = FullMerkleTree::create_leaves_from(data);

        let tree = FullMerkleTree::create_tree(leaves.clone());
        let root_hash = tree.borrow().value;

        Some(Self {
            tree,
            root_hash,
            leaves,
        })
    }

    fn create_leaves_from<T: DataToHash>(data: &[T]) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node::new(get_hash_from_data(el), None, None, None))
            .collect()
    }

    fn create_tree(mut leaves: Vec<MKNode>) -> MKNode {
        while leaves.len() != 1 {
            leaves = leaves
                .chunks(2)
                .map(|el| match el {
                    [a, b] => Self::create_node(a, b),
                    // hash with itself
                    [a] => Self::create_node(a, &Node::<Hash>::clone(a)),
                    _ => panic!("unexpected chunk size"),
                })
                .collect();
        }

        return leaves.get(0).unwrap().to_owned();
    }

    fn create_node(a: &MKNode, b: &MKNode) -> MKNode {
        let node = Node::new(
            get_combined_hash(a.borrow().value, b.borrow().value),
            Some(vec![Rc::clone(a), Rc::clone(b)]),
            None,
            None,
        );
        Node::set_parent_and_siblings(&node, &[a, b]);

        return node;
    }

    pub fn get_leaf_by_idx(&self, idx: usize) -> Option<MKNode> {
        self.leaves.get(idx).and_then(|el| Some(el.clone()))
    }

    pub fn get_leaf_by_hash(&self, hash: Hash) -> Option<MKNode> {
        self.leaves
            .iter()
            .find(|el| el.borrow().value == hash)
            .and_then(|el| Some(el.clone()))
    }

    pub fn add_leaf<T: DataToHash>(&mut self, data: T) {
        let hash = get_hash_from_data(data);
        let node = Node::new(hash, None, None, None);
        self.leaves.push(node);
        self.rebuild_tree();
    }

    pub fn delete_leaf(&mut self, index: usize) {
        if let Some(_) = self.leaves.get(index) {
            self.leaves.remove(index);
            self.rebuild_tree();
        }
    }

    pub fn update_leaf<T: DataToHash>(&mut self, index: usize, data: T) {
        if let Some(node) = self.leaves.get(index) {
            node.borrow_mut().value = get_hash_from_data(data);
            self.rebuild_tree();
        }
    }

    fn rebuild_tree(&mut self) {
        let tree = FullMerkleTree::create_tree(self.leaves.clone());
        let root_hash = tree.borrow().value;
        self.tree = tree;
        self.root_hash = root_hash;
    }

    pub fn gen_proof(&self, leaf_idx: usize) -> Result<Vec<Hash>, &str> {
        let mut proof: Vec<Hash> = Vec::new();
        let mut current_node = match self.leaves.get(leaf_idx) {
            Some(node) => node.clone(),
            None => return Err("No leaf exists with the given index"),
        };

        loop {
            let sibling = current_node.borrow().get_sibling(0);
            // this means we've reached the root node
            if sibling.is_none() {
                break;
            }
            proof.push(sibling.unwrap().borrow().value);
            // if it has a sibling, then it must have a parent
            let parent_node = current_node.borrow().get_parent().unwrap();
            current_node = parent_node;
        }

        return Ok(proof);
    }

    pub fn verify_proof(&self, mut leaf_hash: Hash, mut leaf_idx: usize, proof: Vec<Hash>) -> bool {
        for hash in proof {
            if num::is_even(leaf_idx) {
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
            .find(|(_, el)| el.borrow().value == hash);
        if leaf.is_none() {
            return None;
        };
        let leaf_idx = leaf.unwrap().0;
        return Some((leaf_idx, self.gen_proof(leaf_idx).unwrap()));
    }
}

impl<T: AsRef<[u8]>> From<&[T]> for FullMerkleTree {
    fn from(value: &[T]) -> Self {
        FullMerkleTree::create(value).expect("data can't be empty")
    }
}
