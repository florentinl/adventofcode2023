use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    card_numbers: HashSet<usize>,
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning_numbers, ticket_numbers) = numbers.split_once(" | ").unwrap();
            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            let card_numbers = ticket_numbers
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            Card {
                winning_numbers,
                card_numbers,
            }
        })
        .collect::<Vec<Card>>();

    let challenge1 = input
        .iter()
        .map(|card| {
            let intersection = card
                .winning_numbers
                .intersection(&card.card_numbers)
                .count();
            if intersection == 0 {
                0
            } else {
                (2 as usize).pow(intersection as u32 - 1)
            }
        })
        .sum::<usize>();

    println!("Challenge 1: {}", challenge1);

    let mut counts = Vec::new();
    counts.resize(input.len(), 1);
    for (i, card) in input.iter().enumerate() {
        let matches = card
            .winning_numbers
            .intersection(&card.card_numbers)
            .count();
        let card_count = counts[i];
        for j in 1..=matches {
            if i + j >= counts.len() {
                break;
            }
            counts[i + j] += card_count;
        }
    }
    let challenge2: usize = counts.iter().sum();
    println!("Challenge 2: {}", challenge2)
}
