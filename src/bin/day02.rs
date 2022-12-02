use std::io;

fn main() {
    let mut pt1_score = 0;
    let mut pt2_score = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let opponent = Play::from_opponent(line.chars().nth(0).unwrap());
        pt1_score += Play::from_you(line.chars().nth(2).unwrap()).score(&opponent);

        pt2_score += {
            let outcome = Game::new(line.chars().nth(2).unwrap());
            let your_play = outcome.reverse(&opponent);
            outcome as usize + your_play as usize
        };
    }
    println!("Part 1: {}", pt1_score);
    println!("Part 2: {}", pt2_score);
}

#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn from_opponent(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => todo!(),
        }
    }

    fn from_you(c: char) -> Self {
        match c {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => todo!(),
        }
    }

    fn score(&self, other: &Self) -> usize {
        *self as usize
            + match (self, other) {
                (Self::Rock, Self::Paper)
                | (Self::Paper, Self::Scissors)
                | (Self::Scissors, Self::Rock) => 0,
                (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
                | (Self::Paper, Self::Rock) => 6,
                _ => 3,
            }
    }
}

enum Game {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Game {
    fn new(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => todo!(),
        }
    }

    fn reverse(&self, opponent: &Play) -> Play {
        match (self, opponent) {
            (Self::Lose, Play::Rock) | (Self::Win, Play::Paper) | (Self::Draw, Play::Scissors) => {
                Play::Scissors
            }
            (Self::Lose, Play::Scissors) | (Self::Win, Play::Rock) | (Self::Draw, Play::Paper) => {
                Play::Paper
            }
            (Self::Lose, Play::Paper) | (Self::Win, Play::Scissors) | (Self::Draw, Play::Rock) => {
                Play::Rock
            }
        }
    }
}
