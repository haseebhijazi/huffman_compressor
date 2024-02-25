use std::fmt::Debug;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct HuffmanNode<T> {
    pub value: T,
    pub left: Option<Box<HuffmanNode<T>>>,
    pub right: Option<Box<HuffmanNode<T>>>,
}


impl<T: Debug> HuffmanNode<T> {
    pub fn new(value: T) -> Self {
        HuffmanNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: HuffmanNode<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: HuffmanNode<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn print_tree(mut self) {
        let mut queue: VecDeque<&mut HuffmanNode<T>> = VecDeque::new();
        queue.push_front(&mut self);
        while !queue.is_empty() {
            let HuffmanNode {
                ref mut value,
                ref mut left,
                ref mut right,
            } = queue.pop_back().unwrap();

            println!("{:?}",value);

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    println!("Leaf");
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    println!("Leaf");
                }
            }
        }
    }
}