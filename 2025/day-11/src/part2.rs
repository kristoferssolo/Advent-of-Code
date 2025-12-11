use itertools::Itertools;
use miette::miette;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Name([char; 3]);

impl Name {
    const OUT: Self = Self(['o', 'u', 't']);
    const DAC: Self = Self(['d', 'a', 'c']);
    const FFT: Self = Self(['f', 'f', 't']);
    const SVR: Self = Self(['s', 'v', 'r']);
}

impl FromStr for Name {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.trim().chars().collect_vec();
        if chars.len() != 3 {
            return Err(format!(
                "Name must be exactly 3 characters, got {}",
                chars.len()
            ));
        }
        Ok(Self([chars[0], chars[1], chars[2]]))
    }
}

#[derive(Debug, Clone)]
struct Device {
    input: Name,
    outputs: Vec<Name>,
}

impl FromStr for Device {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let (input_str, output_str) = trimmed.split_once(':').ok_or("Missing `:`")?;

        Ok(Self {
            input: Name::from_str(input_str)?,
            outputs: output_str
                .split_whitespace()
                .map(Name::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Rack(Vec<Device>);

impl FromStr for Rack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rack = s
            .lines()
            .map(Device::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(rack))
    }
}

impl Rack {
    fn solve(&self) -> usize {
        let mut memo = HashMap::new();
        let mut computing = HashSet::new();
        dfs(&Name::SVR, &self.0, false, false, &mut memo, &mut computing)
    }
}

fn dfs(
    current: &Name,
    devices: &[Device],
    visited_dac: bool,
    visited_fft: bool,
    memo: &mut HashMap<(Name, bool, bool), usize>,
    computing: &mut HashSet<(Name, bool, bool)>,
) -> usize {
    let state = (*current, visited_dac, visited_fft);

    if let Some(&result) = memo.get(&state) {
        return result;
    }

    if computing.contains(&state) {
        return 0;
    }

    if current == &Name::OUT {
        return usize::from(visited_dac && visited_fft);
    }

    computing.insert(state);

    let new_visited_dac = visited_dac || (current == &Name::DAC);
    let new_visited_fft = visited_fft || (current == &Name::FFT);

    let result = devices
        .iter()
        .find(|d| d.input == *current)
        .map_or(0, |device| {
            device
                .outputs
                .iter()
                .map(|output| {
                    dfs(
                        output,
                        devices,
                        new_visited_dac,
                        new_visited_fft,
                        memo,
                        computing,
                    )
                })
                .sum()
        });

    computing.remove(&state);
    memo.insert(state, result);
    result
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let rack = Rack::from_str(input).map_err(|e| miette!("{e}"))?;
    Ok(rack.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let result = 2;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
