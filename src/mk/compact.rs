use crate::utils::crypto::*;

pub struct Node<T> {
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

    pub fn get_leaves(&self) -> Vec<MKNode> {
        self.leaves.to_owned()
    }

    pub fn add_leaf<T: DataToHash>(&mut self, data: T) {
        let hash = get_hash_from_data(data);
        self.leaves.push(Node { value: hash });
        self.rebuild_root();
    }

    pub fn delete_leaf<T: DataToHash>(&mut self, index: usize) {
        if let Some(_) = self.leaves.get(index) {
            self.leaves.remove(index);
            self.rebuild_root();
        }
    }

    pub fn update_left<T: DataToHash>(&mut self, index: usize, data: T) {
        if let Some(node) = self.leaves.get_mut(index) {
            node.value = get_hash_from_data(data);
            self.rebuild_root();
        }
    }

    fn rebuild_root(&mut self) {
        let root_hash = Self::calculate_root(self.leaves.clone());
        self.root_hash = root_hash;
    }
}

impl<T: DataToHash> From<&[T]> for CompactMerkleTree {
    fn from(value: &[T]) -> CompactMerkleTree {
        CompactMerkleTree::create(value)
    }
}
