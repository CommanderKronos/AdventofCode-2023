use std::{fs, env, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    // Part 1 value calc
    // Limits: 12 Red, 13 Green, 14 Blue
    // let limits: HashMap<&str, u32> = HashMap::from([
    //     ("red", 12),
    //     ("green", 13),
    //     ("blue", 14),    
    // ]);

    let mut total_value: u32 = 0;

    // Loop over the lines
    for line in filecontent.lines() {

        // Split off Game ID from the rest of the line 
        let (_id, rest) = match line.split_once(":") {
            Some((id, rest)) => (id, rest.strip_prefix(" ").unwrap()),
            None => break,
        };
        
        // Split the rest into the seperate reveals
        let reveals: Vec<&str> = rest.split("; ").collect();

        // Part 1 value calc
        // let mut possible: bool = true;

        // Part 2 minimum needed cubes
        let mut minimums: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);


        // Loop over the reveals
        for reveal in reveals {
            
            let values: Vec<&str> = reveal.split(", "). collect();

            for value in values {
                let (amount, color): (u32, &str) = match value.split_once(" ") {
                    Some((amount, color)) => (amount.parse().unwrap(), color),
                    None => break,
                };
                
                if &amount > minimums.get(color).unwrap() {
                    minimums.insert(color, amount);
                };

                // Part 1 value calc
                // if &amount > limits.get(color).unwrap() {
                //     possible = false
                // }
            };
        };

        let mut line_power: u32 = 1;
        for key in minimums.keys() {
            line_power  *= minimums.get(*key).unwrap()
        }
        
        println!("Updated minimums: {:?}, Line Power: {:?}", minimums, line_power);

        total_value += line_power

        // Part 1 value calc
        // if possible {
        //     total_value += match id.split_once(" ") {
        //         Some((_, id)) => id.parse().unwrap(),
        //         None => 0,
        //     }
        // }
    }

    println!("Total value: {:?}", total_value)
        

}
