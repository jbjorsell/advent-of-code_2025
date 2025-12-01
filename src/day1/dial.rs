use color_eyre::eyre::{self, bail};
use std::str::FromStr;

pub struct DialTurnResult {
    pub zeroes_passed: u32,
    pub final_value: u32,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn sign(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl FromStr for Direction {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => bail!("Unknown direction: {}", s),
        }
    }
}

pub struct DialTurn {
    direction: Direction,
    amount: u32,
}

impl FromStr for DialTurn {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        let (dir_str, amount_str) = s.split_at(1);
        let direction = dir_str.parse()?;
        let amount = amount_str.parse()?;
        Ok(Self { direction, amount })
    }
}

pub struct Dial {
    value: u32,
}

impl Dial {
    pub fn new() -> Self {
        Self { value: 50 }
    }

    pub fn turn(&mut self, turn: DialTurn) -> DialTurnResult {
        let full_laps = turn.amount / 100;
        let value_mod = turn.amount % 100;

        let left_turn_passed_zero = turn.direction == Direction::Left && value_mod >= self.value;
        let right_turn_passed_zero =
            turn.direction == Direction::Right && (self.value + value_mod) >= 100;
        let passed_zero = self.value != 0 && (left_turn_passed_zero || right_turn_passed_zero);
        let zeroes_passed = full_laps + passed_zero as u32;

        let unbounded_value = self.value as i32 + turn.direction.sign() * value_mod as i32;
        self.value = unbounded_value.rem_euclid(100) as u32;
        assert!(self.value < 100);

        DialTurnResult {
            zeroes_passed,
            final_value: self.value,
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}
