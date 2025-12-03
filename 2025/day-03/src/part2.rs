use std::{ops::Add, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Joltage(usize);

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
        value.0
    }
}

impl TryFrom<char> for Joltage {
    type Error = String;
    fn try_from(ch: char) -> std::result::Result<Self, Self::Error> {
        let s = ch.to_string();
        let value = s.parse::<usize>().map_err(|e| e.to_string())?;
        Ok(Self(value))
    }
}
#[derive(Debug, Clone)]
struct Bank(Vec<Joltage>);

impl Bank {
    const LEN: usize = 12;

    fn get_max_value(&self) -> usize {
        if self.0.len() < Self::LEN {
            return self.0.iter().map(|x| x.0).sum();
        }

        let mut result = Joltage(0);
        let mut start = 0;
        let mut remaining = Self::LEN;

        while remaining > 0 {
            let search_end = self.0.len() - remaining + 1;

            let mut max_joltage = self.0[start];
            let mut max_pos = start;

            for i in start..search_end {
                if self.0[i] > max_joltage {
                    max_joltage = self.0[i];
                    max_pos = i;
                }
            }
            result = result + max_joltage;
            start = max_pos + 1;
            remaining -= 1;
        }
        result.into()
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
        let result = 3_121_910_778_619;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
