use miette::{Diagnostic, Result};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MultiplicationError {
    #[error("Failed to parse first value: {0}")]
    #[diagnostic(code(calculator::parse_error::first_value))]
    FirstValueParseError(String),

    #[error("Failed to parse second value: {0}")]
    #[diagnostic(code(calculator::parse_error::second_value))]
    SecondValueParseError(String),

    #[error("Invalid multiplication format")]
    #[diagnostic(code(calculator::parse_error::format))]
    InvalidFormat,
}

#[derive(Debug)]
struct Multiplication(usize, usize);

impl Multiplication {
    fn calculate(&self) -> usize {
        self.0 * self.1
    }
}

impl<T, U> TryFrom<(T, U)> for Multiplication
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    type Error = MultiplicationError;
    fn try_from(value: (T, U)) -> std::result::Result<Self, Self::Error> {
        let first =
            value.0.as_ref().parse().map_err(|_| {
                MultiplicationError::FirstValueParseError(value.0.as_ref().to_string())
            });
        let second =
            value.1.as_ref().parse().map_err(|_| {
                MultiplicationError::SecondValueParseError(value.1.as_ref().to_string())
            });
        Ok(Self(first?, second?))
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum CalculatorError {
    #[error("Failed to compile regex: {0}")]
    #[diagnostic(code(calculator::regex_error))]
    RegexError(#[from] regex::Error),

    #[diagnostic(code(calculator::multiplication_error))]
    #[error("Failed to parse multiplication: {0}")]
    MultiplicationError(#[from] MultiplicationError),

    #[error("Failed to process input: {0}")]
    #[diagnostic(code(calculator::process_error))]
    ProcessError(String),
}

#[derive(Debug)]
struct Calculator {
    do_re: Regex,
    mult_re: Regex,
}

impl Calculator {
    fn new() -> Result<Self, CalculatorError> {
        Ok(Self {
            do_re: Regex::new(r"^.*?don't\(\)|do\(\)(.*?)don't\(\)|do\(\).*$")?,
            mult_re: Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?,
        })
    }

    fn extract_multiplications<'a>(
        &'a self,
        line: &'a str,
    ) -> impl Iterator<Item = Multiplication> + 'a {
        self.mult_re.captures_iter(line).filter_map(|caps| {
            Multiplication::try_from((caps.get(1)?.as_str(), caps.get(2)?.as_str())).ok()
        })
    }

    fn extract_do<'a>(&'a self, line: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.do_re
            .captures_iter(line)
            .filter_map(|caps| caps.get(0).map(|m| m.as_str()))
    }

    fn process(&self, input: &str) -> usize {
        let value = input.lines().collect::<String>();
        self.extract_do(&value)
            .flat_map(|line| self.extract_multiplications(line))
            .map(|mult| mult.calculate())
            .sum()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let calculator = Calculator::new()?;
    Ok(calculator.process(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = 48;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
