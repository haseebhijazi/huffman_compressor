use std::io;
use std::collections::HashMap;
use priority_queue::DoublePriorityQueue;

mod huffman_tree;

fn set_codes(root: &huffman_tree::HuffmanNode<(char, i32)>, s: String, map: &mut HashMap<char, String>) {   
    if root.left.is_none() && root.right.is_none() && root.value.0.is_alphanumeric() {
        map.insert(root.value.0, s);
        return;
    }
    if let Some(node) = &root.left {
        let mut new_s = s.clone();
        new_s.push('0');
        set_codes(node, new_s, map);
    }
    if let Some(node) = &root.right {
        let mut new_s = s.clone();
        new_s.push('1');
        set_codes(node, new_s, map);
    } 
}

fn print_codes(codemap: HashMap<char, String>) {
    println!("Hamming Codes: ");
    for (key, value) in &codemap {
        println!("{}: {}", key, value);
    }
}

fn calculate_stats(text: String, codemap: HashMap<char, String>, character_map: HashMap<char, i32>) {
    let length = text.len();
    println!("Bytes used by the text: {:?}", length*8);
    let mut new_length = 0;
    for (key, value) in &character_map {
        let code = codemap.get(key);
        let code_length: i32;
        if let Some(c) = code {
            code_length = c.len() as i32;
        } else {
            code_length = 0;
        }
        new_length += code_length*(*value);
    }  
    println!("Bytes used by after compression: {:?}", new_length);
}

fn main() {
    let mut text = String::new();

    println!("Input text:");    
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read the input!");

    let mut character_map: HashMap<char, i32> = HashMap::new();

    for ch in text.chars() {
        character_map.entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut pq: DoublePriorityQueue<huffman_tree::HuffmanNode<(char, i32)>, i32> = DoublePriorityQueue::new();

    assert!(pq.is_empty());

    for (key, value) in &character_map {
        pq.push(huffman_tree::HuffmanNode::new((*key, *value)), *value);
    } 
    let mut root = huffman_tree::HuffmanNode::new(('-', 0));

    while !pq.is_empty() {
        if let (Some(q1), Some(q2)) = (pq.pop_min(), pq.pop_min()) {
            let (node1, freq1) = q1;
            let (node2, freq2) = q2;
            let fnode = huffman_tree::HuffmanNode::new(('-', freq1 + freq2))
                .left(node1)
                .right(node2);
    
                root = fnode.clone();
                pq.push(fnode, freq1+freq2);
        } else {
            println!();
        }

        
    }
    println!("The Huffman Tree: ");
    root.clone().print_tree();
    println!();

    let mut codemap: HashMap<char, String> = HashMap::new();
    set_codes(&root, "".to_string(), &mut codemap);
    print_codes(codemap.clone());
    
    calculate_stats(text, codemap.clone(), character_map.clone());
}
