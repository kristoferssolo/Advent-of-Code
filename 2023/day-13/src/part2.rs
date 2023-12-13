use std::iter::from_fn;

use color_eyre::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn detect_fold(input: &str) -> Option<Fold> {
    detect_vertical_fold(input).or(detect_horizontal_fold(input))
}

fn detect_vertical_fold(input: &str) -> Option<Fold> {
    let mut columns_iter_collection = input.lines().map(|line| line.chars()).collect::<Vec<_>>();
    let columns = from_fn(move || {
        let mut items = Vec::new();
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<_>>();
    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
                || line_a
                    .iter()
                    .zip(line_b.iter())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 1
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a = (&columns[0..=index_a]).iter().rev();
            let lines_b = (&columns[index_b..]).iter();
            (lines_a
                .flatten()
                .zip(lines_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(index_a + 1)
        });
    result.map(|num| Fold::Vertical(num))
}

fn detect_horizontal_fold(input: &str) -> Option<Fold> {
    let lines = input.lines().collect::<Vec<_>>();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
                || line_a
                    .chars()
                    .zip(line_b.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 1
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a = (&lines[0..=index_a]).iter().map(|line| line.chars()).rev();
            let lines_b = (&lines[index_b..]).iter().map(|line| line.chars());
            (lines_a
                .flatten()
                .zip(lines_b.flatten())
                .filter(|(a, b)| a != b)
                .count()
                == 1)
                .then_some(index_a + 1)
        });
    result.map(|num| Fold::Horizontal(num))
}

pub fn process(input: &str) -> Result<usize> {
    let (horizontal, vertical) =
        input
            .split("\n\n")
            .flat_map(detect_fold)
            .fold((0, 0), |mut acc, item| match item {
                Fold::Horizontal(num) => {
                    acc.0 += 100 * num;
                    acc
                }
                Fold::Vertical(num) => {
                    acc.1 += num;
                    acc
                }
            });
    Ok(horizontal + vertical)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(400, process(input)?);
        Ok(())
    }

    #[test]
    fn test_horizontal() -> Result<()> {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(Some(Fold::Horizontal(1)), detect_fold(input));

        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(Some(Fold::Horizontal(3)), detect_fold(input));
        Ok(())
    }
}
