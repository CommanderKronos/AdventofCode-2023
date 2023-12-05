use std::{fs, collections::HashMap};
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>
}

impl Map {
    pub fn new(name: String, ranges_raw: Vec<Vec<i128>>) -> Self {
        let mut ranges: Vec<Range> = Vec::new();
        for range in ranges_raw {
            ranges.push(Range::new(range))
        }
        Self { name, ranges }
    }
}

#[derive(Debug)]
struct Range {
    // Calculate the entire ranges and store in Vec on struct initialisation
    // Sort the ranges in seperate Vecs based on the first element, 
    // then map both the source and dest range to a HashMap
    src_range: Vec<i128>,
    dst_range: Vec<i128>
}

impl Range {
    pub fn new(input: Vec<i128>) -> Self {
        Self { src_range: (input[1]..input[1]+input[2]).collect(), dst_range: (input[0]..input[0]+input[2]).collect() }
    }
}

fn main() {
    // Read the file
    let file_name = String::from("test.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    // Split the input into their respective blocks
    let input: Vec<String> = file_content.split("\n\n").map(String::from).to_owned().collect();

    // Get the seed numbers
    let binding = input.clone();
    let seeds: Vec<u128> = binding[0].split_once(" ").unwrap().1.split(" ").map(|s| s.parse().expect("parse error")).collect();
    
    // Initialize the map of maps (grandmap)
    let mut map_map: HashMap<i32, Map> = HashMap::new();

    // Loop over the blocks, create a map after each block
    let mut binding = input.clone();
    let mut map_id: i32 = 1;
    for block in &mut binding[1..input.len()] {
        let mut ranges: Vec<Vec<i128>> = Vec::new();
        let mut map_name: String = String::from("");
        // Get the name of the map and the ranges defined in it
        for line in block.lines() {
            if line.chars().next().map(char::is_numeric).unwrap_or(false)  {               
                ranges.push(line.split(" ").map(|s| s.parse().expect("parse error")).collect())
            } else {
                map_name = line.split_once(" ").unwrap().0.to_string()
                // map_name = line.split_once(" ").unwrap().0
            }
        }
        // Create new map and add to map_map
        let new_map: Map = Map::new(map_name.clone(), ranges);
        map_map.insert(map_id, new_map);
        map_id += 1;
    }

    for (map_name, map) in map_map {
        println!("Map name: {:?}", map_name);
        println!("Map: {:?}", map);
    }

}
