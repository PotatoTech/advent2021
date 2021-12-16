use std::io::{self, Read};

const INPUT_WIDTH: usize = 12;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let input: Vec<String> = input.split_whitespace().map(|x| x.into()).collect();
    let most_common_bits = count_bits(&input);

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for bit in most_common_bits {
        if bit == 0 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();
    println!("Part 1: {}", gamma * epsilon);
}

fn part2(input: &str) {
    let mut o2_gen_nums: Vec<String> = input.split_whitespace().map(|x| x.into()).collect();
    let mut co2_scrubber_nums = o2_gen_nums.clone();

    for i in 0..INPUT_WIDTH {
        let most_common_bits = count_bits(&o2_gen_nums);
        let mut j = 0;
        while o2_gen_nums.len() > 1 && j < o2_gen_nums.len() {
            if o2_gen_nums[j].as_bytes()[i] - b'0' != most_common_bits[i] {
                o2_gen_nums.remove(j);
            } else {
                j += 1;
            }
        }
    }

    for i in 0..INPUT_WIDTH {
        let most_common_bits = count_bits(&co2_scrubber_nums);
        let mut j = 0;
        while co2_scrubber_nums.len() > 1 && j < co2_scrubber_nums.len() {
            if co2_scrubber_nums[j].as_bytes()[i] - b'0' == most_common_bits[i] {
                co2_scrubber_nums.remove(j);
            } else {
                j += 1;
            }
        }
    }

    let o2_gen_rating = u32::from_str_radix(o2_gen_nums[0].as_str(), 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(co2_scrubber_nums[0].as_str(), 2).unwrap();
    println!("Part 2: {}", o2_gen_rating * co2_scrubber_rating);
}

fn count_bits(input: &[String]) -> [u8; INPUT_WIDTH] {
    let mut num_zeros = [0; INPUT_WIDTH];
    let mut num_ones = [0; INPUT_WIDTH];

    for line in input {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => num_zeros[i] += 1,
                '1' => num_ones[i] += 1,
                _ => (),
            }
        }
    }

    let mut output = [0; INPUT_WIDTH];
    for (i, (zeros, ones)) in num_zeros.iter().zip(num_ones.iter()).enumerate() {
        if zeros <= ones {
            output[i] = 1;
        }
    }
    output
}
