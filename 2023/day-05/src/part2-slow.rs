use color_eyre::Result;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Seeds(Vec<usize>);

impl FromStr for Seeds {
    type Err = &'static str;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let nums = s.split(':').last().map(|nums| {
            nums.split_whitespace()
                .map(|num| {
                    num.parse::<usize>()
                        .map_err(|_| "Seed should be a number")
                        .unwrap()
                })
                .collect::<Vec<_>>()
        });
        let seeds = nums
            .ok_or("No seeds found")?
            .chunks(2)
            .flat_map(|chunk| {
                let start = chunk[0];
                let end = start + chunk[1];
                (start..=end).collect::<Vec<usize>>()
            })
            .collect();

        Ok(Self(seeds))
    }
}

#[derive(Debug, Clone)]
struct SeedMap {
    dest_range: Vec<usize>,
    source_range: Vec<usize>,
    range_len: Vec<usize>,
}

impl SeedMap {
    fn iter(&self) -> impl Iterator<Item = (&usize, &usize, &usize)> {
        itertools::izip!(&self.dest_range, &self.source_range, &self.range_len)
    }
}

impl From<Vec<usize>> for SeedMap {
    fn from(value: Vec<usize>) -> Self {
        let mut dest_range = Vec::new();
        let mut source_range = Vec::new();
        let mut range_len = Vec::new();

        for chunk in value.chunks(3) {
            if let [dest, source, range] = chunk {
                dest_range.push(*dest);
                source_range.push(*source);
                range_len.push(*range);
            }
        }

        Self {
            dest_range,
            source_range,
            range_len,
        }
    }
}

pub fn process(input: &str) -> Result<usize> {
    let mut lines: Vec<_> = input.lines().collect();
    lines.push("");
    let seeds = Seeds::from_str(lines.remove(0)).unwrap();

    let mut nums: Vec<usize> = Vec::new();

    let maps: Vec<_> = lines
        .iter()
        .map(|line| {
            if !line.contains(':') && !line.is_empty() {
                line.split_whitespace().for_each(|num_str| {
                    if let Ok(num) = num_str.parse::<usize>() {
                        nums.push(num);
                    }
                });
            }

            if line.is_empty() && !nums.is_empty() {
                let map = SeedMap::from(nums.clone());
                nums.clear();
                Some(map)
            } else {
                None
            }
        })
        .filter_map(|map_option| map_option)
        .collect();

    let mut step = seeds.0;
    let mut next_step = Vec::new();

    for map in &maps {
        for (&dest_range, &source_range, &range_len) in map.iter() {
            for num in step.clone() {
                if (source_range..=source_range + range_len).contains(&num) {
                    step.retain(|&x| x != num);
                    let dest = num + dest_range - source_range;
                    next_step.push(dest);
                }
            }
        }
        next_step.append(&mut step);
        step = next_step.clone();
        next_step.clear();
    }

    Ok(*step.iter().min().unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
