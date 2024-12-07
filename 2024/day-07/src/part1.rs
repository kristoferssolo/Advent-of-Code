use miette::{Diagnostic, Result};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    const fn apply(self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }

    const fn all_operators() -> [Self; 2] {
        [Self::Add, Self::Multiply]
    }
}

#[derive(Debug, Error, Diagnostic)]
enum EquationError {
    #[error("Failed to parse equation")]
    ParseError,
    #[error("Missing value")]
    MissingValue,
}

#[derive(Debug)]
struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn find_result(&self) -> Option<usize> {
        fn recursive_find(
            numbers: &[usize],
            target: usize,
            current: usize,
            index: usize,
            is_first: bool,
        ) -> Option<usize> {
            if index == numbers.len() {
                return if current == target {
                    Some(current)
                } else {
                    None
                };
            }
            let num = numbers[index];

            for op in Operator::all_operators() {
                if let Some(result) = recursive_find(
                    numbers,
                    target,
                    if is_first {
                        num
                    } else {
                        op.apply(current, num)
                    },
                    index + 1,
                    false,
                ) {
                    return Some(result);
                }
            }

            None
        }
        if self.numbers.is_empty() {
            return None;
        }
        recursive_find(&self.numbers, self.result, 0, 0, true)
    }
}

impl FromStr for Equation {
    type Err = EquationError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let line = s.trim().split(':').collect::<Vec<_>>();
        let result = line
            .first()
            .ok_or(EquationError::MissingValue)?
            .parse()
            .map_err(|_| EquationError::ParseError)?;

        let numbers = line
            .last()
            .ok_or(EquationError::MissingValue)?
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| EquationError::ParseError)?;

        Ok(Self { result, numbers })
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let result = input
        .lines()
        .map(Equation::from_str)
        .filter_map(|eq| eq.ok()?.find_result())
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = 3749;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
