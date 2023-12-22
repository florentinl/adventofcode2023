struct Race {
    distance: f64,
    time: f64,
}

// Time:        62     64     91     90
// Distance:   553   1010   1473   1074

fn main() {
    let races = [Race {
        time: 62649190.0,
        distance: 553101014731074.0,
    }];

    // let races = [
    //     Race {
    //         time: 62.0,
    //         distance: 553.0,
    //     },
    //     Race {
    //         time: 64.0,
    //         distance: 1010.0,
    //     },
    //     Race {
    //         time: 91.0,
    //         distance: 1473.0,
    //     },
    //     Race {
    //         time: 90.0,
    //         distance: 1074.0,
    //     },
    // ];

    // let races = [
    //     Race {
    //         time: 7.0,
    //         distance: 9.0,
    //     },
    //     Race {
    //         time: 15.0,
    //         distance: 40.0,
    //     },
    //     Race {
    //         time: 30.0,
    //         distance: 200.0,
    //     },
    // ];

    let challenge1: usize = races
        .iter()
        .map(|race| {
            let bounds = solution_bounds(race);
            println!("Bounds: {:?}", bounds);
            let count = all_integers_between(&bounds);
            count
        })
        .product();
    println!("Challenge 1: {:?}", challenge1);
}

fn solution_bounds(race: &Race) -> (f64, f64) {
    let d = race.distance;
    let t = race.time;
    let delta = (t * t) - (4.0 * d);
    ((t - delta.sqrt()) / 2.0, (t + delta.sqrt()) / 2.0)
}

fn all_integers_between((lower, upper): &(f64, f64)) -> usize {
    let start = lower.floor() as usize + 1;
    let end = upper.ceil() as usize - 1;
    (start..=end).count()
}
