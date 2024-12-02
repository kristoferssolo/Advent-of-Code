use miette::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    Safe,
    Unsafe,
}

impl From<&Vec<usize>> for Level {
    fn from(value: &Vec<usize>) -> Self {
        if (value.iter().is_sorted() || value.iter().rev().is_sorted())
            && value.windows(2).all(|pairs| {
                let diff = pairs[0].abs_diff(pairs[1]);
                (1..=3).contains(&diff)
            })
        {
            return Self::Safe;
        }
        Self::Unsafe
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let sum = input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Level::from(&levels)
        })
        .filter(|&report| report == Level::Safe)
        .count();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let result = 2;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
