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
        let leaves: Vec<Node<[u8; 64]>> = Self::get_leaves_from(data);
        let root_hash = Self::calculate_root(leaves.clone());
        Some(Self { leaves, root_hash })
    }

    fn calculate_root(mut leaves: Vec<MKNode>) -> Hash {
        while leaves.len() > 1 {
            leaves = Self::get_parent_nodes(&leaves);
        }

        return leaves.get(0).unwrap().value;
    }

    fn get_parent_nodes(nodes: &Vec<MKNode>) -> Vec<MKNode> {
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

    fn get_leaves_from<T: DataToHash>(data: &[T]) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node {
                value: get_hash_from_data(el),
            })
            .collect()
    }

    pub fn get_leaves(&self) -> Vec<MKNode> {
        self.leaves.to_owned()
    }

    pub fn add_leaf<T: DataToHash>(&mut self, data: T) {
        let hash = get_hash_from_data(data);
        self.leaves.push(Node { value: hash });
        self.rebuild_root();
    }

    pub fn delete_leaf(&mut self, index: usize) {
        if let Some(_) = self.leaves.get(index) {
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

        return Ok(proof);
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
}

impl<T: DataToHash> From<&[T]> for CompactMerkleTree {
    fn from(value: &[T]) -> CompactMerkleTree {
        CompactMerkleTree::create(value).expect("Data can't be empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_even_merkle_tree() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        assert_eq!(tree.leaves.len(), 4);
        assert_eq!(tree.root_hash.len(), 64);

        let expected_root_hash = get_combined_hash(
            get_combined_hash(get_hash_from_data(&data[0]), get_hash_from_data(&data[1])),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[3])),
        );
        assert_eq!(tree.root_hash, expected_root_hash);
    }

    #[test]
    fn test_create_odd_merkle_tree() {
        let data = vec!["how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        assert_eq!(tree.leaves.len(), 3);
        assert_eq!(tree.root_hash.len(), 64);

        let expected_root_hash = get_combined_hash(
            get_combined_hash(get_hash_from_data(&data[0]), get_hash_from_data(&data[1])),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[2])),
        );
        assert_eq!(tree.root_hash, expected_root_hash);
    }

    #[test]
    fn test_gen_proof_even() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        // test proof for fist leaf
        let proof = vec![
            get_hash_from_data(&data[1]),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[3])),
        ];
        assert_eq!(proof, tree.gen_proof(0).unwrap());

        // test proof for second leaf
        let proof = vec![
            get_hash_from_data(&data[0]),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[3])),
        ];
        assert_eq!(proof, tree.gen_proof(1).unwrap());
    }

    #[test]
    fn test_gen_proof_odd() {
        let data = vec!["how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        // test proof for fist leaf
        let proof = vec![
            get_hash_from_data(&data[1]),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[2])),
        ];
        assert_eq!(proof, tree.gen_proof(0).unwrap());

        // test proof for second leaf
        let proof = vec![
            get_hash_from_data(&data[0]),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[2])),
        ];
        assert_eq!(proof, tree.gen_proof(1).unwrap());
    }

    #[test]
    fn test_verify_proof_odd() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        // test for first
        let leaf_hash = get_hash_from_data(&data[0]);
        assert!(tree.verify_proof(leaf_hash, 0, tree.gen_proof(0).unwrap()));

        let leaf_hash = get_hash_from_data(&data[2]);
        assert!(tree.verify_proof(leaf_hash, 2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_verify_proof_even() {
        let data = vec!["how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        // test for first
        let leaf_hash = get_hash_from_data(&data[0]);
        assert!(tree.verify_proof(leaf_hash, 0, tree.gen_proof(0).unwrap()));

        let leaf_hash = get_hash_from_data(&data[2]);
        assert!(tree.verify_proof(leaf_hash, 2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_verify_proof_odd_fails() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        let leaf_hash = get_hash_from_data("not right");
        assert!(!tree.verify_proof(leaf_hash, 0, tree.gen_proof(0).unwrap()));
    }

    #[test]
    fn test_verify_proof_even_fails() {
        let data = vec!["how", "are", "you"];
        let tree = CompactMerkleTree::create(data.as_slice()).unwrap();

        let leaf_hash = get_hash_from_data("not right");
        assert!(!tree.verify_proof(leaf_hash, 2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_leaf_gets_added() {
        let data = vec!["how", "are", "you"];
        let mut tree = CompactMerkleTree::create(data.as_slice()).unwrap();
        assert_eq!(tree.leaves.len(), 3);

        tree.add_leaf("hello");
        assert_eq!(tree.leaves.len(), 4)
    }
    #[test]
    fn test_leaf_gets_deleted() {
        let data = vec!["hello", "how", "are", "you"];
        let mut tree = CompactMerkleTree::create(data.as_slice()).unwrap();
        assert_eq!(tree.leaves.len(), 4);

        tree.delete_leaf(0);
        assert_eq!(tree.leaves.len(), 3)
    }
    #[test]
    fn test_leaf_gets_updated() {
        let data = vec!["hello", "how", "are", "you"];
        let mut tree = CompactMerkleTree::create(data.as_slice()).unwrap();
        assert_eq!(tree.leaves.len(), 4);
        tree.update_leaf(0, "hi");

        let val = tree.leaves.get(0).unwrap().value;
        assert_eq!(val, get_hash_from_data("hi"))
    }
}
