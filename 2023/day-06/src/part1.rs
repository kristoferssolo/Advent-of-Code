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

#[derive(Debug, Clone)]
struct Records(Vec<Record>);
impl FromStr for Records {
    type Err = &'static str;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time_values: Vec<_> = lines
            .next()
            .and_then(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(|s| s.parse().ok())
                    .collect()
            })
            .ok_or("Invalid input format")?;
        let distabce_values: Vec<_> = lines
            .next()
            .and_then(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(|s| s.parse().ok())
                    .collect()
            })
            .ok_or("Invalid input format")?;
        let records: Vec<_> = time_values
            .into_iter()
            .zip(distabce_values.into_iter())
            .map(Record::from)
            .collect();
        Ok(Records(records))
    }
}

pub fn process(input: &str) -> Result<usize> {
    let mut records: Records = input.parse().expect("Error parsing input");
    records.0.iter_mut().for_each(|record| {
        (0..=record.time).for_each(|time| {
            let distance = time * (record.time - time);
            record.my_distance.push(distance);
        })
    });
    let total: usize = records
        .0
        .iter()
        .map(|record| {
            record
                .my_distance
                .iter()
                .filter(|&&num| num > record.distance)
                .count()
        })
        .product();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
