use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    println!("===============");

    let mut total_value = 0;
    for line in file_content.lines() {
        let (_, card_content) = line.split_once(": ").unwrap();
        let (winning_numbers, gotten_numbers) = card_content.split_once(" | ").unwrap();

        let mut winning_vec: Vec<&str> = winning_numbers.split(" ").collect();
        let mut gotten_vec: Vec<&str> = gotten_numbers.split(" ").collect();

        winning_vec.retain(|&x| x != "");
        gotten_vec.retain(|&x| x != "");
        
        println!("===============");
        println!("Winning Numbers: {:?}, Gotten Numbers: {:?}", winning_vec, gotten_vec);
        
        let mut count = 0;
        for number in gotten_vec {
            if winning_vec.contains(&number) {
                count += 1;
            }
        }

        let mut points = 0;

        if count != 0 {
            if count == 1 {
                points = 1
            } else {
                points = 1;
                for _ in 0..count-1 {
                    points *= 2
                }
            }
        }

        println!("Winning numbers: {:?}, Points: {:?}", count, points);
        total_value += points
        

    }

    println!("Total value: {:?}", total_value);
}
