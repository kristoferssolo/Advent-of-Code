use std::str::FromStr;

use color_eyre::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SpringStatus {
    Operational,
    Unknown,
    Damaged,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Spring {
    status: Vec<SpringStatus>,
    condition: Vec<usize>,
}

impl Spring {
    fn arragement_amount(&self) -> usize {
        let mut combinations = vec![vec![]];
        for status in &self.status {
            let mut new_combinations = Vec::new();
            for combination in &combinations {
                match status {
                    SpringStatus::Operational => {
                        let mut new_combination = combination.clone();
                        new_combination.push(*status);
                        new_combinations.push(new_combination);
                    }
                    SpringStatus::Unknown => {
                        let mut new_combination = combination.clone();
                        new_combination.push(SpringStatus::Operational);
                        new_combinations.push(new_combination);
                        let mut new_combination = combination.clone();
                        new_combination.push(SpringStatus::Damaged);
                        new_combinations.push(new_combination);
                    }
                    SpringStatus::Damaged => {
                        let mut new_combination = combination.clone();
                        new_combination.push(*status);
                        new_combinations.push(new_combination);
                    }
                }
            }
            combinations = new_combinations;
        }

        let combinations = combinations
            .iter()
            .map(|combination| {
                let mut combination_status: Vec<usize> = Vec::new();
                let mut count = 0;
                combination.iter().for_each(|status| {
                    match status {
                        SpringStatus::Operational => {
                            if count > 0 {
                                combination_status.push(count);
                            }
                            count = 0;
                        }
                        SpringStatus::Damaged => count += 1,
                        SpringStatus::Unknown => unimplemented!("Should not happen"),
                    };
                });
                if count > 0 {
                    combination_status.push(count);
                }
                combination_status
            })
            .filter(|status| status == &self.condition);

        combinations.count()
    }
}

impl FromStr for Spring {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let line = s.split_whitespace().collect::<Vec<_>>();
        let status = line
            .first()
            .unwrap()
            .chars()
            .map(|ch| match ch {
                '?' => SpringStatus::Unknown,
                '.' => SpringStatus::Operational,
                '#' => SpringStatus::Damaged,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let condition = line
            .last()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut new_status = status.clone();
        let mut new_condition = condition.clone();

        for _ in 0..4 {
            new_status.push(SpringStatus::Unknown);
            new_status.extend(status.clone().iter());
            new_condition.extend(condition.clone().iter());
        }

        dbg!(&new_status, &new_condition);

        Ok(Self {
            status: new_status,
            condition: new_condition,
        })
    }
}

pub fn process(input: &str) -> Result<usize> {
    let springs = input.lines().map(|line| {
        Spring::from_str(line)
            .unwrap_or_default()
            .arragement_amount()
    });
    Ok(springs.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";
        assert_eq!(525152, process(input)?);
        Ok(())
    }

    #[test]
    fn test_from_str() -> Result<()> {
        let input = "???.### 1,1,3";
        let spring = Spring::from_str(input)?;
        assert_eq!(
            vec![
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Operational,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Operational,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Operational,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Operational,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Unknown,
                SpringStatus::Operational,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
                SpringStatus::Damaged,
            ],
            spring.status
        );
        assert_eq!(
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
            spring.condition
        );
        Ok(())
    }

    #[test]
    fn test_line_1() -> Result<()> {
        assert_eq!(1, Spring::from_str("???.### 1,1,3")?.arragement_amount());
        Ok(())
    }

    #[test]
    fn test_line_2() -> Result<()> {
        assert_eq!(
            16384,
            Spring::from_str(".??..??...?##. 1,1,3")?.arragement_amount()
        );
        Ok(())
    }

    #[test]
    fn test_line_3() -> Result<()> {
        assert_eq!(
            1,
            Spring::from_str("?#?#?#?#?#?#?#? 1,3,1,6")?.arragement_amount()
        );
        Ok(())
    }

    #[test]
    fn test_line_4() -> Result<()> {
        assert_eq!(
            16,
            Spring::from_str("????.#...#... 4,1,1")?.arragement_amount()
        );
        Ok(())
    }

    #[test]
    fn test_line_5() -> Result<()> {
        assert_eq!(
            2500,
            Spring::from_str("????.######..#####. 1,6,5")?.arragement_amount()
        );
        Ok(())
    }

    #[test]
    fn test_line_6() -> Result<()> {
        assert_eq!(
            506250,
            Spring::from_str("?###???????? 3,2,1")?.arragement_amount()
        );
        Ok(())
    }
}
