use miette::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            _ => Err("Wrong value".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    amount: i32,
    direction: Direction,
}

impl FromStr for Rotation {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = s.trim();
        let (direction_str, amount_str) = trimmed.split_at(1);
        let direction = direction_str.parse()?;
        let amount = amount_str.parse::<i32>().map_err(|e| e.to_string())?;

        Ok(Self { amount, direction })
    }
}

#[derive(Debug, Clone)]
struct Sequence {
    rotations: Vec<Rotation>,
}

impl FromStr for Sequence {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rotations = s
            .trim()
            .lines()
            .map(|line| line.parse::<Rotation>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self { rotations })
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> Result<usize> {
    let sequence = input.parse::<Sequence>().unwrap();
    let (_, total_zeros) = sequence
        .rotations
        .iter()
        .fold((50, 0), |(dial, total), rotation| {
            let new_dial = match rotation.direction {
                Direction::Left => (dial - rotation.amount).rem_euclid(100),
                Direction::Right => (dial + rotation.amount).rem_euclid(100),
            };

            let zeros_crossed = count_zeros_crossed(dial, rotation.amount, rotation.direction);
            let new_total = total + zeros_crossed;
            (new_dial, new_total)
        });
    Ok(total_zeros.try_into().unwrap())
}

const fn count_zeros_crossed(pos: i32, amount: i32, direction: Direction) -> i32 {
    match direction {
        Direction::Left => {
            if pos == 0 {
                amount / 100
            } else if amount >= pos {
                1 + (amount - pos) / 100
            } else {
                0
            }
        }
        Direction::Right => {
            if pos == 0 {
                amount / 100
            } else {
                let target = 100 - pos;
                if amount >= target {
                    1 + (amount - target) / 100
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let result = 6;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
