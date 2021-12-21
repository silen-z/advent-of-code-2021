use std::convert::TryInto;

const INPUT: &'static str = include_str!("../../input/day04.txt");

const BOARD_SIZE: usize = 5;

fn main() {
    let mut boards = INPUT.split("\n\n");

    let numbers: Vec<u32> = boards
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let boards: Vec<Board> = boards.map(|b| b.parse().unwrap()).collect();

    println!("{}", part1(boards.clone(), &numbers));
    println!("{}", part2(boards, &numbers));
}

fn part1(mut boards: Vec<Board>, numbers: &[u32]) -> u32 {
    for n in numbers {
        for board in &mut boards {
            board.mark(*n);
            if board.is_win() {
                return board.score() * n;
            }
        }
    }
    panic!()
}

fn part2(mut boards: Vec<Board>, numbers: &[u32]) -> u32 {
    for n in numbers {
        boards.iter_mut().for_each(|b| b.mark(*n));

        if boards.len() == 1 && boards[0].is_win() {
            return boards[0].score() * n;
        }

        boards.retain(|b| !b.is_win());
    }
    panic!()
}

#[derive(Clone, Debug)]
struct Board {
    numbers: [u32; BOARD_SIZE * BOARD_SIZE],
    marked: [bool; BOARD_SIZE * BOARD_SIZE],
}

impl Board {
    fn mark(&mut self, marked: u32) {
        if let Some(pos) = self.numbers.iter().position(|n| *n == marked) {
            self.marked[pos] = true;
        }
    }

    fn is_win(&self) -> bool {
        for row in 0..BOARD_SIZE {
            let start = row * BOARD_SIZE;
            if (start..start + BOARD_SIZE).all(|i| self.marked[i]) {
                return true;
            }
        }

        for col in 0..BOARD_SIZE {
            if (col..BOARD_SIZE * BOARD_SIZE)
                .step_by(BOARD_SIZE)
                .all(|i| self.marked[i])
            {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(|(i, n)| (!self.marked[i]).then(|| n))
            .sum()
    }
}

impl std::str::FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Board {
            numbers: numbers.try_into().unwrap(),
            marked: [false; BOARD_SIZE * BOARD_SIZE],
        })
    }
}
