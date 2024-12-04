use miette::Result;
use std::str::FromStr;

use crate::{grid::Grid, vec::Vec2};

enum Direction {
    Right,     // (1, 0)
    Left,      // (-1, 0)
    Up,        // (0, -1)
    Down,      // (0, 1)
    UpRight,   // (1, -1)
    UpLeft,    // (-1, -1)
    DownRight, // (1, 1)
    DownLeft,  // (-1, 1)
}

impl Direction {
    const fn get_vector(&self) -> Vec2 {
        match self {
            Direction::Right => Vec2::new(1, 0),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
            Direction::UpRight => Vec2::new(1, -1),
            Direction::UpLeft => Vec2::new(-1, -1),
            Direction::DownRight => Vec2::new(1, 1),
            Direction::DownLeft => Vec2::new(-1, 1),
        }
    }

    const fn all_directions() -> [Direction; 8] {
        [
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }
}

trait Part1 {
    fn check_direction(&self, pos: Vec2, direction: &Direction, word: &str) -> bool;
    fn count_word(&self, word: &str) -> usize;
}

impl Part1 for Grid {
    fn check_direction(&self, pos: Vec2, direction: &Direction, word: &str) -> bool {
        let dir_vec = direction.get_vector();
        word.chars().enumerate().all(|(idx, char)| {
            let new_pos = pos + dir_vec.scale(idx as i32);
            self.is_valid_position(&new_pos)
                && self.0[new_pos.y as usize][new_pos.x as usize] == char
        })
    }

    fn count_word(&self, word: &str) -> usize {
        (0..self.rows())
            .flat_map(|row| {
                (0..self.cols()).flat_map(move |col| {
                    Direction::all_directions()
                        .into_iter()
                        .filter(move |direction| {
                            let pos = Vec2::new(col as i32, row as i32);
                            self.check_direction(pos, direction, word)
                        })
                })
            })
            .count()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    Ok(grid.count_word("XMAS"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = 18;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
