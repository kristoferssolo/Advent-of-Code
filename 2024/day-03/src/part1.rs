use std::num::ParseIntError;

use miette::Result;
use regex::Regex;

#[derive(Debug)]
struct Multiplication(usize, usize);

impl Multiplication {
    fn calculate(&self) -> usize {
        self.0 * self.1
    }
}

impl<T, U> TryFrom<(T, U)> for Multiplication
where
    T: Into<String>,
    U: Into<String>,
{
    type Error = ParseIntError;
    fn try_from(value: (T, U)) -> std::result::Result<Self, Self::Error> {
        Ok(Self(
            value.0.into().parse::<usize>()?,
            value.1.into().parse::<usize>()?,
        ))
    }
}

fn extract_multiplications<'a>(
    line: &'a str,
    re: &'a Regex,
) -> impl Iterator<Item = Multiplication> + 'a {
    re.captures_iter(line).filter_map(|caps| {
        let first = caps.get(1)?.as_str().to_owned();
        let second = caps.get(2)?.as_str().to_owned();
        Multiplication::try_from((first, second)).ok()
    })
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = input
        .lines()
        .flat_map(|line| extract_multiplications(line, &re))
        .map(|mult| mult.calculate())
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = 161;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
