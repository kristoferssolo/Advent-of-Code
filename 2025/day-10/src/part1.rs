use miette::miette;
use std::{collections::HashSet, slice::Iter, str::FromStr, vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Light {
    On,
    Off,
}

impl TryFrom<char> for Light {
    type Error = String;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            _ => Err("Unknown Light status".to_string()),
        }
    }
}

impl Light {
    const fn toggle(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

// [.##.]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IndicatorLight(Vec<Light>);

impl FromStr for IndicatorLight {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let digits_str = trimmed
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or("");

        let lights = digits_str
            .chars()
            .map(Light::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(lights))
    }
}

impl IndicatorLight {
    fn toggle(&mut self, button: &Button) {
        for &btn in button.iter() {
            let light = self.0.get_mut(btn).expect("light exsists");
            *light = light.toggle();
        }
    }

    const fn len(&self) -> usize {
        self.0.len()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(vec![Light::Off; capacity])
    }
}

// (3) (1,3) (2) (2,3) (0,2) (0,1)
#[derive(Debug, Clone)]
struct Button(Vec<usize>);

impl FromStr for Button {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let digits_str = trimmed
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(trimmed);

        let button = digits_str
            .split(',')
            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(button))
    }
}

impl Button {
    fn iter(&self) -> Iter<'_, usize> {
        self.0.iter()
    }
}

// {3,5,4,7}
#[derive(Debug, Clone)]
struct Joltage(Vec<usize>);

impl FromStr for Joltage {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let digits_str = trimmed
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .unwrap_or(trimmed);

        let joltage = digits_str
            .split(',')
            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(joltage))
    }
}

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
#[derive(Debug, Clone)]
struct Machine {
    final_state: IndicatorLight, // [.##.]
    current_state: IndicatorLight,
    buttons: Vec<Button>, // (3) (1,3) (2) (2,3) (0,2) (0,1)
    _joltage: Joltage,    // {3,5,4,7}
}

impl FromStr for Machine {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        let bracket_start = trimmed.find('[').ok_or("Missing '['")?;
        let bracket_end = trimmed.find(']').ok_or("Missing ']'")?;
        let final_state = trimmed[bracket_start..=bracket_end].parse::<IndicatorLight>()?;
        let state_len = final_state.len();

        let brace_start = trimmed.find('{').ok_or("Missing '{'")?;
        let brace_end = trimmed.find('}').ok_or("Missing '}'")?;
        let joltage = trimmed[brace_start..=brace_end].parse()?;

        let buttons_str = trimmed[bracket_end + 1..brace_start].trim();
        let buttons = buttons_str
            .split_whitespace()
            .map(Button::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            final_state,
            current_state: IndicatorLight::with_capacity(state_len),
            buttons,
            _joltage: joltage,
        })
    }
}

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<usize> {
    let machines = input
        .lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| miette!("{e}"))?;

    let result = machines
        .iter()
        .map(|machine| {
            let mut set = HashSet::new();
            set.insert(machine.current_state.clone());
            let mut i = 0;
            loop {
                set = set
                    .into_iter()
                    .flat_map(|state| {
                        machine.buttons.iter().map(move |btn| {
                            let mut new_state = state.clone();
                            new_state.toggle(btn);
                            new_state
                        })
                    })
                    .collect();
                i += 1;
                if set.contains(&machine.final_state) {
                    break;
                }
            }
            i
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let result = 7;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
