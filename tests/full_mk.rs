use merkle_tree::mk::full::FullMerkleTree;
use merkle_tree::utils::crypto::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_even_merkle_tree() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());

        assert_eq!(tree.leaves.len(), 4);
        assert_eq!(tree.root_hash.len(), 64);

        let expected_root_hash = get_combined_hash(
            get_combined_hash(get_hash_from_data(&data[0]), get_hash_from_data(&data[1])),
            get_combined_hash(get_hash_from_data(&data[2]), get_hash_from_data(&data[3])),
        );
        assert_eq!(tree.root_hash, expected_root_hash);
    }

    #[test]
    fn test_from_odd_merkle_tree() {
        let data = vec!["how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());

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
        let tree = FullMerkleTree::from(data.as_slice());

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
        let tree = FullMerkleTree::from(data.as_slice());

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
        let tree = FullMerkleTree::from(data.as_slice());

        // test for first
        let leaf_hash = get_hash_from_data(&data[0]);
        assert!(tree.verify_proof(leaf_hash, 0, tree.gen_proof(0).unwrap()));

        let leaf_hash = get_hash_from_data(&data[2]);
        assert!(tree.verify_proof(leaf_hash, 2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_verify_proof_even() {
        let data = vec!["how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());

        // test for first
        let leaf_hash = get_hash_from_data(&data[0]);
        assert!(tree.verify_proof(leaf_hash, 0, tree.gen_proof(0).unwrap()));

        let leaf_hash = get_hash_from_data(&data[2]);
        assert!(tree.verify_proof(leaf_hash, 2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_verify_proof_odd_fails() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());

        let leaf_hash = get_hash_from_data("not right");
        assert!(!tree.verify_proof(leaf_hash, 0, tree.gen_proof(0).unwrap()));
    }

    #[test]
    fn test_verify_proof_even_fails() {
        let data = vec!["how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());

        let leaf_hash = get_hash_from_data("not right");
        assert!(!tree.verify_proof(leaf_hash, 2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_leaf_gets_added() {
        let data = vec!["how", "are", "you"];
        let mut tree = FullMerkleTree::from(data.as_slice());
        assert_eq!(tree.leaves.len(), 3);

        tree.add_leaf("hello");
        assert_eq!(tree.leaves.len(), 4)
    }
    #[test]
    fn test_leaf_gets_deleted() {
        let data = vec!["hello", "how", "are", "you"];
        let mut tree = FullMerkleTree::from(data.as_slice());
        assert_eq!(tree.leaves.len(), 4);

        tree.delete_leaf(0);
        assert_eq!(tree.leaves.len(), 3)
    }
    #[test]
    fn test_leaf_gets_updated() {
        let data = vec!["hello", "how", "are", "you"];
        let mut tree = FullMerkleTree::from(data.as_slice());
        assert_eq!(tree.leaves.len(), 4);
        tree.update_leaf(0, "hi");

        let val = tree.leaves.get(0).unwrap().borrow().value;
        assert_eq!(val, get_hash_from_data("hi"))
    }

    #[test]
    fn test_contains_hash() {
        let data = vec!["hello", "how", "are", "you"];
        let mut tree = FullMerkleTree::from(data.as_slice());
        assert_eq!(tree.leaves.len(), 4);
        tree.update_leaf(0, "hi");

        let res = tree.contains_hash(get_hash_from_data("are"));
        assert_eq!(res.unwrap(), (2, tree.gen_proof(2).unwrap()));
    }

    #[test]
    fn test_get_leaf_by_idx() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());

        let res = tree.get_leaf_by_idx(2);
        assert_eq!(res.unwrap().borrow().value, get_hash_from_data("are"));
    }

    #[test]
    fn test_get_leaf_by_hash() {
        let data = vec!["hello", "how", "are", "you"];
        let tree = FullMerkleTree::from(data.as_slice());
        let hash = get_hash_from_data("are");
        let res = tree.get_leaf_by_hash(hash);
        assert_eq!(res.unwrap().borrow().value, hash);
    }
}
