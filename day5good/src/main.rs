use std::fs;

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
    
    let mut seed_trans: Vec<u64> = seeds;

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
        
        let mut location_values: Vec<u64> = Vec::new();

        for seed in &seed_trans {
            let mut special_trans: bool = false;
            let mut needed_vec: &Vec<u64> = &Vec::new();
            for range in &range_vec {
                let src_start: u64 = range[1];
                let lenght: u64 = range[2];

                if (src_start..src_start+lenght).contains(seed) {
                    special_trans = true;
                    needed_vec = range;
                }
            }
            if special_trans == true && !(needed_vec.is_empty()) {
                let new_number = needed_vec[0] + (seed - needed_vec[1]);
                location_values.push(new_number)
            } else {
                location_values.push(*seed)
            }
        }

        seed_trans = location_values;
        println!("Lowest translated value: {:?}", seed_trans.iter().min());
        }

}
