use std::{collections::HashMap, ops::Deref, rc::Rc};

use color_eyre::{eyre::anyhow, Result};

fn parse_input(input: &str) -> Result<(&str, HashMap<Rc<str>, (Rc<str>, Rc<str>)>)> {
    let mut lines = input.lines();
    let rule = lines.next().ok_or_else(|| anyhow!("Path"))?.into();
    lines.next();
    let mut maps = HashMap::new();
    for line in lines {
        let binding: String = line
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");
        let values = binding.split_whitespace().collect::<Vec<_>>();
        maps.insert(values[0].into(), (values[1].into(), values[2].into()));
    }

    Ok((rule, maps))
}

pub fn process(input: &str) -> Result<usize> {
    let (rule, maps) = parse_input(input)?;
    let (start, end) = ("AAA", "ZZZ");
    let mut current: Rc<str> = Rc::from(start);
    let mut step_count = 0;

    while current.deref() != end {
        for ch in rule.chars() {
            if let Some(side) = maps.get(&current) {
                match ch {
                    'L' => current = side.0.clone(),
                    'R' => current = side.1.clone(),
                    _ => return Err(anyhow!("Should not have happened")),
                }
                step_count += 1;
            }
        }
    }

    Ok(step_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}
