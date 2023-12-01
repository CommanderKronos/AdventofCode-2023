use std::fs;

fn main() {
    let file_path: String = String::from("./input.txt"); 
    println!("Opening file {}", file_path);

    // Read the file
    let contents = fs::read_to_string(file_path)
        .expect("File error");

    println!("{}", contents);
    for c in contents.chars() {
        println!("Char: {}, Byte: {}", c, c as u8);
    }

    // Cast the file content to a string slice of bytes
    let contents_bytes = contents.as_bytes().to_vec();
    println!("{:?}", contents_bytes);

    // Split vec on each 10 byte to split into file lines
    let lines: Vec<Vec<u8>> = contents_bytes.into_iter().fold(Vec::new(), |mut acc, x| {
        if x == 10 || acc.is_empty() {
            acc.push(Vec::new());
        }
        acc.last_mut().unwrap().push(x);
        acc
    });
    println!("{:?}", lines);

    let mut total_value: u16 = 0;
    // Iterate over all lines
    for line in lines {
        let mut cal_value_vec: Vec<u8> = Vec::new();

        // Forward iterate to first occurence of byte between 48 and 57 inclusive
        for char_byte in line.iter() {
            if char_byte >= &48 && char_byte <= &57 {
                cal_value_vec.push(*char_byte);
                break;
            }
        }

        // Reverse iterate to last occurence of byte between 48 and 57 inclusive
        for char_byte in line.iter().rev() {
            if char_byte >= &48 && char_byte <= &57 {
                cal_value_vec.push(*char_byte);
                break;
            }
        }
        
        let cal_value_string: String = String::from_utf8(cal_value_vec).unwrap();
        let cal_value: u16 = cal_value_string.parse().unwrap();
        println!("Calibration value: {:?}", cal_value);

        total_value += cal_value

    }

    println!("Total value: {:?}", total_value)

}
