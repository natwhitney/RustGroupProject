use std::vec::Vec;
use std::collections::HashMap;
use std::fs;
use std::env;
use binary_tree::Node;

mod binary_tree;

fn main() {
    let input_arg: Vec<String> = env::args().collect();

    let file_contents = fs::read_to_string(&input_arg[1])
        .expect("Couldn't Read File");

    let char_vals = count_chars(file_contents);
    let mut node_vec = create_nodes(char_vals);

    node_vec.sort();

    println!("{:?}", node_vec);
}

//Builds a huffman binary tree based off of the node list we made
fn build_huffman(node_vec: Vec<Node<CharCounts>>) -> Node<CharCounts> {
    while node_vec.len() > 1 {
        node_vec.sort_by(|a, b| b.cmp(a));
        let right = node_vec.pop();
        let left = node_vec.pop();
    }

    let final_node = node_vec.pop();
}

//Returns a Hashmap of each character being a key, and its value being how many times it appeared
fn count_chars(input: String) -> HashMap<char, i32>{
    let mut map = HashMap::new();

    for character in input.chars(){
        let char_count = map.entry(character)
            .or_insert(0);
        *char_count += 1;
    }

    map
}

//Creates a list of nodes based on the values counted in the HashMap
fn create_nodes(chars_hash: HashMap<char, i32>) -> Vec<Node<CharCounts>> {
    let mut node_vec = Vec::new();

    for value in chars_hash{
        let cur_val = CharCounts(value.1, Some(value.0));
        node_vec.push(binary_tree::Node::new_node(cur_val, None, None))
    }

    node_vec
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct CharCounts (i32, Option<char>);