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
            match numbers.last() {
                Some(val)
                    if val
                        .last()
                        .map(|((last_num_x, _), _)| last_num_x + 1 == *x)
                        .unwrap_or(false) =>
                {
                    numbers.last_mut().unwrap().push(((*x, *y), *num));
                }
                _ => numbers.push(vec![((*x, *y), *num)]),
            }
        }
    }
    let mut sum = 0;

    for num_list in numbers {
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
        let num_positions: HashSet<_> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();
        let pos_to_check: HashSet<_> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions
                    .iter()
                    .map(|outer_pos| (outer_pos.0 + pos.1, outer_pos.1 + pos.0))
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        let is_part_number = pos_to_check.iter().any(|pos| {
            map.get(pos)
                .map_or(false, |value| matches!(value, Value::Symbol(_)))
        });

        if is_part_number {
            sum += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()?;
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
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
