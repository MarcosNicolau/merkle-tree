//! # My Crate
//!
//! `merkle_tree` is a Rust library that provides implementations for Merkle trees and hash operations.
//!
//! ## Overview
//!
//! This crate includes:
//!
//! - `MerkleTree`: Provides functionality for creating and manipulating Merkle trees.
//! - `Hasher`: Defines traits for generating hash values and combined hashes.
//! - Various utility functions and types related to cryptographic operations.
//!
//! ## Examples
//!
//! ### Creating a Merkle Tree
//!
//! ```
//! use merkle_tree::mk::MerkleTree;
//! use merkle_tree::utils::crypto::Sha256Hasher;
//! use merkle_tree::utils::crypto::Blake2s256Hasher;
//!
//! let data = vec!["hello", "world", "how", "are", "you"];
//! // you can choose between sha and blake 256
//! let hasher = Sha256Hasher::new();
//! // let hasher = Blake2s256Hasher::new();
//! let tree = MerkleTree::create(&data, hasher).unwrap();
//!
//! assert_eq!(tree.leaves.len(), 5);
//! println!("Root hash: {:?}", tree.root_hash);
//! ```
//!
//! ### Using your own Hasher
//!
//! ```
//! use merkle_tree::utils::crypto::{Hash, HashableData};
//! use merkle_tree::Hasher;
//! use merkle_tree::utils::crypto::Sha256Hasher;
//!
//! struct MyHasher;
//!
//! impl Hasher for MyHasher {
//!     fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash {
//!         // Implementation of combined hash
//!         vec![]
//!     }
//!
//!     fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash {
//!         // Implementation of data hashing
//!         vec![]
//!     }
//! }
//!
//! let hasher = MyHasher {};
//! let data = "example data";
//! let hash = hasher.get_hash_from_data(data);
//! println!("Hash: {:?}", hash);
//! ```
//!
//! ## Modules
//!
//! - `mk`: Contains modules related to Merkle trees.
//! - `utils::crypto`: Utilities for cryptographic operations.
//!
//! ## Traits
//!
//! - `MerkleTree`: Interface for interacting with Merkle trees.
//! - `Hasher`: Interface for generating hash values and combined hashes.
//!
//! For more details, refer to the specific modules and trait definitions.
//!
//! ## License
//!
//! This crate is licensed under the MIT license. See the LICENSE file for more details.
//!
pub mod mk;
pub mod tree;
pub mod utils;
