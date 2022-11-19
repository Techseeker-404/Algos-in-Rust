#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: i32,
    right: NodeType,
    left: NodeType,
}

#[derive(Debug)]
struct Tree {
    root: NodeType,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            right: None,
            left: None,
        }
    }
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: i32) {
        match &self.root {
            None => {
                //let _ = std::mem::replace(&mut self.root, Node::new(value).into());
                self.root = Node::new(value).into()
            }

            Some(node) => {
                self._insert_recursive(node.clone(), value);
            }
        }
    }

    fn _insert_recursive(&self, node: Rc<RefCell<Node>>, value: i32) {
        let mut node = node.borrow_mut();
        if value > node.value {
            if let Some(node_type) = &node.right {
                self._insert_recursive(Rc::clone(&node_type), value);
            } else {
                node.right = Node::new(value).into();
            }
        } else if value < node.value {
            match &node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(node_type) => {
                    self._insert_recursive(Rc::clone(&node_type), value);
                }
            }
        }
    }
}

impl From<Node> for Option<Rc<RefCell<Node>>> {
    fn from(node: Node) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {
    let mut tree = Tree::new();
    //let node = Node::new(2);
    println!("{tree:?}");
    tree.insert(12);
    tree.insert(13);
    tree.insert(23);
    tree.insert(13);
    tree.insert(5);
    tree.insert(3);
    tree.insert(17);
    tree.insert(15);
    tree.insert(1);
    tree.insert(7);
    println!("{tree:#?}");
}
