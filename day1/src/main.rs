use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: String = String::from("./input.txt"); 

    // Read the file
    let contents = fs::read_to_string(file_path)
        .expect("File error");

    // Cast the file content to a string slice of bytes
    let contents_bytes = contents.as_bytes().to_vec();

    // Split vec on each 10 byte to split into file lines
    let lines: Vec<Vec<u8>> = contents_bytes.into_iter().fold(Vec::new(), |mut acc, x| {
        if x == 10 || acc.is_empty() {
            acc.push(Vec::new());
        }
        acc.last_mut().unwrap().push(x);
        acc
    });


    let mut total_value: u16 = 0;
    // Iterate over all lines
    for byte_line in lines {

        let line: String = String::from_utf8(byte_line).unwrap();
        
        // Find each first occurence of the calibration value in line
        let mut finds_forward: HashMap<usize, &str> = HashMap::new();
        for check_slice in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            let check = line.find(check_slice);
            match check {
                Some(x) => finds_forward.insert(x, check_slice),
                None => continue,
            };
        }

        let mut finds_reverse: HashMap<usize, &str> = HashMap::new();
        for check_slice in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            let check = line.rfind(check_slice);
            match check {
                Some(x) => finds_reverse.insert(x, check_slice),
                None => continue,
            };
        }

        let keys_forward = Vec::from_iter(finds_forward.keys());
        let keys_reverse = Vec::from_iter(finds_reverse.keys());

        // Find lowest index in finds keys
        let min_value = match keys_forward.iter().min() {
            Some(min) => **min,
            None      => 0,
        };

        // Find highest index in finds keys
        let max_value = match keys_reverse.iter().max() {
            Some(max) => **max,
            None      => 0,
        };

        // Find calibration value for lowest index\
        let first_cal_digit = finds_forward.get(&min_value).unwrap();
        let last_cal_digit = finds_reverse.get(&max_value).unwrap();

        // Final conversion to the calibration value of line
        let cal_value_conv: HashMap<&str, u8> = HashMap::from([
            ("one", 49),
            ("two", 50),
            ("three", 51),
            ("four", 52),
            ("five", 53),
            ("six", 54),
            ("seven", 55),
            ("eight", 56),
            ("nine", 57),
            ("1", 49),
            ("2", 50),
            ("3", 51),
            ("4", 52),
            ("5", 53),
            ("6", 54),
            ("7", 55),
            ("8", 56),
            ("9", 57),
        ]);

        let cal_value_string: String = String::from_utf8(vec![*cal_value_conv.get(first_cal_digit).unwrap(), *cal_value_conv.get(last_cal_digit).unwrap()]).unwrap();
        let cal_value: u16 = cal_value_string.parse().unwrap();

        total_value += cal_value
        // println!("Calibration value: {:?}", cal_value);
        // println!("==============");

    }

    println!("Total value: {:?}", total_value)

    // This was the first attempt for the first puzzle
    // let mut total_value: u16 = 0;
    // // Iterate over all lines
    // for line in lines {
    //     let mut cal_value_vec: Vec<u8> = Vec::new();

    //     // Forward iterate to first occurence of byte between 48 and 57 inclusive
    //     for char_byte in line.iter() {
    //         if char_byte >= &48 && char_byte <= &57 {
    //             cal_value_vec.push(*char_byte);
    //             break;
    //         }
    //     }

    //     // Reverse iterate to last occurence of byte between 48 and 57 inclusive
    //     for char_byte in line.iter().rev() {
    //         if char_byte >= &48 && char_byte <= &57 {
    //             cal_value_vec.push(*char_byte);
    //             break;
    //         }
    //     }
        
    //     let cal_value_string: String = String::from_utf8(cal_value_vec).unwrap();
    //     let cal_value: u16 = cal_value_string.parse().unwrap();
    //     println!("Calibration value: {:?}", cal_value);

    //     total_value += cal_value

    // }

    // println!("Total value: {:?}", total_value)

    

}
