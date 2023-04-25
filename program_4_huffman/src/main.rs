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
    let char_vals = sort_to_vector(char_vals);

    println!("{:?}", char_vals);
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


//Takes the hashmap of characters and apperances, and puts them into a sorted tuple vector
fn sort_to_vector(input: HashMap<char, i32>) -> Vec<CharCounts>{
    let mut temp_vec = Vec::new();

    for value in input{
        let cur_char = CharCounts(value.1, value.0); 
        temp_vec.push(cur_char);
    }

    temp_vec.sort_by(|a, b| a.cmp(b));
    return temp_vec;
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct CharCounts (i32, char);