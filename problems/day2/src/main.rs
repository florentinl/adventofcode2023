#[derive(Debug, Default)]
struct Game {
    red: usize,
    blue: usize,
    green: usize,
}

fn main() {
    let inputs = include_str!("input.txt")
        .lines()
        .map(|line| {
            let game_record = line.split(": ").nth(1).unwrap();
            let rounds = game_record.split(", ");

            rounds.fold(Game::default(), |mut game, round| {
                round.split(", ").for_each(|color_count| {
                    let (score, color) = color_count.split_once(' ').unwrap();
                    let score = score.parse::<usize>().unwrap();
                    match color {
                        "red" => game.red = game.red.max(score),
                        "blue" => game.blue = game.blue.max(score),
                        "green" => game.green = game.green.max(score),
                        _ => unreachable!(),
                    }
                });
                game
            })
        })
        .collect::<Vec<_>>();

    let challenge1 = inputs.iter().enumerate().fold(0, |acc, pair| {
        let (index, game) = pair;
        if game.red <= 12 && game.blue <= 14 && game.green <= 13 {
            acc + index + 1
        } else {
            acc
        }
    });
    let challenge2 = inputs
        .iter()
        .fold(0, |acc, game| acc + (game.red * game.blue * game.green));
    println!("Challenge 1: {}", challenge1);
    println!("Challenge 2: {}", challenge2);
}
