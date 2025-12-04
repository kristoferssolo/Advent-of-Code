use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Roll,
    Empty,
}

impl From<char> for Space {
    fn from(ch: char) -> Self {
        match ch {
            '@' => Self::Roll,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone)]
struct Row(Vec<Space>);

impl FromStr for Row {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s.chars().map(Space::from).collect();
        Ok(Self(row))
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Row>);

impl FromStr for Grid {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(Row::from_str)
            .collect::<Result<Vec<_>, Infallible>>()
            .unwrap();
        Ok(Self(grid))
    }
}

#[derive(Debug)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    const fn offset(&self, coords: &Self) -> Self {
        Self {
            x: self.x + coords.x,
            y: self.y + coords.y,
        }
    }
}

impl Grid {
    fn find_accessible(&mut self) -> usize {
        let mut total_removed = 0;
        loop {
            let accessible = self
                .0
                .iter()
                .enumerate()
                .flat_map(|(x, row)| row.0.iter().enumerate().map(move |(y, _)| (x, y)))
                .filter(|(x, y)| {
                    let coords = Coords::new((*x).try_into().unwrap(), (*y).try_into().unwrap());
                    let adjacent = self.get_adjacent(&coords);
                    adjacent < 4 && matches!(self.get(&coords), Space::Roll)
                })
                .collect::<Vec<_>>();
            if accessible.is_empty() {
                break;
            }
            for (x, y) in &accessible {
                self.0[*x].0[*y] = Space::Empty;
            }
            total_removed += accessible.len();
        }
        total_removed
    }

    fn get_adjacent(&self, coords: &Coords) -> usize {
        let max_x = self.0.len() - 1;
        let max_y = self.0[0].0.len() - 1;

        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .filter(|(x, y)| {
                !(*x == 0 && *y == 0)
                    && !((coords.x == 0 && *x == -1)
                        || (coords.y == 0 && *y == -1)
                        || (usize::try_from(coords.x).unwrap() == max_x && *x == 1)
                        || (usize::try_from(coords.y).unwrap() == max_y && *y == 1))
            })
            .filter(|(x, y)| {
                let coord = coords.offset(&Coords::new(
                    (*x).try_into().unwrap(),
                    (*y).try_into().unwrap(),
                ));

                matches!(self.get(&coord), Space::Roll)
            })
            .count()
    }

    fn get(&self, coords: &Coords) -> Space {
        self.0[usize::try_from(coords.x).unwrap()].0[usize::try_from(coords.y).unwrap()]
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut grid = Grid::from_str(input)?;
    Ok(grid.find_accessible())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        let result = 43;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
