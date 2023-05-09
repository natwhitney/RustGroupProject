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
    let new_string = read_byte_to_string(&"new_file.bin".to_string());

    println!("{}", new_string);
}

/*
fn read_message(input_file: &String, output_file: &String) {

}

fn read_tree(input_string: &mut String) -> Node<Option<char>> {
    let mut node: Node<Option<char>> = binary_tree::Node::new_node(None, None, None);

    for bit in input_string.chars(){
        if bit == '0' {
            node.set_left(binary_tree::Node::new_node(None, None, None));
            node.set_right(binary_tree::Node::new_node(None, None, None));
        } else if bit == '1' {

        }
    }
    node
}
*/


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

    let mut full_string = full_tree_string + &binary_string;

    let padding = full_string.len() % 8;

    //To generate padding so it writes in 8 byte chunks nicely
    for _ in 0..padding {
        full_string += "0";
    }

    println!("{}", full_string);

    write_string_as_bytes(&full_string, new_file_name)
}

fn read_byte_to_string(file_name: &String) -> String {
    let byte_vec = &fs::read(file_name).expect("Couldn't Read File");

    let mut final_string = String::new();

    println!();
    for byte in byte_vec {
        println!("{}", byte);
        let temp_string = &format!("{:0>8b}", byte);
        let reversed_byte = temp_string.chars().rev().collect::<String>();
        final_string += &reversed_byte;
    }

    final_string
}

/*
fn read_byte_to_string(file_name: &String) -> String {
    let byte_vec = &fs::read(file_name).expect("Couldn't Read File");

    let mut final_string = String::new();

    for byte in byte_vec {
        let temp_string = &format!("{:0>8b}", byte);
        final_string += &reverse(temp_string);
    }

    final_string
}*/

fn reverse(string: &str) -> String{
    string.chars().rev().collect()
}

fn write_string_as_bytes(binary_string: &String, file_name: &String) {
    if binary_string.len() % 8 != 0 {
        panic!("Binary String not divisible by bytes");
    }

    let mut byte_vec: Vec<u8> = vec![];

    let mut bucket = [0; 8];
    let mut counter = 0;

    for char in binary_string.chars() {
        match char {
            '0' => bucket[counter] = 0,
            '1' => bucket[counter] = 1,
            _ => panic!("Binary string somehow has something other than 1 or 0")
        };

        if counter == 7 {
            let mut num: u8 = 0;
            for i in 0..bucket.len() {
                if bucket[i] == 1 {
                    num += 2_u8.pow(7 - i as u32)
                }
            }
            
            println!("{}", num);
            byte_vec.push(num);
            counter = 0;
            bucket = [0; 8];
        } else {
            counter += 1;  
        }
    }

    std::fs::write(file_name, byte_vec)
        .expect("File Couldn't be created");
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