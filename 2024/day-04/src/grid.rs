use crate::vec::Vec2;
use miette::Diagnostic;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub(crate) enum GridError {}

#[derive(Debug)]
pub(crate) struct Grid(pub(crate) Vec<Vec<char>>);

impl Grid {
    pub(crate) fn is_valid_position(&self, pos: &Vec2) -> bool {
        pos.x >= 0 && pos.x < self.cols() as i32 && pos.y >= 0 && pos.y < self.rows() as i32
    }

    pub(crate) fn rows(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn cols(&self) -> usize {
        self.0[0].len()
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

        Ok(Grid(data))
    }
}
