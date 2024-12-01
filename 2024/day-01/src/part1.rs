use miette::Result;

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let (mut firsts, mut lasts): (Vec<_>, Vec<_>) = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (*nums.first().unwrap(), *nums.last().unwrap())
        })
        .unzip();

    firsts.sort();
    lasts.sort();

    let sum = firsts
        .iter()
        .zip(lasts.iter())
        .map(|(first, last)| first.abs_diff(*last))
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = 11;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
