use color_eyre::eyre::Result;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
struct Card {
    win_nums: HashSet<usize>,
    my_nums: HashSet<usize>,
    points: usize,
}

impl Card {
    fn score(&mut self) {
        self.points = if self.points == 0 { 1 } else { self.points * 2 };
    }
}

impl FromStr for Card {
    type Err = &'static str;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let line: Vec<_> = s.split(|ch| ch == ':' || ch == '|').collect();

        let win_nums = line
            .get(1)
            .ok_or("Missing winning numbers field")
            .and_then(|field| {
                field
                    .split_whitespace()
                    .map(|num| {
                        num.parse::<usize>()
                            .map_err(|_| "Winning numbers should be numbers")
                    })
                    .collect()
            });

        let my_nums = line
            .get(2)
            .ok_or("Missing 'my' numbers field")
            .and_then(|field| {
                field
                    .split_whitespace()
                    .map(|num| {
                        num.parse::<usize>()
                            .map_err(|_| "'My' numbers should be numbers")
                    })
                    .collect()
            });

        Ok(Self {
            win_nums: win_nums?,
            my_nums: my_nums?,
            points: 0,
        })
    }
}

pub fn process(input: &str) -> Result<usize> {
    let mut cards: Vec<Card> = input.lines().flat_map(Card::from_str).collect();

    let sum = cards
        .iter_mut()
        .flat_map(|card| {
            for num in card.win_nums.clone() {
                if card.my_nums.contains(&num) {
                    card.score();
                }
            }
            Some(card.points)
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
