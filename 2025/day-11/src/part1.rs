use itertools::Itertools;
use miette::miette;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Name([char; 3]);

impl Name {
    const YOU: Self = Self(['y', 'o', 'u']);
    const OUT: Self = Self(['o', 'u', 't']);
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
        dfs(&Name::YOU, &self.0)
    }
}

fn dfs(current: &Name, devices: &[Device]) -> usize {
    if current == &Name::OUT {
        return 1;
    }
    devices
        .iter()
        .find(|d| d.input == *current)
        .map_or(0, |device| {
            device
                .outputs
                .iter()
                .map(|output| dfs(output, devices))
                .sum()
        })
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
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let result = 5;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
