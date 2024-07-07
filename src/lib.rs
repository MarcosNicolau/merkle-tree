use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type NodeParent<T> = Weak<RefCell<Node<T>>>;
type NodeChildren<T> = Vec<Rc<RefCell<Node<T>>>>;
type NodeSiblings<T> = Vec<Weak<RefCell<Node<T>>>>;

pub struct Node<T> {
    value: T,
    parent: Option<NodeParent<T>>,
    children: Option<NodeChildren<T>>,
    siblings: Option<NodeSiblings<T>>,
}
type TreeNode<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    fn new(
        value: T,
        children: Option<NodeChildren<T>>,
        siblings: Option<NodeSiblings<T>>,
        parent: Option<NodeParent<T>>,
    ) -> TreeNode<T> {
        Rc::new(RefCell::new(Node {
            children,
            parent,
            siblings,
            value,
        }))
    }
}

type Hash = [u8; 64];

type HashTree = TreeNode<Hash>;

pub struct MerkleTree {
    tree: HashTree,
    leaves: Vec<TreeNode<Hash>>,
    root_hash: Hash,
}

// creates an alias for the trait AsRef<[u8]>, so that we don't have to write every time
trait Data: AsRef<[u8]> {}
impl<T: AsRef<[u8]>> Data for T {}
type Nodes = Vec<HashTree>;

impl MerkleTree {
    fn create<T: Data>(data: Vec<T>) -> Self {
        todo!()
    }
}

impl<T: Data> From<Vec<T>> for MerkleTree {
    fn from(value: Vec<T>) -> Self {
        MerkleTree::create(value)
    }
}
