use std::collections::{BTreeMap, HashSet};

use color_eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

pub fn process(input: &str) -> Result<u32> {
    let map: BTreeMap<(i32, i32), Value> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                (
                    (y as i32, x as i32),
                    match ch {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("Should be a number"))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = Vec::new();

    for ((y, x), value) in &map {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(val)
                    if val
                        .last()
                        .map(|((last_num_x, _), _)| last_num_x + 1 == *x)
                        .unwrap_or(false) =>
                {
                    numbers.last_mut().unwrap().push(((*x, *y), *num))
                }
                _ => numbers.push(vec![((*x, *y), *num)]),
            }
        }
    }

    let mut sum = 0;
    for symbol in map
        .iter()
        .filter(|(_, value)| matches!(value, Value::Symbol('*')))
    {
        let positions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let pos_to_check: HashSet<_> = positions
            .iter()
            .map(|outer_pos| (outer_pos.0 + symbol.0 .1, outer_pos.1 + symbol.0 .0))
            .collect();

        let indexes_of_numbers: HashSet<_> = pos_to_check
            .iter()
            .flat_map(|pos| {
                numbers.iter().enumerate().filter_map(move |(i, num_list)| {
                    num_list
                        .iter()
                        .find(|(num_pos, _)| num_pos == pos)
                        .map(|_| i)
                })
            })
            .collect();

        let is_gear = indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            sum += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap_or_default()
                })
                .product::<u32>();
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
