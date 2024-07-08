use crate::utils::crypto::*;

struct Node<T> {
    value: T,
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
    leaves: Vec<MKNode>,
    root_hash: Hash,
}

impl CompactMerkleTree {
    fn create<T: DataToHash>(data: &[T]) -> Self {
        let leaves: Vec<Node<[u8; 64]>> = Self::get_leaves_from(data);
        let root_hash = Self::calculate_root(leaves.clone());
        Self { leaves, root_hash }
    }

    fn calculate_root(mut leaves: Vec<MKNode>) -> Hash {
        while leaves.len() > 1 {
            leaves = leaves
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
                .collect();
        }

        return leaves.get(0).unwrap().value;
    }

    fn get_leaves_from<T: DataToHash>(data: &[T]) -> Vec<MKNode> {
        data.iter()
            .map(|el| Node {
                value: get_hash_from_data(el),
            })
            .collect()
    }
}

impl<T: DataToHash> From<&[T]> for CompactMerkleTree {
    fn from(value: &[T]) -> CompactMerkleTree {
        CompactMerkleTree::create(value)
    }
}
