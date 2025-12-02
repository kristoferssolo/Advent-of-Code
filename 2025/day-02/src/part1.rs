use miette::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Id(usize);

impl FromStr for Id {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let val = s.parse().unwrap();
        Ok(Self(val))
    }
}

#[derive(Debug)]
struct Range {
    start: Id,
    end: Id,
}

impl Range {
    fn find_invalid(&self) -> Vec<Id> {
        (self.start.0..=self.end.0)
            .filter_map(|x| {
                if has_repeating_sequence(x) {
                    Some(Id(x))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn has_repeating_sequence(num: usize) -> bool {
    let s = num.to_string();
    let len = s.len();
    len.is_multiple_of(2) && s[0..len / 2].repeat(2) == s
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut trimmed = s.trim().split('-');
        let start = trimmed.next().unwrap().parse().unwrap();
        let end = trimmed.next_back().unwrap().parse().unwrap();
        Ok(Self { start, end })
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> Result<usize> {
    let result = input
        .trim()
        .split(',')
        .map(Range::from_str)
        .flat_map(|range| range.unwrap().find_invalid())
        .map(|x| x.0)
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = 1_227_775_554;
        assert_eq!(process(input)?, result);
        Ok(())
    }

    #[rstest]
    #[case(11)]
    #[case(22)]
    #[case(1010)]
    #[case(1_188_511_885)]
    #[case(222_222)]
    #[case(446_446)]
    #[case(38_593_859)]
    fn repeating(#[case] num: usize) {
        assert!(has_repeating_sequence(num));
    }

    #[rstest]
    #[case(111)]
    #[case(222)]
    #[case(222_222_222)]
    fn not_repeating(#[case] num: usize) {
        assert!(!has_repeating_sequence(num));
    }
}
