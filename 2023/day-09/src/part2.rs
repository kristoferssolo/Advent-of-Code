use color_eyre::Result;

use itertools::{Itertools, Position};

fn process_line(line: &str) -> i64 {
    let mut nums = line
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut start_nums = Vec::new();

    while !nums.iter().all(|num| num == &0) {
        nums = nums
            .iter()
            .tuple_windows()
            .with_position()
            .map(|(pos, (left, right))| {
                if let Position::First | Position::Only = pos {
                    start_nums.push(*left)
                }
                right - left
            })
            .collect()
    }
    dbg!(&start_nums);
    let result = start_nums.iter().rev().fold(0, |acc, num| num - acc);
    result
}

pub fn process(input: &str) -> Result<i64> {
    let total = input.lines().map(process_line).sum();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
