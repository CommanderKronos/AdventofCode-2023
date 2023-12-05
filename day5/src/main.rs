use std::fs;

// struct Map {
//     name: String,
//     ranges: Vec<Range>
// }

// struct Range {
//     // Calculate the entire ranges and store in Vec on struct initialisation
//     // Sort the ranges in seperate Vecs based on the first element, 
//     // then map both the source and dest range to a HashMap
//     source_ranges: Vec<Vec<u32>>,
//     destination_ranges: Vec<Vec<u32>>
// }

// impl Range {
//     pub fn new() -> Self {
//         // Input source_start, destination_start and range. Return source_range and Destination Range in Range object
//     }
// }

fn main() {
    let file_name = String::from("test.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };
    
    for line in file_content.lines() {
        
    }
    
}
