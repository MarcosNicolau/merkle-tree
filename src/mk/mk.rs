use crate::utils::crypto::{Hash, HashableData};

pub trait MerkleTree<MKNode> {
    fn get_leaf_by_idx(&self, idx: usize) -> Option<MKNode>;

    fn get_leaf_by_hash(&self, hash: &Hash) -> Option<MKNode>;

    fn add_leaf<T: HashableData>(&mut self, data: T);

    fn delete_leaf(&mut self, index: usize);

    fn update_leaf<T: HashableData>(&mut self, index: usize, data: T);

    fn gen_proof(&self, leaf_idx: usize) -> Option<Vec<Hash>>;

    fn verify_proof(&self, leaf_hash: &Hash, leaf_idx: usize, proof: Vec<Hash>) -> bool;

    fn contains_hash(&self, hash: &Hash) -> Option<(usize, Vec<Hash>)>;
}
