const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut part1: usize = 0;
    for line in &lines {
        let first = line.find(char::is_numeric).unwrap();
        let first_digit = line[first..=first].parse::<usize>().unwrap();
        let last = line.rfind(char::is_numeric).unwrap();
        let last_digit = line[last..=last].parse::<usize>().unwrap();
        part1 += first_digit * 10 + last_digit;
    }
    println!("{part1}");
    let mut part2 = 0;
    for line in lines {
        let [first, last] = find(line);
        part2 += first * 10 + last;
    }
    println!("{part2}");
}

fn find(line: &str) -> [usize; 2] {
    let mut first = None;
    let mut last = None;

    let mut set = |d| {
        first = first.or(Some(d));
        last = Some(d);
    };

    for i in 0..line.len() {
        if line[i..].starts_with(char::is_numeric) {
            set(line[i..=i].parse::<usize>().unwrap());
        }
        for (d, digit) in DIGITS.iter().enumerate() {
            if line[i..].starts_with(digit) {
                set(d);
            }
        }
    }

    [first.unwrap(), last.unwrap()]
}
