use std::vec::Vec;
use std::collections::HashMap;
use std::fs;
use std::env;

mod binary_tree;

fn main() {
    let input_arg: Vec<String> = env::args().collect();

    let file_contents = fs::read_to_string(&input_arg[1])
        .expect("Couldn't Read File");

    let char_vals = count_chars(file_contents);
    let char_vals = sort_to_vector(&char_vals);

    println!("{:?}", char_vals);
}

//Returns a Hashmap of each character being a key, and its value being how many times it appeared
fn count_chars(input: String) -> HashMap<char, i32>{
    let mut list = HashMap::new();

    for character in input.chars(){
        let char_count = list.entry(character)
            .or_insert(0);
        *char_count += 1;
    }

    list
}


//Takes the hashmap of characters and apperances, and puts them into a sorted tuple vector
fn sort_to_vector(input: &HashMap<char, i32>) -> Vec<(&i32, &char)>{
    let mut temp_vec = Vec::new();

    for value in input{
        temp_vec.push((value.1,value.0));
    }

    temp_vec.sort_by(|a, b| b.0.cmp(a.0));

    return temp_vec;
}