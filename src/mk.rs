use crate::tree::*;
use crate::utils;
use blake2::{Blake2b512, Digest};
use std::rc::Rc;

type Hash = [u8; 64];
type MKNode = TreeNode<Hash>;
pub struct MerkleTree {
    tree: MKNode,
    leaves: Vec<MKNode>,
    root_hash: Hash,
}

// creates an alias for the trait AsRef<[u8]>, so that we don't have to write every time
pub trait Data: AsRef<[u8]> {}
impl<T: AsRef<[u8]>> Data for T {}

impl MerkleTree {
    fn create<T: Data>(data: Vec<T>) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        let leaves = MerkleTree::get_leaves_from(data);

        let tree = MerkleTree::create_tree(leaves.clone());
        let root_hash = tree.borrow().value;

        Some(Self {
            tree,
            root_hash,
            leaves,
        })
    }

    fn get_leaves_from<T: Data>(data: Vec<T>) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node::new(MerkleTree::get_hash_from_data(el), None, None, None))
            .collect()
    }

    fn create_tree(mut leaves: Vec<MKNode>) -> MKNode {
        while leaves.len() != 1 {
            leaves = leaves
                .chunks(2)
                .map(|el| match el {
                    [a, b] => {
                        let node = Node::new(
                            MerkleTree::get_combined_hash(a.borrow().value, b.borrow().value),
                            Some(vec![Rc::clone(a), Rc::clone(b)]),
                            None,
                            None,
                        );
                        Node::set_parent_and_siblings(&node, &[a, b]);
                        return node;
                    }
                    // hash with itself
                    [a] => {
                        let node = Node::new(
                            MerkleTree::get_combined_hash(a.borrow().value, a.borrow().value),
                            Some(vec![Rc::clone(a)]),
                            None,
                            None,
                        );
                        // does not have siblings
                        Node::set_parent(&node, a);
                        return node;
                    }
                    _ => panic!("unexpected chunk size"),
                })
                .collect();
        }

        return leaves.get(0).unwrap().to_owned();
    }

    pub fn get_leaves(&self) -> Vec<MKNode> {
        self.leaves.to_owned()
    }

    pub fn add_leaf<T: Data>(&mut self, data: T) {
        let hash = MerkleTree::get_hash_from_data(data);
        let node = Node::new(hash, None, None, None);
        self.leaves.push(node);
        self.rebuild_tree();
    }

    pub fn delete_leaf<T: Data>(&mut self, index: usize) {
        if let Some(_) = self.leaves.get(index) {
            self.leaves.remove(index);
            self.rebuild_tree();
        }
    }

    pub fn update_left<T: Data>(&mut self, index: usize, data: T) {
        if let Some(node) = self.leaves.get(index) {
            node.borrow_mut().value = MerkleTree::get_hash_from_data(data);
            self.rebuild_tree();
        }
    }

    fn rebuild_tree(&mut self) {
        let tree = MerkleTree::create_tree(self.leaves.clone());
        let root_hash = tree.borrow().value;
        self.tree = tree;
        self.root_hash = root_hash;
    }

    pub fn gen_proof<T: Data>(&self, leaf_idx: usize) -> Result<Vec<Hash>, &str> {
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

    fn get_combined_hash(a: Hash, b: Hash) -> Hash {
        let mut hasher = Blake2b512::new_with_prefix(a);
        hasher.update(b);
        hasher.finalize().into()
    }

    fn get_hash_from_data<T: Data>(el: T) -> Hash {
        Blake2b512::new_with_prefix(el).finalize().into()
    }
}

impl<T: AsRef<[u8]>> From<Vec<T>> for MerkleTree {
    fn from(value: Vec<T>) -> Self {
        MerkleTree::create(value).expect("data can't be empty")
    }
}
