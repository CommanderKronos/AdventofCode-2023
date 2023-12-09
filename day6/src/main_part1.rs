use std::{time::{Instant, Duration}, fs, collections::HashMap};

fn main() {
    let start = Instant::now();

    // Read the file
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let (times, distances) = file_content.split_once("\n").unwrap();
    let mut times: Vec<&str> = times.split_once(":").unwrap().1.trim().split(" ").collect();
    let mut distances: Vec<&str> = distances.split_once(":").unwrap().1.trim().split(" ").collect();
    times.retain(|&x| x != "");
    distances.retain(|&x| x != "");

    let times: Vec<u32> = times.iter().map(|x| x.parse().unwrap()).collect();
    let distances: Vec<u32> = distances.iter().map(|x| x.parse().unwrap()).collect();

    let races: HashMap<_, _> = times.iter().zip(distances.iter()).collect();

    println!("{:?}", times);
    println!("{:?}", distances);
    println!("{:?}", races);

    let mut total_wins: u32 = 1;
    for (max_time, record_distance) in races {
        let mut possible_wins: u32 = 0;
        for time_held_down in 1..*max_time {
            // speed == time_held_down
            // time_remaining == max_time - time_held_down
            // distance == speed * time_remaining
            let distance_travelled = time_held_down * (max_time - time_held_down);
            if distance_travelled > *record_distance {
                possible_wins += 1
            }
        }
        total_wins *= possible_wins
    }

    println!("Total wins: {:?}", total_wins);
    println!("Time elapsed: {:?}", start.elapsed());
}
