use blake2::{Blake2b512, Digest};

pub type Hash = [u8; 64];

pub trait DataToHash: AsRef<[u8]> {}
impl<T: AsRef<[u8]>> DataToHash for T {}

pub fn get_combined_hash(a: Hash, b: Hash) -> Hash {
    let mut hasher = Blake2b512::new_with_prefix(a);
    hasher.update(b);
    hasher.finalize().into()
}

pub fn get_hash_from_data<T: DataToHash>(el: T) -> Hash {
    Blake2b512::new_with_prefix(el).finalize().into()
}
