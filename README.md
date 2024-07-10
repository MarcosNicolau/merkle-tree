# Merkle ree Library

This Rust library provides functionality for working with Merkle trees in memory. Merkle trees are cryptographic data structures that enable efficient verification of data integrity and consistency.

## Options

### Full Tree

Store the entire Merkle tree structure in memory.

-   Pros: Efficient traversal and verification of any part of the tree.
-   Cons: Higher memory usage and complexity in storage management.

### Compact tree

Calculate and store only the root hash of the Merkle tree.

-   Pros: Lower memory footprint, suitable for space-constrained environments and constant nodes.
-   Cons: Requires more CPU resources for root hash calculation, limited to verifying integrity of the entire dataset.

## Usage

### Installation

Add the following line to your cargo.toml under dependencies:

```toml
[dependencies]
merkle_tree = { git = "https://github.com/MarcosNicolau/merkle-tree.git", branch = "main" }
```

or you can also do:

`cargo add --git https://github.com/MarcosNicolau/merkle-tree.git --branch main`

### Example

```rust
use merkle_tree::mk::compact::CompactMerkleTree;
use merkle_tree::mk::full::FullMerkleTree;
use merkle_tree::mk::mk::MerkleTree;
use merkle_tree::utils::crypto::{Hash, Hasher, Sha256Hasher};

fn verify_block_by_hash(hash: Hash, mk: &FullMerkleTree<Sha256Hasher>) -> bool {
    if let Some((idx, proof)) = mk.contains_hash(&hash) {
        return mk.verify_proof(&hash, idx, proof);
    }

    return false;
}

fn main() {
    let data = vec!["hello", "how", "are", "you"];

    // here you can choose between the compact or the full version (their api is the same)
    let mk = FullMerkleTree::create(data.as_slice(), Sha256Hasher::new()).unwrap();
    // let mk = CompactMerkleTree::create(data.as_slice(), Sha256Hasher::new()).unwrap();

    if verify_block_by_hash(mk.hasher.get_hash_from_data("hello"), &mk) {
        print!("Hash is contained!")
    } else {
        panic!("Hash is not contained!")
    }

    // you can also get leafs by their id
    let leaf = mk.get_leaf_by_idx(1).unwrap();
    // note that if you are using the CompactMerkleTree you don't need the borrow
    let hash = &leaf.borrow().value;
}
```

### Using you own hasher

This library provides you with Blake256 and Sha256, but you can also implement your own hashes, even one that implements 512 bits.

```rust
use blake2::{Blake2s256, Digest};
use merkle_tree::mk::compact::CompactMerkleTree;
use merkle_tree::mk::full::FullMerkleTree;
use merkle_tree::mk::mk::MerkleTree;
use merkle_tree::utils::crypto::{Hash, HashableData, Hasher};

struct MyHasher {}
// you have to implement the Hasher trait
impl Hasher for MyHasher {
    fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash {
        let mut hasher = Blake2s256::new_with_prefix(a);
        hasher.update(b);
        hasher.finalize().to_vec()
    }

    fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash {
        Blake2s256::new_with_prefix(el).finalize().to_vec()
    }
}
type MyFullMerkleTree = FullMerkleTree<MyHasher>;

fn main() {
    let data = vec!["hello", "how", "are", "you"];
    let mk = FullMerkleTree::create(data.as_slice(), MyHasher {}).unwrap();
    ...
}
```
