use std::fmt::Debug;
use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;
use priority_queue::DoublePriorityQueue;

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

fn main() {
    let mut text = String::new();

    println!("Input text:");    
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read the input!");

    let mut charachter_map: HashMap<char, i32> = HashMap::new();

    for ch in text.chars() {
        charachter_map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut pq: DoublePriorityQueue<HuffmanNode<(char, i32)>, i32> = DoublePriorityQueue::new();

    assert!(pq.is_empty());

    for (key, value) in &charachter_map {
        pq.push(HuffmanNode::new((*key, *value)), *value);
    } 
    let mut root = HuffmanNode::new(('-', 0));

    while !pq.is_empty() {
        if let (Some(q1), Some(q2)) = (pq.pop_min(), pq.pop_min()) {
            let (node1, freq1) = q1;
            let (node2, freq2) = q2;
            let fnode = HuffmanNode::new(('-', freq1 + freq2))
                .left(node1)
                .right(node2);
    
                root = fnode.clone();
                pq.push(fnode, freq1+freq2);
        } else {
            println!("Error while retrieving node from queue");
        }

        
    }
    println!("The Huffman Tree: ");
    root.clone().print_tree();
    println!();
}
