use blake2::{Blake2s256, Digest};
use sha2::Sha256;

pub type Hash = Vec<u8>;

pub trait HashableData: AsRef<[u8]> {}
impl<T: AsRef<[u8]>> HashableData for T {}

/// Trait defining operations for generating hashes and combined hashes.
pub trait Hasher {
    /// Computes the combined hash of two hash values `a` and `b`.
    ///
    /// # Parameters
    ///
    /// - `a`: The first hash value.
    /// - `b`: The second hash value.
    ///
    /// # Returns
    ///
    /// A `Hash` representing the combined hash of `a` and `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Hasher;
    /// use my_crate::utils::crypto::Hash;
    ///
    /// struct MyHasher;
    ///
    /// impl Hasher for MyHasher {
    ///     fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash {
    ///         // Implement your combined hash logic here
    ///         vec![]
    ///     }
    /// }
    ///
    /// let hasher = MyHasher {};
    /// let hash_a = vec![0x01, 0x02, 0x03];
    /// let hash_b = vec![0x04, 0x05, 0x06];
    /// let combined_hash = hasher.get_combined_hash(&hash_a, &hash_b);
    /// println!("Combined Hash: {:?}", combined_hash);
    /// ```
    fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash;

    /// Computes the hash of data `el` that implements `HashableData`.
    ///
    /// # Parameters
    ///
    /// - `el`: The data to hash, implementing `HashableData`.
    ///
    /// # Returns
    ///
    /// A `Hash` representing the hash of the provided data `el`.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Hasher;
    /// use my_crate::utils::crypto::{Hash, HashableData};
    ///
    /// struct MyHasher;
    ///
    /// impl Hasher for MyHasher {
    ///     fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash {
    ///         // Implement your data hashing logic here
    ///         vec![]
    ///     }
    /// }
    ///
    /// let hasher = MyHasher {};
    /// let data_to_hash = "example data";
    /// let hash = hasher.get_hash_from_data(data_to_hash);
    /// println!("Hash: {:?}", hash);
    /// ```
    fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash;
}

pub struct Sha256Hasher {}

impl Sha256Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Hasher for Sha256Hasher {
    fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash {
        let mut hasher = Sha256::new_with_prefix(a);
        hasher.update(b);
        hasher.finalize().to_vec()
    }

    fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash {
        Sha256::new_with_prefix(el).finalize().to_vec()
    }
}

pub struct Blake2s256Hasher {}

impl Hasher for Blake2s256Hasher {
    fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash {
        let mut hasher = Blake2s256::new_with_prefix(a);
        hasher.update(b);
        hasher.finalize().to_vec()
    }

    fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash {
        Blake2s256::new_with_prefix(el).finalize().to_vec()
    }
}

impl Blake2s256Hasher {
    pub fn new() -> Self {
        Self {}
    }
}
