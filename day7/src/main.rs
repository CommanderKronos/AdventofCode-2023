use std::{fs, time::Instant, collections::{HashSet, HashMap}};

fn main() {
    let start = Instant::now();

    let file_name = String::from("type_test.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let mut all_hands: Vec<Hand> = Vec::new();
    for line in file_content.lines() {
        let (card_str, bid_str) = line.split_once(" ").unwrap();
        all_hands.push(Hand::new(card_str, bid_str))
    }


    for hand in all_hands {
        println!("{:?}", hand)
    }

    println!("Time elapsed: {:?}", start.elapsed())
}

fn check_type(cards: Vec<char>) -> HandType {
    
    let mut occurences: HashMap<char, usize> = HashMap::new();
    for card in cards {
        *occurences.entry(card).or_default() += 1;
    }
    println!("=========================");
    println!("{:?}", occurences);

    let mut values: Vec<usize> = occurences.clone()
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    values.sort();

    // Values meaning:
    // [5] == five of a kind
    // [1, 4] == four of a kind
    // [1, 1, 3] == three of a kind
    // [2, 3] == full house
    // [1, 2, 2] == two pair
    // [1, 1, 1, 2] == one pair
    // [1, 1, 1, 1] == high card
    match values[..] {
        [5] => HandType::Kind5,
        [1, 4] => HandType::Kind4,
        [1, 1, 3] => HandType::Kind3,
        [2, 3] => HandType::FullH,
        [1, 2, 2] => HandType::Pair2,
        [1, 1, 1, 2] => HandType::Pair1,
        [1, 1, 1, 1, 1] => HandType::High,
        _ => panic!("Shit went wrong")
    }
  
}

#[derive(Debug)]
enum HandType {
    Kind5,
    Kind4,
    FullH,
    Kind3,
    Pair2,
    Pair1,
    High
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: u32
}

impl Hand {
    pub fn new(card_str: &str, bid_str: &str) -> Self {
        let card_vec: Vec<char> = card_str.chars().collect();
        let hand_type = check_type(card_vec.clone());
        // let hand_type = HandType::FullH;

        Self { cards: card_vec, hand_type, bid: bid_str.parse().unwrap() }
    }
}
