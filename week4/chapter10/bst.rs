use std::fmt::Display;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                Some(child) => child.insert(value),
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
        } else if value > self.value {
            match &mut self.right {
                Some(child) => child.insert(value),
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn inorder(&self)
    where
        T: Display,
    {
        match &self.left {
            Some(child) => child.inorder(),
            None => {}
        }

        print!("{} ", self.value);

        match &self.right {
            Some(child) => child.inorder(),
            None => {}
        }
    }
}

fn main() {
    let mut root = Node::new(20);

    for value in [10, 30, 5, 15, 25, 35] {
        root.insert(value);
    }

    println!("Sorted values:");
    root.inorder();
    println!();

    root.insert(10);

    println!("After inserting duplicate 10:");
    root.inorder();
    println!();
}