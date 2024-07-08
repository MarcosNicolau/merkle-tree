use crate::tree::*;
use crate::utils::crypto::*;
use crate::utils::num;
use std::rc::Rc;

type Hash = [u8; 64];
type MKNode = TreeNode<Hash>;
pub struct FullMerkleTree {
    tree: MKNode,
    leaves: Vec<MKNode>,
    root_hash: Hash,
}

impl FullMerkleTree {
    fn create<T: DataToHash>(data: Vec<T>) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        let leaves = FullMerkleTree::get_leaves_from(data);

        let tree = FullMerkleTree::create_tree(leaves.clone());
        let root_hash = tree.borrow().value;

        Some(Self {
            tree,
            root_hash,
            leaves,
        })
    }

    fn get_leaves_from<T: DataToHash>(data: Vec<T>) -> Vec<MKNode> {
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
                    [a] => Self::create_node(a, a),
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

    pub fn get_leaves(&self) -> Vec<MKNode> {
        self.leaves.to_owned()
    }

    pub fn add_leaf<T: DataToHash>(&mut self, data: T) {
        let hash = get_hash_from_data(data);
        let node = Node::new(hash, None, None, None);
        self.leaves.push(node);
        self.rebuild_tree();
    }

    pub fn delete_leaf<T: DataToHash>(&mut self, index: usize) {
        if let Some(_) = self.leaves.get(index) {
            self.leaves.remove(index);
            self.rebuild_tree();
        }
    }

    pub fn update_left<T: DataToHash>(&mut self, index: usize, data: T) {
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

    pub fn gen_proof<T: DataToHash>(&self, leaf_idx: usize) -> Result<Vec<Hash>, &str> {
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
}

impl<T: AsRef<[u8]>> From<Vec<T>> for FullMerkleTree {
    fn from(value: Vec<T>) -> Self {
        FullMerkleTree::create(value).expect("data can't be empty")
    }
}
