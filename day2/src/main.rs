use std::{fs, env, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    println!("{}", filecontent);
    println!("=========================");

    // Limits: 12 Red, 13 Green, 14 Blue
    let limits: HashMap<&str, u16> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),    
    ]);

    let mut total_value = 0;

    // Loop over the lines
    for line in filecontent.lines() {

        // Split off Game ID from the rest of the line 
        let (id, rest) = match line.split_once(":") {
            Some((id, rest)) => (id, rest.strip_prefix(" ").unwrap()),
            None => break,
        };
        
        // Split the rest into the seperate reveals
        let reveals: Vec<&str> = rest.split("; ").collect();

        println!("Game ID: {:?}", id);
        println!("Games: {:?}", reveals);

        let mut possible: bool = true;

        // Loop over the reveals
        for reveal in reveals {
            
            let values: Vec<&str> = reveal.split(", "). collect();
            println!("Values: {:?}", values);

            for value in values {
                let (amount, color): (u16, &str) = match value.split_once(" ") {
                    Some((amount, color)) => (amount.parse().unwrap(), color),
                    None => break,
                };

                if &amount > limits.get(color).unwrap() {
                    possible = false
                }
            };
        };

        if possible {
            total_value += match id.split_once(" ") {
                Some((_, id)) => id.parse().unwrap(),
                None => 0,
            }
        }

        println!("=========================");
    }

    println!("Total value: {:?}", total_value)
        

}
