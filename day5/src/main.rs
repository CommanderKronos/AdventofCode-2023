use std::{iter::zip, {fs, collections::HashMap}, cmp::min};
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    id: i32,
    name: String,
    ranges: Vec<Range>
}

impl Map {
    pub fn new(id: i32, name: String, ranges_raw: Vec<Vec<u64>>) -> Self {
        let mut ranges: Vec<Range> = Vec::new();
        for range in ranges_raw {
            ranges.push(Range::new(range))
        }
        Self { id, name, ranges }
    }

    pub fn calc_map(&mut self) -> HashMap<u64, u64> {
        // Sort the source ranges

        self.ranges.sort_by_key(|x| x.src_range[0]);

        let mut tuples: Vec<Vec<(u64, u64)>> = Vec::new();
        for range in &self.ranges {
            tuples.push(zip(range.src_range.clone(), range.dst_range.clone()).collect());
            // tuples.append(range.src_range.iter().zip(range.dst_range.iter()).collect());
        }

        let out: HashMap<u64, u64> = tuples.concat().into_iter().collect();
        out
    }


}

#[derive(Debug, Clone)]
struct Range {
    // Calculate the entire ranges and store in Vec on struct initialisation
    // Sort the ranges in seperate Vecs based on the first element, 
    // then map both the source and dest range to a HashMap
    src_range: Vec<u64>,
    dst_range: Vec<u64>
}

impl Range {
    pub fn new(input: Vec<u64>) -> Self {
        println!("hier3");
        Self { src_range: (input[1]..input[1]+input[2]).collect(), dst_range: (input[0]..input[0]+input[2]).collect() }
    }
}

fn main() {
    // Read the file
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    // Split the input into their respective blocks
    let input: Vec<String> = file_content.split("\n\n").map(String::from).to_owned().collect();

    // Get the seed numbers
    let binding = input.clone();
    let seeds: Vec<u64> = binding[0].split_once(" ").unwrap().1.split(" ").map(|s| s.parse().expect("parse error")).collect();

    // Vec of maps
    // let mut map_vec: Vec<Map> = Vec::new();

    let mut seed_trans: Vec<u64> = seeds;

    // Loop over the blocks, create a map after each block
    let mut binding = input.clone();
    let mut map_id: i32 = 1; 
    for block in &mut binding[1..input.len()] {
        let mut map_to_end_all_maps: HashMap<u64, u64> = HashMap::new();
        println!("hier1");
        let mut location_values: Vec<u64> = Vec::new();
        // let mut ranges: Vec<Vec<u64>> = Vec::new();
        // let mut map_name: String = String::from("");
        // Get the name of the map and the ranges defined in it
        for line in block.lines() {
            if line.chars().next().map(char::is_numeric).unwrap_or(false)  {               
                // ranges.push(line.split(" ").map(|s| s.parse().expect("parse error")).collect())
                let range_bounds: Vec<u64> = line.split(" ").map(|s| s.parse().expect("parse error")).collect();
                let mut count = 0;
                println!("hier2");
                for source_number in range_bounds[1]..range_bounds[1]+range_bounds[2] {
                    let destination_number = range_bounds[0] + count;
                    map_to_end_all_maps.insert(source_number, destination_number);
                    count += 1;
                }
            } 
            // else {
            //     map_name = line.split_once(" ").unwrap().0.to_string()
            //     // map_name = line.split_once(" ").unwrap().0
            // }
        }
        println!("hier3");
        // Create new map and add to map_vec
        // let mut new_map: Map = Map::new(map_id, map_name.clone(), ranges);

        for seed in &seed_trans {
            
            // let translation_map = new_map.calc_map();
            // let temp = match translation_map.get(&seed) {
            //     Some(x) => x,
            //     None => seed,
            // };
            let temp = match map_to_end_all_maps.get(seed) {
                Some(x) => x,
                None => seed,
            };
            location_values.push(*temp);
            println!("hier4");
        }
        seed_trans = location_values;
        println!("Lowest translated value: {:?}", seed_trans.iter().min());

        // map_vec.push(new_map);
        map_id += 1;
    }

    // map_vec.sort_by_key(|x| x.id);

    // println!("Seeds: {:?}", seeds);
    // println!("========================================================================");

    // let mut location_values: Vec<u64> = Vec::new();
    // for seed in seeds {
    //     let mut seed_trans = seed;
    //     for map in &mut map_vec {
    //         let translation_map = map.calc_map();
    //         seed_trans = match translation_map.get(&seed_trans) {
    //             Some(x) => x.to_owned(),
    //             None => seed_trans,
    //         }
    //     }
    //     location_values.push(seed_trans)
    // }

}
