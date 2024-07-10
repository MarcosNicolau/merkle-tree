use blake2::{Blake2s256, Digest};
use sha2::Sha256;

pub type Hash = Vec<u8>;

pub trait HashableData: AsRef<[u8]> {}
impl<T: AsRef<[u8]>> HashableData for T {}

pub trait Hasher {
    fn get_combined_hash(&self, a: &Hash, b: &Hash) -> Hash;
    fn get_hash_from_data<T: HashableData>(&self, el: T) -> Hash;
}

pub struct Sha256Hasher {}

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
