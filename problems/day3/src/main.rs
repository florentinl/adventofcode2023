pub fn main() {
    let map = include_bytes!("input.txt");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;

    let challenge_one = (0..map.len() - 2)
        .filter(|i| {
            map[*i].is_ascii_digit()
                && !map.get(i.wrapping_sub(1)).map_or(false, u8::is_ascii_digit)
        })
        .map(|i| {
            let d = (i + 1..i + 4)
                .position(|i| !map[i].is_ascii_digit())
                .unwrap()
                + 1;
            (i, d as _, atoi::atoi::<usize>(&map[i..i + d]).unwrap())
        })
        .filter(|(i, d, _n)| {
            (-width - 2..-width + *d)
                .chain([-1, *d])
                .chain(width..width + *d + 2)
                .any(|j| {
                    map.get((*i as isize + j) as usize)
                        .map_or(false, |b| b != &b'.' && b.is_ascii_punctuation())
                })
        })
        .map(|(_i, _d, n)| n)
        .sum::<usize>();

    let challenge_two = (0..map.len() - 2)
        .filter(|i| map[*i] == b'*')
        .filter_map(|i| {
            let mut nums = vec![];
            nums.extend(
                (-width - 2..=-width)
                    .chain([-1, 1])
                    .chain(width..=width + 2)
                    .map(|pos| (i as isize + pos) as usize)
                    .filter(|pos| map[*pos].is_ascii_digit())
                    .flat_map(|pos| {
                        (pos.saturating_sub(2)..=pos)
                            .rev()
                            .take_while(|i| map[*i].is_ascii_digit())
                            .last()
                    }),
            );
            nums.dedup();
            (nums.len() == 2).then(|| {
                nums.iter()
                    .map(|i| atoi::atoi::<usize>(&map[*i..*i + 3]).unwrap())
                    .product::<usize>()
            })
        })
        .sum::<usize>();
    println!("Challenge 1: {}", challenge_one);
    println!("Challenge 2: {}", challenge_two);
}
