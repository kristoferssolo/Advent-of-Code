use std::{collections::HashMap, fmt::Display};

use color_eyre::Result;
use glam::IVec2;

#[derive(Debug, Clone, Copy)]
enum Rock {
    Movable,
    Immovable,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rock::Movable => "0",
                Rock::Immovable => "#",
            }
        )
    }
}

fn print_grid(map: &HashMap<IVec2, Rock>, boundaries: &IVec2) {
    for y in 0..boundaries.y {
        for x in 0..boundaries.x {
            match map.get(&IVec2::new(x, y)) {
                Some(rock) => print!("{rock}"),
                None => print!("."),
            }
        }
        println!("");
    }
}

fn rock_shift_north(
    rock_map: &HashMap<IVec2, Rock>,
    boundaries: &IVec2,
    static_rocks: &HashMap<IVec2, Rock>,
) -> HashMap<IVec2, Rock> {
    let mut results = static_rocks.clone();
    let mut next_potentially_available_pos = IVec2::new(0, 0);

    for x in 0..boundaries.x {
        next_potentially_available_pos = IVec2::new(x, 0);
        for y in 0..boundaries.y {
            match rock_map.get(&IVec2::new(x, y)) {
                Some(Rock::Immovable) => next_potentially_available_pos = IVec2::new(x, y + 1),
                Some(Rock::Movable) => {
                    let next_pos = next_potentially_available_pos;
                    results.insert(next_pos, Rock::Movable);
                    next_potentially_available_pos.y += 1;
                }
                None => (),
            }
        }
    }
    results
}

pub fn process(input: &str) -> Result<usize> {
    let columns = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let boundaries = IVec2::new(columns as i32, rows as i32);
    let rock_map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '0' => Some((IVec2::new(x as i32, y as i32), Rock::Movable)),
                    '#' => Some((IVec2::new(x as i32, y as i32), Rock::Immovable)),
                    _ => None,
                })
        })
        .collect::<HashMap<IVec2, Rock>>();

    let static_rocks = rock_map
        .iter()
        .filter_map(|(pos, rock)| match rock {
            Rock::Movable => None,
            Rock::Immovable => Some((*pos, *rock)),
        })
        .collect::<HashMap<IVec2, Rock>>();
    let next_state = rock_shift_north(&rock_map, &boundaries, &static_rocks);
    let sum = next_state
        .iter()
        .filter_map(|(pos, rock)| match rock {
            Rock::Movable => Some(boundaries.y - pos.y),
            Rock::Immovable => None,
        })
        .sum::<i32>();
    Ok(sum as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(136, process(input)?);
        Ok(())
    }
}
