use std::{time::Instant, fs, collections::BTreeMap, sync::Mutex};
use rayon::prelude::*;

fn main() {
    let start = Instant::now();

    // Read the file
    let file_name = String::from("test2.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let mut lines = file_content.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    let mut a_nodes: Vec<String> = Vec::new();
    let mut z_nodes: Vec<String> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            let (node_name, node_options) = line.split_once(" = ").unwrap();
            let name_chars: Vec<char> = node_name.chars().collect();
            if name_chars[2] == 'A' {
                a_nodes.push(node_name.to_owned())
            } else if name_chars[2] == 'Z' {
                z_nodes.push(node_name.to_owned())
            }
            let new_node: Node = Node::new(node_options.to_owned());
            nodes.insert(node_name.to_owned(), new_node);
        }
    }

    println!("Start and Finish:");
    println!("{:?}", a_nodes);
    println!("{:?}", z_nodes);
    println!("============================================================");

    // Calculate how many cycles it takes to reach the z node per a node
    for a_node in a_nodes.clone() {
        let mut z_reached = false;
        let mut a_reached = false;
        let mut steps: u32 = 0;
        let mut instruction_index: usize = 0;
        let mut node_index = a_node.clone();
        let mut steps_to_z: u32 = 0;
        let mut steps_to_a: u32 = 0;
        while !(z_reached && a_reached) {
            if instruction_index >= instructions.len().try_into().unwrap() {
                instruction_index = 0;
            }

            let cur_node = nodes.get(&node_index).unwrap();
            if instructions[instruction_index] == 'R' {
                node_index = cur_node.right.clone()
            } else if instructions[instruction_index] == 'L' {
                node_index = cur_node.left.clone()
            }

            steps += 1;
            if node_index.chars().last().unwrap() == 'Z' {
                println!("It took node {} {} steps to get to node {}", a_node, steps, node_index);
                steps_to_z = steps;
                z_reached = true
            } else if node_index.chars().last().unwrap() == 'A' {
                println!("It took node {} {} steps to get to node {}", a_node, steps, node_index);
                steps_to_a = steps;
                a_reached = true
            }
            instruction_index += 1;
        }

        println!("It took node: {} {} steps to get to node {} and then another {} steps to node {} again", a_node, steps_to_z, node_index, steps_to_a, a_node)
    }

    println!("============================================================");

    let mut cur_nodes = a_nodes.clone();
    let mut dst_reached = false;
    let mut instruction_index: usize = 0;
    let mut steps: u32 = 0;
    while !dst_reached {
        if instruction_index >= instructions.len().try_into().unwrap() {
            instruction_index = 0
        }

        let mut new_nodes: Vec<String> = Vec::new();
        for node_str in &cur_nodes {
            let node = nodes.get(node_str).unwrap();
            if instructions[instruction_index] == 'R' {
                new_nodes.push(node.right.clone())
            } else if instructions[instruction_index] == 'L' {
                new_nodes.push(node.left.clone())
            }
        }
        steps += 1;
        instruction_index += 1;
        cur_nodes = new_nodes.clone();
        
        // println!("{:?}", cur_nodes);
        if cur_nodes == z_nodes {
            dst_reached = true
        }
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
