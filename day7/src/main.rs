use std::{fs, time::Instant, collections::HashMap};

fn main() {
    let start = Instant::now();

    let file_name = String::from("input.txt");
    let file_content: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening the file. Error: {:?}", error),
    };

    // Create a Hand object from all the lines and fill the all_hands vector with them
    let mut all_hands: Vec<Hand> = Vec::new();
    for line in file_content.lines() {
        let (card_str, bid_str) = line.split_once(" ").unwrap();
        all_hands.push(Hand::new(card_str, bid_str))
    }

    // Sort by (individual) card occurence_count first
    all_hands.sort_by(|hand1, hand2| hand1.card_occurence_count.cmp(&hand2.card_occurence_count));

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
    
    // Count occurences of each card label
    let mut occurences: HashMap<char, usize> = HashMap::new();
    for card in cards {
        *occurences.entry(card).or_default() += 1;
    }
    println!("=========================");
    println!("{:?}", occurences);

    // If occurences contains the key 'J', store it's value in the highest value after removing the entry
    if occurences.contains_key(&'J') {
        // Make sure the hand of cards isn't ONLY jokers (cheecky fuckers)
        if occurences.get(&'J').unwrap().to_owned() != 5 {
            let joker_value = occurences.remove(&'J').unwrap();
            let occurences_clone = occurences.clone();
            let max_value_key = occurences_clone
                                .iter()
                                .max_by(|a, b| a.1.cmp(&b.1))
                                .map(|(k, _v)| k).unwrap();
            *occurences.get_mut(max_value_key).unwrap() += joker_value;
        }
    }

    // Get the values for same-card occurences into a vector, sorted so there are only a
    // small amount of possible versions of the vector
    let mut occurence_count: Vec<usize> = occurences.clone()
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    occurence_count.sort();

    // occurence_count meaning:
    // [5] == five of a kind
    // [1, 4] == four of a kind
    // [1, 1, 3] == three of a kind
    // [2, 3] == full house
    // [1, 2, 2] == two pair
    // [1, 1, 1, 2] == one pair
    // [1, 1, 1, 1] == high card
    match occurence_count[..] {
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
    // Convert a vector of card labels to a vector of the related card label value
    let mut value_vec: Vec<u32> = Vec::new();
    for card in cards {
        value_vec.push(
            match card {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'T' => 10,
                'J' => 1,
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
    // call this function to assign a numerical value to the different types of hands
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
    card_occurence_count: Vec<u32>,
    hand_type: HandType,
    bid: u32
}

impl Hand {
    pub fn new(card_str: &str, bid_str: &str) -> Self {
        let card_vec: Vec<char> = card_str.chars().collect();
        let hand_type = check_type(card_vec.clone());
        let card_occurence_count = check_card_value(card_vec.clone());

        Self { cards: card_vec, card_occurence_count, hand_type, bid: bid_str.parse().unwrap() }
    }
}
