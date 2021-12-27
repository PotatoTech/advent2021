use std::collections::HashMap;
use std::io::{self, Read};

struct VentLine {
    start: (i32, i32),
    end: (i32, i32),
}

impl VentLine {
    fn points(&self) -> Vec<(i32, i32)> {
        fn range(a: i32, b: i32) -> Vec<i32> {
            if a <= b {
                (a..=b).collect()
            } else {
                (b..=a).rev().collect()
            }
        }

        let mut points = Vec::new();
        if self.start.0 == self.end.0 {
            for y in range(self.start.1, self.end.1) {
                points.push((self.start.0, y));
            }
        } else if self.start.1 == self.end.1 {
            for x in range(self.start.0, self.end.0) {
                points.push((x, self.start.1));
            }
        } else {
            for (x, y) in range(self.start.0, self.end.0)
                .into_iter()
                .zip(range(self.start.1, self.end.1).into_iter())
            {
                points.push((x, y));
            }
        }

        points
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let vent_lines: Vec<VentLine> = input
        .lines()
        .filter_map(|x| {
            let vent_line = parse_vent_line(x);
            if vent_line.start.0 == vent_line.end.0 || vent_line.start.1 == vent_line.end.1 {
                Some(vent_line)
            } else {
                None
            }
        })
        .collect();

    let mut vent_field = HashMap::<(i32, i32), u32>::new();
    for line in vent_lines {
        for point in line.points() {
            vent_field.entry(point).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let mut overlapping_points = 0;
    for _ in vent_field.values().filter(|x| **x >= 2) {
        overlapping_points += 1;
    }
    println!("Part 1: {}", overlapping_points);
}

fn part2(input: &str) {
    let vent_lines: Vec<VentLine> = input.lines().map(|x| parse_vent_line(x)).collect();

    let mut vent_field = HashMap::<(i32, i32), u32>::new();
    for line in vent_lines {
        for point in line.points() {
            vent_field.entry(point).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let mut overlapping_points = 0;
    for _ in vent_field.values().filter(|x| **x >= 2) {
        overlapping_points += 1;
    }
    println!("Part 2: {}", overlapping_points);
}

fn parse_vent_line(vent_line: &str) -> VentLine {
    let (start, end) = vent_line.split_once(" -> ").unwrap();
    let (x1, y1) = start.split_once(',').unwrap();
    let (x2, y2) = end.split_once(',').unwrap();

    let x1: i32 = x1.parse().unwrap();
    let y1: i32 = y1.parse().unwrap();
    let x2: i32 = x2.parse().unwrap();
    let y2: i32 = y2.parse().unwrap();

    VentLine {
        start: (x1, y1),
        end: (x2, y2),
    }
}
