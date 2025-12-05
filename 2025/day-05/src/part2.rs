use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl FromStr for Id {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.trim().parse::<usize>().map_err(|e| e.to_string())?;
        Ok(Self(num))
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: Id,
    end: Id,
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.trim().split_once('-').ok_or("`-` not found")?;
        let start = start_str.parse::<Id>()?;
        let end = end_str.parse::<Id>()?;
        Ok(Self { start, end })
    }
}

#[derive(Debug, Clone)]
struct DB {
    ranges: Vec<Range>,
}

impl DB {
    fn count_range_ids(&self) -> usize {
        let mut sorted_ranges = self.ranges.clone();
        sorted_ranges.sort_by_key(|r| r.start.0);

        let (total, start, end) =
            sorted_ranges
                .into_iter()
                .fold((0, None, None), |(total, s, e), range| match (s, e) {
                    (Some(s), Some(e)) if range.start.0 <= e + 1 => {
                        (total, Some(s), Some(range.end.0.max(e)))
                    }
                    (Some(s), Some(e)) => {
                        (total + (e - s + 1), Some(range.start.0), Some(range.end.0))
                    }
                    _ => (total, Some(range.start.0), Some(range.end.0)),
                });

        total + start.zip(end).map_or(0, |(s, e)| e - s + 1)
    }
}

impl FromStr for DB {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ranges_section, _) = s
            .split_once("\n\n")
            .ok_or("No blank line separator found")?;
        let ranges = ranges_section
            .lines()
            .map(Range::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { ranges })
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let db = DB::from_str(input).unwrap();
    Ok(db.count_range_ids())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let result = 14;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
