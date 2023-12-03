use std::{fs, env, char, ops::Add, thread::current};

#[derive(Copy, Clone, Debug)]
struct Point {
    line_num: u32,
    col_num: u32
}

impl Point {
    pub fn new(line_num: u32, col_num: u32) -> Self {
        Self { line_num, col_num }
    }
}

struct Special {
    coords: Point,
    neighbours: Vec<Point>,
    line_limit: i32,
    col_limit: i32,
    part_type: char,
    partnumbers: Vec<String>
}

impl Special {
    pub fn new(line: u32, col: u32, line_limit: i32, col_limit: i32, part_type: char) -> Self {
        
        // Find all neighbours of the special char
        let mut all_coords: Vec<Point> = Vec::new();
        let manips: &[(i32, i32)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for (line_manip, col_manip) in manips {
            let new_line = line as i32 + line_manip;
            let new_col = col as i32 + col_manip;
            if new_line <= line_limit && new_col <= col_limit && new_line >= 0 && new_col >= 0 {
                all_coords.push(Point::new(new_line as u32, new_col as u32));
            }
        };

        // Initialize obj
        Self {
            coords: Point::new(line, col),
            neighbours: all_coords,
            line_limit,
            col_limit,
            part_type,
            partnumbers: Vec::new()
        }
    }
}

fn file_to_vec(file: String) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        let line_chars = line.chars().collect();
        output.push(line_chars);
    };
    output
}

fn get_char_at_point(point: Point, engine: Vec<Vec<char>>) -> char {
    engine[point.line_num as usize][point.col_num as usize]
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_name = &args[1];
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    // PLAN:
    // 1. Load the engine into a 2d vector DONE
    // 2. Get the coords of all special characters DONE
    //      - Special chars: -%+=*/$&#@ (gotten from cyberchef)
    // 3. Check all coords around every special char for a numerical character. 
    //      - Neighbours are stored in the Special struct. 
    // 4. Then check all of those numerical chars forward and backwards until another dot is found.

    // Initialize the 2d vector with the input (the engine)
    let engine: Vec<Vec<char>> = file_to_vec(file_content);

    let line_limit: i32 = engine.clone().len() as i32;
    let col_limit: i32 = engine.clone().into_iter().nth(0).unwrap().len() as i32;

    // Find all special characters in the engine
    let special_chars = String::from("-%+=*/$&#@");
    let mut specials: Vec<Special> = Vec::new();
    for (line_num, line) in engine.clone().iter().enumerate() {
        for (col_num, char) in line.iter().enumerate() {
            if special_chars.contains(*char) {
                let new_special = Special::new(line_num as u32, col_num as u32, line_limit, col_limit, get_char_at_point(Point::new(line_num as u32, col_num as u32), engine.clone()));
                specials.push(new_special)
            }
        }
    }

    // Part 1
    // let mut part_numbers: Vec<String> = Vec::new();
    // Loop over the specials
    let mut total_value: u32 = 0;
    for special in specials {
        let mut parts: Vec<String> = Vec::new();
        // println!("Special at point: {:?}, Type: {:?}", special.coords, special.part_type);
        // Loop over it's neighbours
        for neighbour in special.neighbours.clone() {
            
            let char = get_char_at_point(neighbour, engine.clone());
            if char.is_numeric() {
                let mut part_number = String::new();
                let mut rev_pos = neighbour.col_num;
                let mut reverse = true;
                while reverse {
                    let new_char = get_char_at_point(Point::new(neighbour.line_num, rev_pos) , engine.clone());
                    if new_char.is_numeric() {
                        part_number = format!("{}{}", new_char, part_number);
                        if !(rev_pos == 0) {
                            rev_pos -= 1
                        } else {
                            break;
                        }
                    } else { reverse = false }
                }

                let mut fow_pos = neighbour.col_num + 1;
                let mut forward = true;
                while forward {
                    let new_char = get_char_at_point(Point::new(neighbour.line_num, fow_pos), engine.clone());
                    if new_char.is_numeric() {
                        part_number.push(new_char);
                        if !(fow_pos == (col_limit - 1) as u32) {
                            fow_pos += 1
                        } else {
                            break;
                        }
                    } else { forward = false }
                }
                parts.push(part_number);
            }
        }

        parts.dedup();
        if special.part_type == '*' && parts.len() == 2 {
            let gear_ratio1: u32 = parts[0].parse().unwrap();
            let gear_ratio2: u32 = parts[1].parse().unwrap();
            total_value += gear_ratio1 * gear_ratio2;
        }
    }

    // part_numbers.dedup();
    // let mut total_value: u32 = 0;
    // for number in part_numbers {
    //     let number_int: u32 = number.parse().unwrap();
    //     total_value += number_int
    // }
    println!("Total value: {:?}", total_value)

}
