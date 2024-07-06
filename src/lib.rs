pub struct Node<T> {
    value: T,
    parent: Option<Box<Node<T>>>,
    children: Option<Box<Vec<Node<T>>>>,
}

pub struct MerkleTree<T> {
    tree: Node<T>,
}

impl<T> MerkleTree<T> {
    //todo
    fn create_tree(data: Vec<T>) -> Node<T> {}
}

impl<T> From<Vec<T>> for MerkleTree<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            tree: MerkleTree::create_tree(value),
        }
    }
}
