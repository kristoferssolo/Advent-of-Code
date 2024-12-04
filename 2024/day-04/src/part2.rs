use itertools::Itertools;
use miette::{Diagnostic, Result};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UpRight,   // (1, -1)
    UpLeft,    // (-1, -1)
    DownRight, // (1, 1)
    DownLeft,  // (-1, 1)
}

impl Direction {
    const fn get_vector(&self) -> (i32, i32) {
        match self {
            Direction::UpRight => (1, -1),
            Direction::UpLeft => (-1, -1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (-1, 1),
        }
    }

    fn pairs() -> impl Iterator<Item = (Self, Self)> {
        [Self::UpRight, Self::UpLeft, Self::DownRight, Self::DownLeft]
            .into_iter()
            .combinations(2)
            .map(|pair| (pair[0], pair[1]))
    }
}

#[derive(Debug, Error, Diagnostic)]
enum GridError {
    #[error("Error parsing")]
    ParseError,
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn check_direction(&self, row: i32, col: i32, direction: &Direction, word: &str) -> bool {
        let (dx, dy) = direction.get_vector();
        let start_row = row - dy;
        let start_col = col - dx;
        word.chars().enumerate().all(|(idx, char)| {
            let new_row = start_row + dy * idx as i32;
            let new_col = start_col + dx * idx as i32;
            self.is_valid_position(new_row, new_col)
                && self.data[new_row as usize][new_col as usize] == char
        })
    }

    fn is_valid_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32
    }

    fn count_word(&self, word: &str) -> usize {
        (0..self.rows)
            .flat_map(|row| {
                (0..self.cols).filter(move |&col| {
                    if self.data[row][col] != 'A' {
                        return false;
                    }
                    Direction::pairs().any(|(dir1, dir2)| {
                        self.check_direction(row as i32, col as i32, &dir1, word)
                            && self.check_direction(row as i32, col as i32, &dir2, word)
                    })
                })
            })
            .count()
    }
}

impl FromStr for Grid {
    type Err = GridError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let rows = data.len();
        if rows == 0 {
            return Err(GridError::ParseError);
        }
        let cols = data[0].len();

        if data.iter().any(|row| row.len() != cols) {
            return Err(GridError::ParseError);
        }

        Ok(Grid { data, rows, cols })
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
