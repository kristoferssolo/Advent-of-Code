use miette::{Diagnostic, Result};
use std::{fmt::Display, ops::Add, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    row: i32,
    col: i32,
}

impl From<Vec2> for (usize, usize) {
    fn from(value: Vec2) -> Self {
        (value.row as usize, value.col as usize)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(value: (usize, usize)) -> Self {
        Vec2 {
            row: value.0 as i32,
            col: value.1 as i32,
        }
    }
}

impl Add<Direction> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Vec2 {
                row: self.row.saturating_sub(1),
                col: self.col,
            },
            Direction::Right => Vec2 {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Down => Vec2 {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Vec2 {
                row: self.row,
                col: self.col.saturating_sub(1),
            },
        }
    }
}

impl Add<Vec2> for Direction {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
}

#[derive(Debug, Error, Diagnostic)]
enum PositionError {
    #[error("Failed to parse data")]
    ParseError,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Iterator for Direction {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        Some(*self)
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Position {
    Guard(Direction),
    Obsticle,
    #[default]
    Unvisited,
    Visited,
}

impl TryFrom<char> for Position {
    type Error = PositionError;
    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '^' => Ok(Position::Guard(Direction::Up)),
            '>' => Ok(Position::Guard(Direction::Right)),
            'v' => Ok(Position::Guard(Direction::Down)),
            '<' => Ok(Position::Guard(Direction::Left)),
            '#' => Ok(Position::Obsticle),
            '.' => Ok(Position::Unvisited),
            'X' => Ok(Position::Visited),
            _ => Err(PositionError::ParseError),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Position::Guard(direction) => match direction {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            },
            Position::Obsticle => '#',
            Position::Unvisited => '.',
            Position::Visited => 'X',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Error, Diagnostic)]
enum LabError {
    #[error("Failed to parse data")]
    ParseError,
    #[error("No guard was found")]
    NoGuardFound,
}

impl From<PositionError> for LabError {
    fn from(value: PositionError) -> Self {
        match value {
            PositionError::ParseError => LabError::ParseError,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    pos: Vec2,
    direction: Direction,
}

impl Guard {
    fn next_pos(&self) -> Vec2 {
        self.pos + self.direction
    }

    fn rotate(&mut self) {
        self.direction = self.direction.next().unwrap();
    }

    fn move_(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
}

#[derive(Debug, Clone)]
struct Lab {
    grid: Vec<Vec<Position>>,
    guard: Guard,
}

impl Lab {
    fn visit(&mut self, pos: Vec2) {
        let (row, col) = pos.into();
        if let Position::Unvisited = self.grid[row][col] {
            self.grid[row][col] = Position::Visited;
        }
    }

    fn walk(&mut self) {
        while let Some(next_pos) = self.get_next_move() {
            self.execute_move(next_pos);
        }
        self.visit(self.guard.pos);
    }

    fn execute_move(&mut self, next_pos: Vec2) {
        self.visit(self.guard.pos);

        let (row, col) = next_pos.into();
        match self.grid[row][col] {
            Position::Obsticle => self.guard.rotate(),
            _ => self.guard.move_(next_pos),
        };
    }

    fn get_next_move(&self) -> Option<Vec2> {
        let next_pos = self.guard.next_pos();
        if !self.is_within_grid(next_pos) {
            return None;
        }

        Some(next_pos)
    }

    fn is_within_grid(&self, pos: Vec2) -> bool {
        pos.row >= 0
            && pos.col >= 0
            && pos.row < self.grid.len() as i32
            && pos.col < self.grid[0].len() as i32
    }
}

impl FromStr for Lab {
    type Err = LabError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut guard = None;
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let position = Position::try_from(ch).map_err(LabError::from)?;
                        if let Position::Guard(dir) = position {
                            guard = Some(Guard {
                                pos: (row, col).into(),
                                direction: dir,
                            });
                            return Ok(Position::Visited);
                        }
                        Ok(position)
                    })
                    .collect::<Result<Vec<_>, LabError>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let guard = guard.ok_or(LabError::NoGuardFound)?;

        Ok(Lab { grid, guard })
    }
}

impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.grid.iter().enumerate() {
            for position in row {
                write!(f, "{}", position)?;
            }
            if idx < self.grid.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let mut lab = Lab::from_str(input)?;
    lab.walk();
    let result = lab
        .grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&&pos| matches!(pos, Position::Visited))
                .count()
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = 41;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
