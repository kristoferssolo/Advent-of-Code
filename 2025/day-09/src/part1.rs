use glam::{USizeVec2, usize};
use miette::miette;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Tile(USizeVec2);

impl Tile {
    const fn x(&self) -> usize {
        self.0.x
    }

    const fn y(&self) -> usize {
        self.0.y
    }

    const fn area(&self, other: Self) -> usize {
        (self.x().abs_diff(other.x()) + 1) * (self.y().abs_diff(other.y()) + 1)
    }
}

impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s.trim().split(',').collect::<Vec<_>>();
        if str.len() < 2 {
            return Err("Missing coords".to_string());
        }
        Ok(Self(USizeVec2 {
            x: str_to_usize(str[0])?,
            y: str_to_usize(str[1])?,
        }))
    }
}

fn str_to_usize(s: &str) -> Result<usize, String> {
    s.parse::<usize>().map_err(|e| e.to_string())
}

#[derive(Debug, Clone)]
struct Grid(Vec<Tile>);

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .trim()
            .lines()
            .map(Tile::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(grid))
    }
}

impl Grid {
    fn get_max_area(&self) -> usize {
        self.area_list().max().unwrap_or(0)
    }

    fn area_list(&self) -> impl Iterator<Item = usize> {
        self.0
            .iter()
            .flat_map(|&a| self.0.iter().map(move |&b| a.area(b)))
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let grid = Grid::from_str(input).map_err(|e| miette!("{e}"))?;
    Ok(grid.get_max_area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let result = 50;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
