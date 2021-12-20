use std::collections::HashMap;
use std::io::{self, Read};

const BOARD_SIZE: usize = 5;

struct Board {
    board: HashMap<i32, (usize, usize)>,
    nums_unmarked_in_rows: [u32; BOARD_SIZE],
    nums_unmarked_in_cols: [u32; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            board: HashMap::new(),
            nums_unmarked_in_rows: [0; BOARD_SIZE],
            nums_unmarked_in_cols: [0; BOARD_SIZE],
        }
    }

    fn insert_num(&mut self, num: i32, row: usize, col: usize) {
        if self.board.insert(num, (row, col)).is_none() {
            self.nums_unmarked_in_rows[row] += 1;
            self.nums_unmarked_in_cols[col] += 1;
        }
    }

    fn mark_num(&mut self, num: i32) -> Option<i32> {
        if let Some(&(row, col)) = self.board.get(&num) {
            self.nums_unmarked_in_rows[row] -= 1;
            self.nums_unmarked_in_cols[col] -= 1;
            self.board.remove(&num);

            if self.nums_unmarked_in_rows[row] == 0 || self.nums_unmarked_in_cols[col] == 0 {
                let mut score = 0;
                for unmarked_num in self.board.keys() {
                    score += unmarked_num;
                }
                score *= num;

                return Some(score);
            }
        }

        None
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let (drawn_nums, mut boards) = parse_input(input);

    for num in drawn_nums {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark_num(num) {
                println!("Part 1: {}", score);
                return;
            }
        }
    }
}

fn part2(input: &str) {
    let (drawn_nums, mut boards) = parse_input(input);

    let mut last_score = 0;
    for num in drawn_nums {
        let mut i = 0;
        while i < boards.len() {
            if let Some(score) = boards.get_mut(i).unwrap().mark_num(num) {
                last_score = score;
                boards.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    println!("Part 2: {}", last_score);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let (drawn_nums, input) = input.split_once("\n\n").unwrap();
    let drawn_nums: Vec<i32> = drawn_nums.split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();
    for raw_board in input.split("\n\n") {
        let mut board = Board::new();
        for (row, nums_in_row) in raw_board.lines().enumerate() {
            for (col, num) in nums_in_row.split_whitespace().enumerate() {
                board.insert_num(num.parse().unwrap(), row, col);
            }
        }
        boards.push(board);
    }

    (drawn_nums, boards)
}
