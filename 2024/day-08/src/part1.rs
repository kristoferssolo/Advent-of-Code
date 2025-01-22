use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2 {
    row: usize,
    col: usize,
}

impl Vec2 {
    const fn abs_diff(self, other: Self) -> Self {
        Self {
            row: self.row.abs_diff(other.row),
            col: self.col.abs_diff(other.col),
        }
    }

    const fn manhattan_distance(self, other: Self) -> usize {
        let diff = self.abs_diff(other);
        diff.row + diff.col
    }
}

impl From<Vec2> for (usize, usize) {
    fn from(value: Vec2) -> Self {
        (value.row, value.col)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            row: self.row.saturating_sub(rhs.row),
            col: self.col.saturating_sub(rhs.col),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Location {
    Antenna(char),
    Antinode,
    Empty,
}

impl From<char> for Location {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Antinode,
            ch => Self::Antenna(ch),
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Antenna(ch) => *ch,
            Self::Antinode => '#',
            Self::Empty => '.',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Error, Diagnostic)]
enum RoofError {
    #[error("Failed to parse roof")]
    ParseError,
}

fn calculate_diffs(positions: &[Vec2]) -> Vec<Vec2> {
    positions
        .iter()
        .enumerate()
        .flat_map(|(idx, &pos1)| {
            positions[idx + 1..]
                .iter()
                .map(move |&pos2| pos1.abs_diff(pos2))
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Roof(Vec<Vec<Location>>);

impl Roof {
    fn place_antinodes(&mut self) {
        let antenna_positions = self.antenna_positions();
        for (_, pos) in antenna_positions.iter() {
            for i in 0..pos.len() {
                for j in (i + 1)..pos.len() {
                    let (pos1, pos2) = (pos[i], pos[j]);

                    if let Some(antinode_pos) = self.find_antinode_position(pos1, pos2) {
                        self.0[antinode_pos.row][antinode_pos.col] = Location::Antinode;
                    }
                }
            }
        }
    }

    fn find_antinode_position(&self, pos1: Vec2, pos2: Vec2) -> Option<Vec2> {
        let diff = pos1.abs_diff(pos2);
        let total_dist = diff.row + diff.col;

        // Try each step between the antennas
        for step in 0..=total_dist {
            let row = if pos1.row < pos2.row {
                pos1.row + step.min(diff.row)
            } else {
                pos1.row - step.min(diff.row)
            };

            let col = if pos1.col < pos2.col {
                pos1.col + step.min(diff.col)
            } else {
                pos1.col - step.min(diff.col)
            };

            let candidate = Vec2 { row, col };

            // Check if position is valid and equidistant
            if row < self.0.len()
                && col < self.0[0].len()
                && matches!(self.0[row][col], Location::Empty)
            {
                let dist1 = pos1.manhattan_distance(candidate);
                let dist2 = pos2.manhattan_distance(candidate);
                if dist1 == dist2 {
                    return Some(candidate);
                }
            }
        }
        None
    }

    fn antenna_positions(&self) -> HashMap<char, Vec<Vec2>> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(col_idx, col)| match col {
                        Location::Antenna(ch) => Some((
                            *ch,
                            Vec2 {
                                row: row_idx,
                                col: col_idx,
                            },
                        )),
                        _ => None,
                    })
            })
            .fold(HashMap::new(), |mut acc, (key, pos)| {
                acc.entry(key).or_default().push(pos);
                acc
            })
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|pos| matches!(pos, Location::Antinode))
                    .count()
            })
            .sum()
    }

    fn row_len(&self) -> usize {
        self.0.len()
    }

    fn col_len(&self) -> usize {
        self.0[0].len()
    }

    fn is_within_bounds(&self, pos: &Vec2) -> bool {
        pos.row < self.row_len() && pos.col < self.col_len()
    }
}

impl FromStr for Roof {
    type Err = RoofError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let roof = s
            .lines()
            .map(|line| line.chars().map(Location::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self(roof))
    }
}

impl Display for Roof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.0.iter().enumerate() {
            for pos in row {
                write!(f, "{}", pos)?;
            }
            if idx < self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let mut roof = Roof::from_str(input)?;
    roof.place_antinodes();
    println!("{roof}");
    Ok(roof.count())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = 14;
        assert_eq!(process(input)?, result);
        Ok(())
    }

    #[rstest]
    #[case(
        "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
        "..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
.........."
    )]
    /* #[case(
            "..........
    ..........
    ..........
    ....a.....
    ........a.
    .....a....
    ..........
    ..........
    ..........
    ..........",
            "..........
    ...#......
    #.........
    ....a.....
    ........a.
    .....a....
    ..#.......
    ......#...
    ..........
    .........."
        )]
        #[case(
            "............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............",
            "......#....#
    ...#....0...
    ....#0....#.
    ..#....0....
    ....0....#..
    .#....A.....
    ...#........
    #......#....
    ........A...
    .........A..
    ..........#.
    ..........#."
        )] */
    fn test_layout(#[case] input_str: &str, #[case] output_str: &str) -> Result<()> {
        let mut input = Roof::from_str(input_str)?;
        input.place_antinodes();
        let output = Roof::from_str(output_str)?;
        assert_eq!(input, output);
        Ok(())
    }
}
