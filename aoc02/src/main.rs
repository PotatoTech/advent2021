use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().unwrap();
        let distance: i32 = tokens.next().unwrap().parse().unwrap();

        if direction == "forward" {
            horizontal_pos += distance;
        } else if direction == "down" {
            depth += distance;
        } else if direction == "up" {
            depth -= distance;
        }
    }
    println!("Part 1: {}", horizontal_pos * depth);
}

fn part2(input: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().unwrap();
        let distance: i32 = tokens.next().unwrap().parse().unwrap();

        if direction == "forward" {
            horizontal_pos += distance;
            depth += aim * distance;
        } else if direction == "down" {
            aim += distance;
        } else if direction == "up" {
            aim -= distance;
        }
    }
    println!("Part 2: {}", horizontal_pos * depth);
}
