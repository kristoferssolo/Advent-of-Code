use std::str::FromStr;

use color_eyre::Result;

#[derive(Debug, Clone)]
struct Record {
    time: usize,
    distance: usize,
    my_distance: Vec<usize>,
}

impl From<(usize, usize)> for Record {
    fn from(value: (usize, usize)) -> Self {
        let (time, distance) = value;
        Self {
            time,
            distance,
            my_distance: Vec::new(),
        }
    }
}

impl FromStr for Record {
    type Err = &'static str;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time_line = lines.next().ok_or("Invalid input format")?;
        let time = time_line
            .split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| "Failed to parse distance")?;

        let distance_line = lines.next().ok_or("Invalid input format")?;
        let distance = distance_line
            .split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| "Failed to parse distance")?;

        Ok(Record {
            time,
            distance,
            my_distance: Vec::new(),
        })
    }
}

pub fn process(input: &str) -> Result<usize> {
    let mut record: Record = input.parse().expect("Error parsing input");
    dbg!(&record);

    (0..=record.time).for_each(|time| {
        let distance = time * (record.time - time);
        record.my_distance.push(distance);
    });

    let total = record
        .my_distance
        .iter()
        .filter(|&&num| num > record.distance);

    Ok(total.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
