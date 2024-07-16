use crate::utils::crypto::{Hash, HashableData};

/// A trait defining operations for a Merkle tree.
pub trait MerkleTree<MKNode> {
    /// Retrieves a leaf node by its index in the Merkle tree.
    ///
    /// Returns `Some(node)` if a leaf exists at the specified `idx`, otherwise `None`.
    fn get_leaf_by_idx(&self, idx: usize) -> Option<MKNode>;

    /// Retrieves a leaf node by its hash value in the Merkle tree.
    ///
    /// Returns `Some(node)` if a leaf exists with the specified `hash`, otherwise `None`.
    fn get_leaf_by_hash(&self, hash: &Hash) -> Option<MKNode>;

    /// Adds a new leaf node to the Merkle tree. This will rebuild the tree
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::MerkleTree;
    ///
    /// let mut tree = create_merkle_tree(); // Assuming a function to create a Merkle tree
    /// tree.add_leaf("new_data");
    /// ```
    fn add_leaf<T: HashableData>(&mut self, data: T);

    /// Deletes a leaf node from the Merkle tree by its index. This will rebuild the tree
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::MerkleTree;
    ///
    /// let mut tree = create_merkle_tree(); // Assuming a function to create a Merkle tree
    /// tree.delete_leaf(0);
    /// ```
    fn delete_leaf(&mut self, index: usize);

    /// Updates a leaf node in the Merkle tree by its index. This will rebuild the tree
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::MerkleTree;
    ///
    /// let mut tree = create_merkle_tree(); // Assuming a function to create a Merkle tree
    /// tree.update_leaf(0, "updated_data");
    /// ```
    fn update_leaf<T: HashableData>(&mut self, index: usize, data: T);

    /// Generates a Merkle proof for a leaf node at the specified index.
    ///
    /// Returns `Some(proof)` if a leaf exists at the specified `leaf_idx`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::MerkleTree;
    ///
    /// let tree = create_merkle_tree(); // Assuming a function to create a Merkle tree
    /// if let Some(proof) = tree.gen_proof(0) {
    ///     println!("Merkle Proof: {:?}", proof);
    /// }
    /// ```
    fn gen_proof(&self, leaf_idx: usize) -> Option<Vec<Hash>>;

    /// Verifies a Merkle proof for a leaf node.
    ///
    /// Returns `true` if the proof is valid for the leaf node with `leaf_hash` and `leaf_idx`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::MerkleTree;
    ///
    /// let tree = create_merkle_tree(); // Assuming a function to create a Merkle tree
    /// let leaf_hash = get_hash("data_to_verify");
    /// let proof = vec![get_hash("sibling_hash"), get_hash("parent_hash")];
    /// assert!(tree.verify_proof(&leaf_hash, 0, proof));
    /// ```
    fn verify_proof(&self, leaf_hash: &Hash, leaf_idx: usize, proof: Vec<Hash>) -> bool;

    /// Checks if a hash exists as a leaf node in the Merkle tree.
    ///
    /// Returns `Some((idx, proof))` if the `hash` exists as a leaf node, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::MerkleTree;
    ///
    /// let tree = create_merkle_tree(); // Assuming a function to create a Merkle tree
    /// let hash_to_find = get_hash("data_to_find");
    /// if let Some((idx, proof)) = tree.contains_hash(&hash_to_find) {
    ///     println!("Found at index {}: {:?}", idx, proof);
    /// }
    /// ```
    fn contains_hash(&self, hash: &Hash) -> Option<(usize, Vec<Hash>)>;
}
