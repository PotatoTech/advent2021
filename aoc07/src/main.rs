use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let (positions, min, max) = parse_input(input);

    let mut cost = u32::MAX;
    for i in min..max {
        let mut cur_cost = 0;
        for pos in positions.iter() {
            cur_cost += i32::unsigned_abs(i - pos);
        }
        if cur_cost < cost {
            cost = cur_cost;
        }
    }
    println!("Part 1: {}", cost);
}

fn part2(input: &str) {
    let (positions, min, max) = parse_input(input);

    let mut cost = u32::MAX;
    for i in min..max {
        let mut cur_cost = 0;
        for pos in positions.iter() {
            let steps = i32::unsigned_abs(i - pos);
            cur_cost += steps * (steps + 1) / 2;
        }
        if cur_cost < cost {
            cost = cur_cost;
        }
    }
    println!("Part 2: {}", cost);
}

fn parse_input(input: &str) -> (Vec<i32>, i32, i32) {
    let positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (positions, min, max)
}
