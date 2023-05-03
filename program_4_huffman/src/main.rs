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


    let char_vals = count_chars(&file_contents);
    let mut node_vec = create_nodes(&char_vals);

    let final_node = build_huffman_tree(&mut node_vec);

    let mut char_hash = HashMap::new();
    gen_binary_reps(final_node, &mut char_hash, "");

    let binary_string = replace_chars(&file_contents, &char_hash);

    println!("{}", file_contents);
    println!("{:?}", char_vals);
    println!("{:?}", char_hash);
    println!("{}", binary_string);
}


//generates a new binary representation for each character, using the huffman tree
fn gen_binary_reps(node: Node<CharCounts>, char_hash: &mut HashMap<char, String>, binary_string: &str) {
    let cur_char = node.get_info().0.1;
    let new_nodes = node.get_children();

    if cur_char != None {
        char_hash.insert(cur_char.unwrap(), binary_string.to_string());
    } else {
        if new_nodes.0 != None {
            gen_binary_reps(*new_nodes.0.unwrap(), char_hash, (binary_string.to_owned() + "0").as_str());
        }   
        if new_nodes.1 != None {
            gen_binary_reps(*new_nodes.1.unwrap(), char_hash, (binary_string.to_owned() + "1").as_str());
        }
    }
}

//replaces characters with a binary string
fn replace_chars(input_string: &String, char_hash: &HashMap<char, String>) -> String {
    let mut final_string = String::new();

    for char in input_string.chars() {
        final_string += char_hash.get(&char).unwrap();
    }
    
    final_string
}

//Builds a huffman binary tree based off of the node list we made
fn build_huffman_tree(node_vec: &mut Vec<Node<CharCounts>>) -> Node<CharCounts> {
    while node_vec.len() > 1 {
        node_vec.sort_by(|a, b| b.cmp(a));

        let right = node_vec.pop().unwrap();
        let left = node_vec.pop().unwrap();
        let sum = right.get_info().0.0 + left.get_info().0.0;

        let new_node = binary_tree::Node::new_node(CharCounts(sum, None), Some(Box::new(left)), Some(Box::new(right)));

        node_vec.push(new_node);
    }

    node_vec.pop().unwrap()
}

//Returns a Hashmap of each character being a key, and its value being how many times it appeared
fn count_chars(input: &String) -> HashMap<char, i32>{
    let mut char_hash = HashMap::new();

    for character in input.chars(){
        let char_count = char_hash.entry(character)
            .or_insert(0);
        *char_count += 1;
    }

    char_hash
}

//Creates a list of nodes based on the values counted in the HashMap
fn create_nodes(chars_hash: &HashMap<char, i32>) -> Vec<Node<CharCounts>> {
    let mut node_vec = Vec::new();

    for value in chars_hash{
        let cur_val = CharCounts(*value.1, Some(*value.0));
        node_vec.push(binary_tree::Node::new_node(cur_val, None, None))
    }

    node_vec
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct CharCounts (i32, Option<char>);