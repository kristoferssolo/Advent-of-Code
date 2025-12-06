use miette::miette;
use std::{fmt::Display, str::FromStr, vec};

#[derive(Debug, Clone)]
struct Row<T>(Vec<T>);

impl<T> FromStr for Row<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s
            .split_whitespace()
            .map(str::parse::<T>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Self(row))
    }
}

#[derive(Debug, Clone)]
struct Grid<T>(Vec<T>);

impl<T: Clone> Grid<Row<T>> {
    fn to_transposed(&self) -> Self {
        if self.0.is_empty() {
            return Self(Vec::new());
        }

        let num_cols = self.0[0].0.len();
        let mut transposed = vec![Vec::new(); num_cols];

        for row in &self.0 {
            for (col_idx, val) in row.0.iter().enumerate() {
                if col_idx < transposed.len() {
                    transposed[col_idx].push(val.clone());
                }
            }
        }

        Self(transposed.into_iter().map(Row).collect())
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(T::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Self(rows))
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let (rest, last_line) = match input.trim_end().rsplit_once('\n') {
        Some((r, l)) => (r, l),
        None => ("", ""),
    };
    let digits = rest
        .parse::<Grid<Row<usize>>>()
        .map_err(|e| miette!("{e}"))?;
    let operators = last_line.parse::<Row<char>>().map_err(|e| miette!("{e}"))?;
    let result = operators
        .0
        .iter()
        .zip(digits.to_transposed().0)
        .map(|(operator, row)| match operator {
            '+' => row.0.iter().sum(),
            '*' => row.0.iter().product::<usize>(),
            op => panic!("Unknown operator: {op}"),
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let result = 4_277_556;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
