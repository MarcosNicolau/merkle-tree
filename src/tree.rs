use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type NodeParent<T> = Weak<RefCell<Node<T>>>;
pub type NodeChildren<T> = Vec<Rc<RefCell<Node<T>>>>;
pub type NodeSiblings<T> = Vec<Weak<RefCell<Node<T>>>>;

pub struct Node<T> {
    pub value: T,
    pub parent: Option<NodeParent<T>>,
    pub children: Option<NodeChildren<T>>,
    pub siblings: Option<NodeSiblings<T>>,
}

pub type TreeNode<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    pub fn new(
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

    pub fn set_parent(parent: &TreeNode<T>, node: &TreeNode<T>) {
        node.borrow_mut().parent = Some(Rc::downgrade(parent));
    }

    pub fn set_parent_and_siblings(parent: &TreeNode<T>, siblings: &[&TreeNode<T>]) {
        for sibling in siblings {
            let mut sibling_mut = sibling.borrow_mut();
            sibling_mut.parent = Some(Rc::downgrade(parent));

            let mut new_siblings = Vec::new();
            for other_sibling in siblings {
                if Rc::ptr_eq(sibling, other_sibling) {
                    continue;
                }
                new_siblings.push(Rc::downgrade(other_sibling));
            }
            sibling_mut.siblings = Some(new_siblings);
        }
    }
}
