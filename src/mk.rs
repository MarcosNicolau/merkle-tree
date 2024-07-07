use std::rc::Rc;

use crate::tree::*;

use blake2::{Blake2b512, Digest};

type Hash = [u8; 64];

type MKNode = TreeNode<Hash>;

pub struct MerkleTree {
    tree: MKNode,
    leaves: Vec<MKNode>,
    root_hash: Hash,
}

// creates an alias for the trait AsRef<[u8]>, so that we don't have to write every time
trait Data: AsRef<[u8]> {}
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

    fn get_leaves(&self) -> Vec<MKNode> {
        todo!()
    }

    fn add_leaf<T: Data>(&mut self, data: T) {
        todo!()
    }

    fn remove_leaf<T: Data>(&mut self, data: T) {
        todo!()
    }

    fn update_left(&mut self) {
        todo!()
    }

    fn delete_leaf(&mut self) {
        todo!()
    }

    fn gen_proof<T: Data>(&self, data: Vec<T>) -> Hash {
        todo!()
    }

    fn verify_proof(&self, proof: Hash) -> bool {
        todo!()
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