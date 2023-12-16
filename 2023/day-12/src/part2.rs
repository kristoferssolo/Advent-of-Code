use color_eyre::Result;
use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Puzzle {
    spaces_to_fill: usize,
    line: String,
    batches: Vec<usize>,
}

impl Puzzle {
    fn generate_permutations(&self) -> impl Iterator<Item = String> {
        repeat_n([".", "#"].into_iter(), self.spaces_to_fill)
            .multi_cartesian_product()
            .map(|x| x.join(""))
    }

    fn check_option(&self, option: &str) -> bool {
        let mut option_iter = option.chars();
        let filled_option = self
            .line
            .chars()
            .map(|ch| match ch {
                '?' => option_iter
                    .next()
                    .expect("Should have a len similar to needed gaps"),
                value => value,
            })
            .collect::<String>();
        let counts = filled_option
            .chars()
            .group_by(|ch| ch == &'#')
            .into_iter()
            .filter_map(|(is_hashes, group)| is_hashes.then_some(group.into_iter().count()))
            .collect_vec();
        &self.batches[..] == &counts[..]
    }

    fn possible_solution_count(&self) -> usize {
        let count = self
            .generate_permutations()
            .filter(|option| self.check_option(option))
            .count();
        count
    }
}

fn parse_line(input: &str) -> IResult<&str, Puzzle> {
    let (input, (line, batches)) = separated_pair(
        is_a("?.#"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;
    let expanded_line = std::iter::repeat(line).take(5).join("?");
    let spaces_to_fill = expanded_line.chars().filter(|ch| ch != &'?').count();
    Ok((
        input,
        Puzzle {
            spaces_to_fill,
            line: expanded_line,
            batches: std::iter::repeat(batches)
                .take(5)
                .flatten()
                .map(|x| x as usize)
                .collect(),
        },
    ))
}

pub fn process(input: &str) -> Result<usize> {
    let puzzles = input
        .lines()
        .map(parse_line)
        .collect::<std::result::Result<Vec<(&str, Puzzle)>, nom::Err<nom::error::Error<&str>>>>()
        .expect("Parsisng to succeed");
    let sum = puzzles
        .iter()
        .map(|(_, puzzle)| puzzle.possible_solution_count());
    Ok(sum.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";
        assert_eq!(525152, process(input)?);
        Ok(())
    }
}
