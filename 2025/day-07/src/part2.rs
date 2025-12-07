use std::collections::HashMap;
use tracing::info;

type Column = usize;
type PathCount = usize;

#[derive(Debug, Clone)]
struct PathCounts {
    positions: HashMap<Column, PathCount>,
}

impl PathCounts {
    fn new(start_column: Column) -> Self {
        let mut positions = HashMap::new();
        positions.insert(start_column, 1);
        Self { positions }
    }
    fn apply_row(&self, line: &str) -> Self {
        let mut new_positions = HashMap::new();

        for (&column, &count) in &self.positions {
            if line.as_bytes()[column] == b'^' {
                info!(column, "split at");
                *new_positions.entry(column - 1).or_insert(0) += count;
                *new_positions.entry(column + 1).or_insert(0) += count;
            } else {
                *new_positions.entry(column).or_insert(0) += count;
            }
        }

        Self {
            positions: new_positions,
        }
    }

    fn total_paths(&self) -> usize {
        self.positions.values().sum()
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut lines = input.lines().enumerate();
    let (_, first_line) = lines.next().unwrap();
    let start_column = first_line.chars().position(|ch| ch == 'S').unwrap();

    let final_state = lines.fold(PathCounts::new(start_column), |state, (_, line)| {
        state.apply_row(line)
    });
    Ok(final_state.total_paths())
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
        let result = 40;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
