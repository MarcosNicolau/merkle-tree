use crate::tree::*;
use crate::utils::crypto::*;
use crate::utils::num;
use std::rc::Rc;

use super::mk::MerkleTree;

type MKNode = TreeNode<Hash>;
pub struct FullMerkleTree<H: Hasher> {
    pub hasher: H,
    pub tree: MKNode,
    pub leaves: Vec<MKNode>,
    pub root_hash: Hash,
}

impl<H: Hasher> FullMerkleTree<H> {
    pub fn create<T: HashableData>(data: &[T], hasher: H) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        let leaves = FullMerkleTree::create_leaves_from(data, &hasher);

        let tree = FullMerkleTree::create_tree(leaves.clone(), &hasher);
        let root_hash = tree.borrow().value.clone();

        Some(Self {
            tree,
            leaves,
            root_hash,
            hasher,
        })
    }

    fn create_leaves_from<T: HashableData>(data: &[T], hasher: &H) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node::new(hasher.get_hash_from_data(el), None, None, None))
            .collect()
    }

    fn create_tree(mut leaves: Vec<MKNode>, hasher: &H) -> MKNode {
        while leaves.len() > 1 {
            leaves = leaves
                .chunks(2)
                .map(|el| match el {
                    [a, b] => Self::create_node(a, b, hasher),
                    // hash with itself
                    [a] => Self::create_node(a, &Node::<Hash>::clone(a), hasher),
                    _ => panic!("unexpected chunk size"),
                })
                .collect();
        }

        // there has to be a first, otherwise the while would keep running
        return leaves.first().unwrap().to_owned();
    }

    fn create_node(a: &MKNode, b: &MKNode, hasher: &H) -> MKNode {
        let node = Node::new(
            hasher.get_combined_hash(&a.borrow().value, &b.borrow().value),
            Some(vec![Rc::clone(a), Rc::clone(b)]),
            None,
            None,
        );
        Node::set_parent_and_siblings(&node, &[a, b]);

        node
    }

    fn rebuild_tree(&mut self) {
        let tree = FullMerkleTree::create_tree(self.leaves.clone(), &self.hasher);
        let root_hash = tree.borrow().value.clone();
        self.tree = tree;
        self.root_hash = root_hash;
    }
}

impl<H: Hasher> MerkleTree<MKNode> for FullMerkleTree<H> {
    fn get_leaf_by_idx(&self, idx: usize) -> Option<MKNode> {
        self.leaves.get(idx).cloned()
    }

    fn get_leaf_by_hash(&self, hash: &Hash) -> Option<MKNode> {
        self.leaves
            .iter()
            .find(|el| el.borrow().value == *hash)
            .cloned()
    }

    fn add_leaf<T: HashableData>(&mut self, data: T) {
        let hash = self.hasher.get_hash_from_data(data);
        let node = Node::new(hash, None, None, None);
        self.leaves.push(node);
        self.rebuild_tree();
    }

    fn delete_leaf(&mut self, index: usize) {
        if self.leaves.get(index).is_some() {
            self.leaves.remove(index);
            self.rebuild_tree();
        }
    }

    fn update_leaf<T: HashableData>(&mut self, index: usize, data: T) {
        if let Some(node) = self.leaves.get(index) {
            node.borrow_mut().value = self.hasher.get_hash_from_data(data);
            self.rebuild_tree();
        }
    }

    fn gen_proof(&self, leaf_idx: usize) -> Option<Vec<Hash>> {
        let mut proof: Vec<Hash> = Vec::new();
        let mut current_node = match self.leaves.get(leaf_idx) {
            Some(node) => node.clone(),
            None => return None,
        };

        loop {
            let sibling = current_node.borrow().get_sibling(0);
            // this means we've reached the root node
            if sibling.is_none() {
                break;
            }
            proof.push(sibling.unwrap().borrow().value.clone());
            // if it has a sibling, then it must have a parent
            let parent_node = current_node.borrow().get_parent().unwrap();
            current_node = parent_node;
        }

        Some(proof)
    }

    fn verify_proof(&self, leaf_hash: &Hash, mut leaf_idx: usize, proof: Vec<Hash>) -> bool {
        let mut leaf_hash = leaf_hash.clone();
        for hash in proof {
            if num::is_even(leaf_idx) {
                leaf_hash = self.hasher.get_combined_hash(&leaf_hash, &hash);
            } else {
                leaf_hash = self.hasher.get_combined_hash(&hash, &leaf_hash);
            }
            leaf_idx /= 2;
        }
        leaf_hash == self.root_hash
    }

    fn contains_hash(&self, hash: &Hash) -> Option<(usize, Vec<Hash>)> {
        let leaf = self
            .leaves
            .iter()
            .enumerate()
            .find(|(_, el)| el.borrow().value == *hash);

        let leaf_idx = leaf?.0;
        // if the leaf exists then the gen_proof also does
        return Some((leaf_idx, self.gen_proof(leaf_idx).unwrap()));
    }
}
