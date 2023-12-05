use std::{fs, collections::HashMap};
use itertools::Itertools;

#[derive(Clone)]
struct Card  {
    id: u32,
    instances: u32,
    wins: u32,
    winning_numbers: Vec<String>,
    gotten_numbers: Vec<String>
}

impl Card {
    pub fn new(id: u32, winning_numbers: Vec<String>, gotten_numbers: Vec<String>) -> Self {
        
        let mut wins: u32 = 0;
        for number in gotten_numbers.clone() {
            if winning_numbers.contains(&number) {
                wins += 1;
            }
        }

        Self { id, instances: 1, wins, winning_numbers, gotten_numbers }
    }

    pub fn add_instance(&mut self) {
        self.instances += 1;
    }
}


fn main() {
    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    // let mut cards: Vec<Card> = Vec::new();
    let mut cards: HashMap<u32, Card> = HashMap::new();

    for line in file_content.lines() {
        let (card_id, card_content) = line.split_once(": ").unwrap();
        let (winning_numbers, gotten_numbers) = card_content.split_once(" | ").unwrap();

        let mut winning_vec: Vec<String> = winning_numbers.split(" ").map(String::from).to_owned().collect();
        let mut gotten_vec: Vec<String> = gotten_numbers.split(" ").map(String::from).to_owned().collect();

        winning_vec.retain(|x| x != "");
        gotten_vec.retain(|x| x != "");

        let (_, id) = card_id.split_once(" ").unwrap();
        // let id: u32 = id.parse().unwrap();
        let id: u32 = match id.trim().parse() {
            Ok(x) => x,
            Err(err) => panic!("Error: {:?}, on digit: {:?}, in card: {:?}", err, id, card_id),
        };

        let new_card: Card = Card::new(id, winning_vec, gotten_vec);
        cards.insert(id, new_card);

    }

    let binding = cards.clone();
    let card_limit = binding.keys().max().unwrap();

    for card_id in cards.clone().keys().sorted() {
        
        for _ in 0..cards[card_id].instances {
            let mut index = cards[card_id].id;
            for _ in 0..cards[card_id].wins {
                index += 1;
                if index > *card_limit {
                    break;
                } else {
                    cards.get_mut(&index).unwrap().add_instance();
                }
            }
        }
    }

    let mut total_value: u32 = 0;
    for card_id in cards.clone().keys().sorted() {
        total_value += cards.get(card_id).unwrap().instances
        // println!("{:?}", cards.get(card_id).unwrap())
    }

    println!("Total card amount: {:?}", total_value)

}
