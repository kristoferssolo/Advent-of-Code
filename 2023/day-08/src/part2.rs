use std::{collections::HashMap, rc::Rc};

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
    let start: Vec<_> = maps
        .keys()
        .filter(|&key| key.ends_with('A'))
        .cloned()
        .collect();

    let mut current_values = start;
    let mut step_count = 0;
    println!("{:?}", &current_values);

    'outer: loop {
        for ch in rule.chars() {
            for value in &mut current_values {
                if let Some(side) = maps.get(value) {
                    match ch {
                        'L' => *value = side.0.clone(),
                        'R' => *value = side.1.clone(),
                        _ => return Err(anyhow!("Unexpected character in rule")),
                    }
                }
            }
            step_count += 1;
            if current_values.iter().all(|x| x.ends_with('Z')).clone() {
                break 'outer;
            }
        }
        println!("{}", step_count);
    }

    Ok(step_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, process(input)?);
        Ok(())
    }
}

