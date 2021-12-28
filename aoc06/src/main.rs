use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    println!("Part 1: {}", simulate_fish(input, 80));
}

fn part2(input: &str) {
    println!("Part 2: {}", simulate_fish(input, 256));
}

fn simulate_fish(input: &str, days: u32) -> u64 {
    let mut fish = [0u64; 9];
    for timer in input.trim().split(',').map(|x| x.parse::<u32>().unwrap()) {
        fish[timer as usize] += 1;
    }

    for _ in 0..days {
        let new_fish_count = fish[0];
        fish.copy_within(1..9, 0);
        fish[6] += new_fish_count;
        fish[8] = new_fish_count;
    }
    fish.iter().sum()
}
