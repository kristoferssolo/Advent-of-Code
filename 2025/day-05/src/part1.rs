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

impl Range {
    fn contains(&self, x: Id) -> bool {
        x >= self.start && x <= self.end
    }
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
    ids: Vec<Id>,
}

impl DB {
    fn contains(&self, x: Id) -> bool {
        self.ranges.iter().any(|range| range.contains(x))
    }

    fn count_fresh(&self) -> usize {
        self.ids.iter().filter(|&&id| self.contains(id)).count()
    }
}

impl FromStr for DB {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ranges_section, ids_section) = s
            .split_once("\n\n")
            .ok_or("No blank line separator found")?;
        let ranges = ranges_section
            .lines()
            .map(Range::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let ids = ids_section
            .lines()
            .map(Id::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { ranges, ids })
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let db = DB::from_str(input).unwrap();
    Ok(db.count_fresh())
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
        let result = 3;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
