use std::fs;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

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
    let seeds: Vec<&[u64]> = seeds.chunks(2).collect();

    let mut seed_ranges: Vec<Vec<u64>> = Vec::new();
    for seed_range in seeds {
        seed_ranges.push(seed_range.to_vec())
    }

    println!("Passed: INIT");

    let mut all_seeds: Vec<Vec<u64>> = Vec::new();
    for bounds in &seed_ranges {
        all_seeds.push((bounds[0]..bounds[0]+bounds[1]).collect());       
    }
    all_seeds.sort();

    println!("Passed: SEED_VEC_CREATE");

    let mut binding = input.clone();
    for block in &mut binding[1..input.len()] {
        let mut range_vec: Vec<Vec<u64>> = Vec::new();
        for line in block.lines() {
            if line.chars().next().map(char::is_numeric).unwrap_or(false) {

                // range_bounds[0] == dst_range start, range_bounds[1] == src_range start, range_bounds[2] == range length
                // given source range 33 34 35 36 37 38, start 1, length 6
                // given dst    range 21 22 23 24 25 26, start 4, length 6
                // number 35 == 23.  35 - 33 = 2, 21 + 2 == 23
                // AKA to translate input to output, if input is in range source start to source start plus length
                // do input minus source range start, dst range start plus answer == translated
                let range_bounds: Vec<u64> = line.split(" ").map(|s| s.parse().expect("parse error")).collect();
                range_vec.push(range_bounds);
            }
        }

        println!("Passed: BLOCK_PARSE");

        println!("You're not going crazy");
        println!("You're not going crazy");
        println!("You're not going crazy");
        println!("You're not going crazy");

        let mut new_all_seeds: Vec<Vec<u64>> = Vec::new();
        println!("Trust me");
        println!("Trust me");
        println!("");
        
        for seed_vec in all_seeds {
            print!("=vec");
            let mut new_seed_vec: Vec<u64> = Vec::new();
            for seed in seed_vec {
                let mut special_trans: bool = false;
                let mut needed_vec: Vec<u64> = Vec::new();
                for range in &range_vec {
                    let src_start: u64 = range[1];
                    let length: u64 = range[2];

                    if (src_start..src_start+length).contains(&seed) {
                        special_trans = true;
                        needed_vec = range.to_vec();
                    }
                }

                if special_trans && !(needed_vec.is_empty()) {
                    let new_number = needed_vec[0] + (seed - needed_vec[1]);
                    new_seed_vec.push(new_number);
                } else {
                    new_seed_vec.push(seed);
                }
            }
            new_all_seeds.push(new_seed_vec);
        }
        println!("");

        all_seeds = new_all_seeds;
    }

    let mut minimums: Vec<u64> = Vec::new();
    for seed_vec in all_seeds {
        minimums.push(*seed_vec.iter().min().unwrap())
    }

    let duration = start.elapsed();
    println!("The minimum Location value is: {:?}, This was calculated in: {:?}", minimums.iter().min().unwrap(), duration)

}
