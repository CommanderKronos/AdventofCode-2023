use std::{time::Instant, fs};

fn main() {
    let start = Instant::now();

    // Read the file
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let (times, distances) = file_content.split_once("\n").unwrap();
    let max_time: String = times.split_once(":").unwrap().1.trim().chars().filter(|x| !x.is_whitespace()).collect();
    let distance: String = distances.split_once(":").unwrap().1.trim().chars().filter(|x| !x.is_whitespace()).collect();

    let max_time: u128 = max_time.parse().unwrap();
    let distance: u128 = distance.parse().unwrap();

    let mut total_wins: u128 = 0;
    for time_held_down in 1..max_time {
        // speed == time_held_down
        // time_remaining == max_time - time_held_down
        // distance == speed * time_remaining
        let distance_travelled = time_held_down * (max_time - time_held_down);
        if distance_travelled > distance {
            total_wins += 1
        }
    }

    println!("Total wins: {:?}", total_wins);
    println!("Time elapsed: {:?}", start.elapsed());
}
