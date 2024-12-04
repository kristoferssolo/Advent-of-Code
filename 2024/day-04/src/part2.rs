use crate::{grid::Grid, vec::Vec2};
use itertools::Itertools;
use miette::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UpRight,   // (1, -1)
    UpLeft,    // (-1, -1)
    DownRight, // (1, 1)
    DownLeft,  // (-1, 1)
}

impl Direction {
    const fn get_vector(&self) -> Vec2 {
        match self {
            Direction::UpRight => Vec2::new(1, -1),
            Direction::UpLeft => Vec2::new(-1, -1),
            Direction::DownRight => Vec2::new(1, 1),
            Direction::DownLeft => Vec2::new(-1, 1),
        }
    }

    fn pairs() -> impl Iterator<Item = (Self, Self)> {
        [Self::UpRight, Self::UpLeft, Self::DownRight, Self::DownLeft]
            .into_iter()
            .combinations(2)
            .map(|pair| (pair[0], pair[1]))
    }
}

trait Part2 {
    fn check_direction(&self, pos: Vec2, direction: &Direction, word: &str) -> bool;
    fn count_word(&self, word: &str) -> usize;
}

impl Part2 for Grid {
    fn check_direction(&self, pos: Vec2, direction: &Direction, word: &str) -> bool {
        let dir_vec = direction.get_vector();
        let start_pos = pos + dir_vec.scale(-1); // Move back one position
        word.chars().enumerate().all(|(idx, char)| {
            let new_pos = start_pos + dir_vec.scale(idx as i32);
            self.is_valid_position(&new_pos)
                && self.0[new_pos.y as usize][new_pos.x as usize] == char
        })
    }

    fn count_word(&self, word: &str) -> usize {
        (0..self.rows())
            .flat_map(|row| {
                (0..self.cols()).filter(move |&col| {
                    let pos = Vec2::new(col as i32, row as i32);
                    if self.0[row][col] != 'A' {
                        return false;
                    }
                    Direction::pairs().any(|(dir1, dir2)| {
                        self.check_direction(pos, &dir1, word)
                            && self.check_direction(pos, &dir2, word)
                    })
                })
            })
            .count()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    Ok(grid.count_word("MAS"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = 9;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
