use itertools::Itertools;
use std::{ops::Add, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Joltage(u8);

impl Add for Joltage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 * 10 + rhs.0)
    }
}

impl Add for &Joltage {
    type Output = Joltage;
    fn add(self, rhs: Self) -> Self::Output {
        Joltage(self.0 * 10 + rhs.0)
    }
}

impl From<Joltage> for usize {
    fn from(value: Joltage) -> Self {
        Self::from(value.0)
    }
}

impl TryFrom<char> for Joltage {
    type Error = String;
    fn try_from(ch: char) -> std::result::Result<Self, Self::Error> {
        let s = ch.to_string();
        let value = s.parse::<u8>().map_err(|e| e.to_string())?;
        Ok(Self(value))
    }
}
#[derive(Debug, Clone)]
struct Bank(Vec<Joltage>);

impl Bank {
    fn get_max_value(&self) -> usize {
        let mut clone = self.0.clone();
        clone.truncate(self.0.len() - 1);
        let max1 = clone.iter().max().unwrap();
        let (pos1, _) = self.0.iter().find_position(|&x| x == max1).unwrap();
        let list = self.0.clone().split_off(pos1 + 1);
        let max2 = list.iter().max().unwrap();
        (max1 + max2).into()
    }
}

impl FromStr for Bank {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bank = s
            .trim()
            .chars()
            .map(Joltage::try_from)
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self(bank))
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let banks = input
        .lines()
        .map(|line| Bank::from_str(line).unwrap())
        .map(|bank| bank.get_max_value())
        .sum();
    Ok(banks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        let result = 357;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
