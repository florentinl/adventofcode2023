use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<usize>,
    bid: usize,
}

fn main() {
    challenge1();
    challenge2();
}

fn challenge1() {
    let mut hands: Vec<Hand> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            let cards = hand
                .chars()
                .map(|c| match c {
                    'A' => 1,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    _ => c.to_digit(10).unwrap() as usize,
                })
                .collect::<Vec<_>>();
            let map = to_hash_map(&cards);
            let hand_type = get_hand_type(&map);
            Hand {
                hand_type,
                cards,
                bid,
            }
        })
        .collect();
    hands.sort();

    let challenge1 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>();
    println!("Challenge 1: {}", challenge1);
}

fn challenge2() {
    let mut hands: Vec<Hand> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            let cards = hand
                .chars()
                .map(|c| match c {
                    'A' => 1,
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    _ => c.to_digit(10).unwrap() as usize,
                })
                .collect::<Vec<_>>();
            let map = to_hash_map(&cards);
            let hand_type = get_hand_type(&map);
            Hand {
                hand_type,
                cards,
                bid,
            }
        })
        .collect();
    hands.sort();
}

fn to_hash_map(cards: &[usize]) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for card in cards.iter() {
        *map.entry(*card).or_default() += 1;
    }
    map
}

fn get_hand_type(map: &HashMap<usize, usize>) -> HandType {
    match map.values().max().unwrap() {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if map.values().any(|&v| v == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if map.values().filter(|&v| *v == 2).count() == 2 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

fn get_hand_type_with_wildcard(map: &HashMap<usize, usize>) -> HandType {
    let js = map.get(&1).unwrap_or(&0);
    let map_without_jokers = map
        .iter()
        .filter(|(&k, _)| k != 1)
        .map(|(&k, &v)| (k, v))
        .collect::<HashMap<_, _>>();
    match (js, map_without_jokers.values().max().unwrap_or(&0)) {
        (5, 0) | (4, 1) | (3, 2) | (2, 3) => HandType::FiveOfAKind,
        (4, 0) => HandType::FourOfAKind,
        (3, 0) => HandType::ThreeOfAKind,
        (2, 0) => HandType::OnePair,
        (1, 0) => HandType::HighCard,
        (3, 1) => HandType::FullHouse,
        (2, 1) => HandType::TwoPairs,
        (1, 1) => HandType::OnePair,
        (0, 1) => HandType::HighCard,
        _ => panic!("Invalid hand"),
    }
}
