use std::collections::HashMap;
use std::cmp::Ordering;

pub fn run(file: &str) -> (usize,usize) {
    let mut data = read_data(file);
    data.sort();

    let mut data2 = read_data_2(file);
    data2.sort();
    (pt1(&data),pt2(&data2))
}

fn pt1(data: &[Hand]) -> usize {
    data.iter().enumerate().fold(0, |acc,(rank,x)| {
        acc + (rank+1)*x.bid
 } )
}
fn pt2(data: &[Hand]) -> usize {
    data.iter().enumerate().fold(0, |acc,(rank,x)| {
        acc + (rank+1)*x.bid
 } )
}

fn read_data(file: &str) -> Vec<Hand> {
    std::fs::read_to_string(file).expect("Failed to read file")
    .lines()
    .map(|line| {
        let mut l_iter = line.split_whitespace();
        let (hand,all_cards) = parse_cards(l_iter.next().expect("Failed to get hand"));
        let bid = l_iter.next().expect("Failed to get bid").parse::<usize>().expect("Failed to parse bid");
        Hand{
            cards: hand,
            all_cards,
            bid,
        }
    }).collect::<Vec<Hand>>()
}
fn read_data_2(file: &str) -> Vec<Hand> {
    std::fs::read_to_string(file).expect("Failed to read file")
    .lines()
    .map(|line| {
        let mut l_iter = line.split_whitespace();
        let (hand,all_cards) = parse_cards_2(l_iter.next().expect("Failed to get hand"));
        let bid = l_iter.next().expect("Failed to get bid").parse::<usize>().expect("Failed to parse bid");
        Hand{
            cards: hand,
            all_cards,
            bid,
        }
    }).collect::<Vec<Hand>>()
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    all_cards: Vec<Card>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            return cmp_vec_cards(&self.all_cards, &other.all_cards)
        }
        self.cards.cmp(&other.cards)
    }
}

fn cmp_vec_cards(cards: &Vec<Card>, other:&Vec<Card>) -> Ordering{
    for (i,card) in cards.iter().enumerate() {
        if *card != other[i] {
            return card.cmp(&other[i])
        }
    }
    Ordering::Equal
    }


impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

#[derive(PartialEq,PartialOrd,Debug,Ord,Eq)]
enum Cards {
    High,
    OneP,
    TwoP,
    ThreeK,
    FullH,
    FourK,
    FiveK,
}

#[derive(PartialEq,PartialOrd,Debug,Ord,Eq)]
enum Card {
    Numeric(usize),
    T,
    J,
    Q,
    K,
    A,
}

fn card_char_to_card(c: char) -> Card {
    if c.is_numeric() {
        return Card::Numeric(c.to_digit(10).expect("Failed to parse numeric card") as usize)
    }
    match c {
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        _ => Card::A,
    }
}

fn parse_cards(cards: &str) -> (Cards,Vec<Card>) {
    let mut map: HashMap<char,usize> = HashMap::new();
    let mut all_cards = Vec::new();
    let mut cards_iter = cards.chars();
    let first_card_char = cards_iter.next().expect("Failed to get first card");
    all_cards.push(card_char_to_card(first_card_char));

    map.entry(first_card_char).or_insert(1);
    for card in cards_iter {
        all_cards.push(card_char_to_card(card));
        *map.entry(card).or_insert(0) += 1;
    }

    let ret = match map.len() {
        1 => Cards::FiveK,
        2 => {
            if map.values().max().expect("Should have got a max value from keys") == &4 {
                Cards::FourK
            } else {
                Cards::FullH
            }
        },
        3 => {
            if map.values().max().expect("Should have got a max value from keys") == &3 {
                Cards::ThreeK
            } else {
                Cards::TwoP
            }
        },
        4 => Cards::OneP,
        _ => Cards::High,
    };
    (ret,all_cards)
}

fn parse_cards_2(cards: &str) -> (Cards,Vec<Card>) {
    let mut map: HashMap<char,usize> = HashMap::new();
    let mut all_cards = Vec::new();
    let cards_iter = cards.chars();

    for card in cards_iter {
        let fake_card = if card == 'J' {
            '1'
        } else {
            card
        };
        
        all_cards.push(card_char_to_card(fake_card));
        *map.entry(card).or_insert(0) += 1;
    }
    let jokers = if let Some(n) = map.remove(&'J') {
        n
    } else {
        0
    };
    //println!("Map: {:?}", map);
   // println!("Jokers: {:?}", jokers);
    let ret = match map.len() {
        0 => Cards::FiveK,
        1 => Cards::FiveK,
        2 => if jokers > 1 {
                Cards::FourK
            } else if jokers == 1 && map.values().max().expect("Should have got a max value from keys") == &3 {
                Cards::FourK
            } else if jokers == 1 {
                Cards::FullH
            } else if map.values().max().expect("Should have got a max value from keys") == &4 {
                Cards::FourK
             } else {
                Cards::FullH
             },
        3 => {
            if jokers > 0 {
                Cards::ThreeK
            } else if map.values().max().expect("Should have got a max value from keys") == &3 {
                Cards::ThreeK
            } else {
                Cards::TwoP
            }
        },
        4 => {
            Cards::OneP
        },
        _ => Cards::High,
    };
  //  println!("Ret: {:?}",ret);
    (ret,all_cards)
}