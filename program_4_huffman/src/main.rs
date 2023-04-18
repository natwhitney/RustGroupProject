use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let input_arg: Vec<String> = env::args().collect();

    let file_path = &input_arg[1];

    let file_contents = fs::read_to_string(file_path)
        .expect("Couldn't Read File");

    let char_contents = count_chars(file_contents);

    println!("{:?}", char_contents)
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


//TODO: Work this out
fn sort_hashmap(input: HashMap<char, i32>) -> HashMap<char, i32>{
    let mut list = HashMap::new();

    for value in input{
        
    }

    list
}