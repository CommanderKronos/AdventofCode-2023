use std::{time::Instant, fs, collections::BTreeMap};

fn main() {
    let start = Instant::now();

    // Read the file
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let mut lines = file_content.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    println!("Instructions: {:?}", instructions);

    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    for line in lines {
        if !line.is_empty() {
            let (node_name, node_options) = line.split_once(" = ").unwrap();
            let new_node: Node = Node::new(node_options.to_owned());
            nodes.insert(node_name.to_owned(), new_node);
        }
    }

    println!("{:?}", nodes);

    let mut node_index = "AAA".to_owned();
    let mut dst_reached = false;
    let mut instruction_index: usize = 0;
    let mut steps: u32 = 0;
    while !dst_reached {
        if instruction_index >= instructions.len().try_into().unwrap() {
            instruction_index = 0
        }
        let cur_node = nodes.get(&node_index).unwrap();
        if instructions[instruction_index] == 'R' {
            node_index = cur_node.right.clone()
        } else if instructions[instruction_index] == 'L' {
            node_index = cur_node.left.clone()
        }

        steps += 1;
        if node_index == String::from("ZZZ") {
            dst_reached = true
        }
        instruction_index += 1;
    }

    println!("Steps needed: {:?}", steps);
    println!("Time elapsed: {:?}", start.elapsed());
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

impl Node {
    pub fn new(node_str: String) -> Self{
        let (left, right) = node_str
                                            .strip_prefix("(").unwrap()
                                            .strip_suffix(")").unwrap()
                                            .split_once(", ").unwrap();
        Self { left: left.to_owned(), right: right.to_owned() }
    }
}
