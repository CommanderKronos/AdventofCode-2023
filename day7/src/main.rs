use std::{fs, time::Instant, collections::HashMap};

fn main() {
    let start = Instant::now();

    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    let mut all_hands: Vec<Hand> = Vec::new();
    for line in file_content.lines() {
        let (card_str, bid_str) = line.split_once(" ").unwrap();
        all_hands.push(Hand::new(card_str, bid_str))
    }

    // Sort by card values first
    all_hands.sort_by(|hand1, hand2| hand1.card_values.cmp(&hand2.card_values));

    // Sort by hand_type value
    all_hands.sort_by(|hand1, hand2| hand1.hand_type.value().cmp(&hand2.hand_type.value()));

    let mut total_winnings: u32 = 0;
    for (rank, hand) in all_hands.iter().enumerate() {
        total_winnings += hand.bid * (rank as u32 + 1);
        println!("Rank: {:?}, Hand: {:?}", rank+1, hand)
    }

    println!("Total_winnings: {:?}", total_winnings);

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

fn check_card_value(cards: Vec<char>) -> Vec<u32> {
    let mut value_vec: Vec<u32> = Vec::new();
    for card in cards {
        value_vec.push(
            match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => card.to_digit(10).unwrap()
            }
        )
    }

    value_vec
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

impl HandType {
    const fn value(&self) -> u32 {
        match *self {
            HandType::Kind5 => 7,
            HandType::Kind4 => 6,
            HandType::FullH => 5,
            HandType::Kind3 => 4,
            HandType::Pair2 => 3,
            HandType::Pair1 => 2,
            HandType::High => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    card_values: Vec<u32>,
    hand_type: HandType,
    bid: u32
}

impl Hand {
    pub fn new(card_str: &str, bid_str: &str) -> Self {
        let card_vec: Vec<char> = card_str.chars().collect();
        let hand_type = check_type(card_vec.clone());
        let card_values = check_card_value(card_vec.clone());

        Self { cards: card_vec, card_values, hand_type, bid: bid_str.parse().unwrap() }
    }
}
