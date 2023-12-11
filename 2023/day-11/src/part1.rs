use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Universe {
    Galaxy,
    Empty,
}

impl Universe {
    fn is_empty(&self) -> bool {
        match self {
            Universe::Galaxy => false,
            Universe::Empty => true,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> usize {
        (self.x as i32 - other.x as i32).abs() as usize
            + (self.y as i32 - other.y as i32).abs() as usize
    }
}

fn expand_universe(input: Vec<Vec<Universe>>) -> Result<Vec<Vec<Universe>>> {
    let mut output = input.clone();
    let mut add_cols = Vec::new();
    let mut add_rows = Vec::new();

    // Check if any row contains only dots
    for (idx, line) in input.iter().enumerate() {
        if line.iter().all(|&ch| ch.is_empty()) {
            add_rows.push(idx + 1);
        }
    }

    // Check if any column contains only dots
    let num_cols = input.first().unwrap().len();
    for col in 0..num_cols {
        if input.iter().all(|line| line.get(col).unwrap().is_empty()) {
            add_cols.push(col);
        }
    }

    // Add rows if necessary
    for row in add_rows {
        let new_row = vec![Universe::Empty; num_cols];
        output.insert(row, new_row);
    }

    // Add columns if necessary
    for (idx, &col) in add_cols.iter().enumerate() {
        for line in output.iter_mut() {
            line.insert(col + idx + 1, Universe::Empty);
        }
    }

    Ok(output)
}

pub fn process(input: &str) -> Result<usize> {
    let universe: Vec<Vec<Universe>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Universe::Empty,
                    '#' => Universe::Galaxy,
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect();

    // Create a vector of points
    let points = expand_universe(universe)?.iter().enumerate().fold(
        Vec::new(),
        |acc: Vec<Point>, (y, row)| {
            row.iter().enumerate().fold(acc, |mut acc, (x, col)| {
                if !col.is_empty() {
                    acc.push(Point::new(x + 1, y + 1));
                }
                acc
            })
        },
    );

    // Sum the shortest path between each pair of points
    let sum = points.iter().enumerate().fold(0, |acc, (idx, point)| {
        points
            .iter()
            .skip(idx + 1)
            .fold(acc, |acc, other| acc + point.distance(other))
    });

    Ok(sum)

    /* TRIED

    - 10492576

    */
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, process(input)?);
        Ok(())
    }

    #[test]
    fn test_expand_no_expansion_needed() -> Result<()> {
        let input = vec![
            vec![Universe::Galaxy, Universe::Empty],
            vec![Universe::Empty, Universe::Galaxy],
        ];
        assert_eq!(expand_universe(input.clone())?, input);
        Ok(())
    }

    #[test]
    fn test_expand_last_rows() -> Result<()> {
        let input = vec![
            vec![Universe::Galaxy, Universe::Galaxy],
            vec![Universe::Empty, Universe::Empty],
        ];
        assert_eq!(
            expand_universe(input)?,
            vec![
                vec![Universe::Galaxy, Universe::Galaxy],
                vec![Universe::Empty, Universe::Empty],
                vec![Universe::Empty, Universe::Empty],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_expand_first_rows() -> Result<()> {
        let input = vec![
            vec![Universe::Empty, Universe::Empty],
            vec![Universe::Galaxy, Universe::Galaxy],
        ];
        assert_eq!(
            expand_universe(input)?,
            vec![
                vec![Universe::Empty, Universe::Empty],
                vec![Universe::Empty, Universe::Empty],
                vec![Universe::Galaxy, Universe::Galaxy],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_expand_first_columns() -> Result<()> {
        let input = vec![
            vec![Universe::Empty, Universe::Galaxy],
            vec![Universe::Empty, Universe::Galaxy],
        ];
        assert_eq!(
            expand_universe(input)?,
            vec![
                vec![Universe::Empty, Universe::Empty, Universe::Galaxy],
                vec![Universe::Empty, Universe::Empty, Universe::Galaxy],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_expand_last_columns() -> Result<()> {
        let input = vec![
            vec![Universe::Galaxy, Universe::Empty],
            vec![Universe::Galaxy, Universe::Empty],
        ];
        assert_eq!(
            expand_universe(input)?,
            vec![
                vec![Universe::Galaxy, Universe::Empty, Universe::Empty],
                vec![Universe::Galaxy, Universe::Empty, Universe::Empty],
            ]
        );
        Ok(())
    }
}
