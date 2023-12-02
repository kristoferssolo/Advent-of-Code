use color_eyre::Result;

pub fn process(input: &str) -> Result<u32> {
    let sum: u32 = input
        .lines()
        .flat_map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();
            match (digits.first(), digits.last()) {
                (Some(first), Some(last)) => Some(first * 10 + last),
                _ => None,
            }
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
