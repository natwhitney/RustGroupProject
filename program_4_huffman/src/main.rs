use std::vec::Vec;
use std::collections::HashMap;
use std::fs;
use std::env;
use binary_tree::Node;
use bitvec::prelude::*;
use std::io::Write;

mod binary_tree;

fn main() {
    let input_arg: Vec<String> = env::args().collect();

    let file_contents = fs::read_to_string(&input_arg[1])
        .expect("Couldn't Read File");

    build_and_write(&file_contents, &input_arg[2]);
}

fn build_and_write(input_string: &String, new_file_name: &String) {
    let char_vals = count_chars(&input_string);
    let mut node_vec = create_nodes(&char_vals);

    let final_node = build_huffman_tree(&mut node_vec);

    let mut char_hash = HashMap::new();
    gen_binary_reps(&final_node, &mut char_hash, "");

    let binary_string = replace_chars(&input_string, &char_hash);

    let mut tree_string = String::new();
    store_tree(&final_node, &mut tree_string);

    let full_tree_string = convert_tree_to_full_binary(&tree_string);

    println!("{}", input_string);
    println!("{:?}", char_vals);
    println!("{:?}", char_hash);
    println!("{}\n", binary_string);
    println!("{}\n", tree_string);
    println!("{}\n", full_tree_string);

    let full_string = binary_string + "0000000000000000" + &full_tree_string;

    println!("{}", full_string);

    let mut new_file = fs::File::create(new_file_name)
        .expect("Couldn't Create File");

    write!(new_file, "{}", full_string)
        .expect("Couldn't Write to File");

    /* 
    let mut full_bit_vec = bitvec!();
    for char in full_string.chars() {
        match char {
            '0' => full_bit_vec.push(false),
            '1' => full_bit_vec.push(true),
            _ => panic!("Unexpected value in binary string!"),
        }
    }

    println!("{:?}", full_bit_vec);
    
    let mut new_file = fs::File::create(new_file_name)
        .expect("Couldn't Create File");

    let mut pos = 0;
    
    while pos < full_bit_vec.len() {
        let bytes_written = new_file.write(&full_bit_vec[pos]);
    }
    */


}

fn convert_tree_to_full_binary(binary_string: &String) -> String {
    let mut bit_string = String::new();
    let mut leaf_flag = false;
    for char in binary_string.clone().into_bytes() {
        if leaf_flag == true {
            let cur_char = &format!("{:0>8b}", char);
            bit_string += cur_char;
            leaf_flag = false;
        }
        //48 is the ascii for 0 
        else if char == 48 {
            bit_string.push('0');
        }
        //49 is the ascii for 1 
        else if char == 49 {
            bit_string.push('1');
            leaf_flag = true;
        }
    }

    bit_string
}


//generates a new binary representation for each character, using the huffman tree
fn gen_binary_reps(node: &Node<CharCounts>, char_hash: &mut HashMap<char, String>, binary_string: &str) {
    let cur_char = node.get_info().0.1;
    let new_nodes = node.get_children();

    if cur_char != None {
        char_hash.insert(cur_char.unwrap(), binary_string.to_string());
    } else {
        if new_nodes.0 != &None {
            gen_binary_reps(&new_nodes.0.as_ref().unwrap(), char_hash, (binary_string.to_owned() + "0").as_str());
        }   
        if new_nodes.1 != &None {
            gen_binary_reps(&new_nodes.1.as_ref().unwrap(), char_hash, (binary_string.to_owned() + "1").as_str());
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

//stores the tree in a string, followed by 16 bits of 0,
//to make sure we know where the tree ends, and the actual data starts.

//If the node is a leaf, store a 1 and then its character represented by 8 bits
//Else put a 0 down and then recursively call for the left and right children.
fn store_tree(node: &Node<CharCounts>, output_string: &mut String) {
    if node.is_leaf() {
        output_string.push('1');
        output_string.push(node.get_info().0.1.unwrap());
    } else {
        let left_child = node.get_children().0.as_ref().unwrap();
        let right_child = node.get_children().1.as_ref().unwrap();
        output_string.push('0');
        store_tree(&left_child, output_string);
        store_tree(right_child, output_string);
    }
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