use crate::tree::*;

type Hash = [u8; 64];
type HashTree = TreeNode<Hash>;

pub struct MerkleTree {
    tree: HashTree,
    leaves: Vec<TreeNode<Hash>>,
    root_hash: Hash,
}

// creates an alias for the trait AsRef<[u8]>, so that we don't have to write every time
trait Data: AsRef<[u8]> {}
impl<T: AsRef<[u8]>> Data for T {}
type Nodes = Vec<HashTree>;

impl MerkleTree {
    fn create<T: Data>(data: Vec<T>) -> Self {
        todo!()
    }
}

impl<T: Data> From<Vec<T>> for MerkleTree {
    fn from(value: Vec<T>) -> Self {
        MerkleTree::create(value)
    }
}
