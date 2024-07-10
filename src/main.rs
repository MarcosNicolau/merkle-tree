use merkle_tree::mk::compact::CompactMerkleTree;
use merkle_tree::mk::full::FullMerkleTree;
use merkle_tree::utils::crypto::{get_hash_from_data, Hash};

fn verify_block_by_hash(hash: Hash, mk: &FullMerkleTree) -> bool {
    if let Some((idx, proof)) = mk.contains_hash(hash) {
        return mk.verify_proof(hash, idx, proof);
    }

    false
}

fn main() {
    let data = vec!["hello", "how", "are", "you"];
    // here you can choose between the compact or the full version (their api is the same)
    let mk = FullMerkleTree::try_from(data.as_slice()).unwrap();
    // let mk = CompactMerkleTree::try_from(data.as_slice()).unwrap();
    if verify_block_by_hash(get_hash_from_data("hello"), &mk) {
        print!("Hash is contained!")
    } else {
        panic!("Hash is not contained!")
    }

    let leaf = mk.get_leaf_by_idx(1).unwrap();
    let _hash = leaf.borrow().value;
}
