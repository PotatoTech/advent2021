use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let (first, remainder) = input.split_once("\n").unwrap();

    let mut prev_depth: i32 = first.parse().unwrap();
    let mut times_depth_increased = 0;

    for line in remainder.lines() {
        let depth: i32 = line.parse().unwrap();
        if depth > prev_depth {
            times_depth_increased += 1;
        }
        prev_depth = depth;
    }
    println!("Part 1: {}", times_depth_increased);
}

fn part2(input: &str) {
    let depths: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut prev_depth_sum: i32 = depths[0..3].iter().sum();
    let mut times_sum_increased = 0;

    for window in depths.windows(3).skip(1) {
        let depth_sum = window.iter().sum();
        if depth_sum > prev_depth_sum {
            times_sum_increased += 1;
        }
        prev_depth_sum = depth_sum;
    }
    println!("Part 2: {}", times_sum_increased);
}
