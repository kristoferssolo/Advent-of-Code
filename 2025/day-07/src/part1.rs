use std::collections::HashSet;
use tracing::info;

type Column = usize;
type Row = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Splitter {
    column: Column,
    row: Row,
}

#[derive(Debug)]
struct SimulationState {
    active_positions: HashSet<Column>,
    splitters: HashSet<Splitter>,
}

impl SimulationState {
    fn new(start_column: Column) -> Self {
        let mut active_positions = HashSet::new();
        active_positions.insert(start_column);
        Self {
            active_positions,
            splitters: HashSet::new(),
        }
    }

    fn process_row(&mut self, row: Row, line: &str) {
        let mut next_positions = HashSet::new();

        for col in &self.active_positions {
            if line.as_bytes()[*col] == b'^' {
                info!(?col, "split at");
                next_positions.insert(col - 1);
                next_positions.insert(col + 1);
                next_positions.insert(col + 1);
                self.splitters.insert(Splitter { column: *col, row });
            } else {
                next_positions.insert(*col);
            }
        }

        self.active_positions = next_positions;
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut lines = input.lines().enumerate();
    let (_, first_line) = lines.next().unwrap();
    let start_column = first_line.chars().position(|ch| ch == 'S').unwrap();

    let mut state = SimulationState::new(start_column);

    for (row, line) in lines {
        state.process_row(row, line);
    }

    Ok(state.splitters.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        let result = 21;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
