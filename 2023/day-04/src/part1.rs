use std::{borrow::BorrowMut, str::FromStr};

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Card {
    id: usize,
    win_nums: Vec<usize>,
    my_nums: Vec<usize>,
    points: usize,
}

impl Card {
    fn add_point(&mut self) {
        if self.points == 0 {
            self.points = 1
        } else {
            self.points *= 2
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let line: Vec<_> = s.split(|ch| ch == ':' || ch == '|').collect();

        let id = line
            .get(0)
            .ok_or("Missing ID field")
            .and_then(|field| field.split_whitespace().last().ok_or("Invalid ID format"))
            .and_then(|id_str| id_str.parse::<usize>().map_err(|_| "Failed to parse ID"))?;

        let win_nums: Result<Vec<_>, _> = line
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

        let my_nums: Result<Vec<_>, _> =
            line.get(2)
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
            id,
            win_nums: win_nums?,
            my_nums: my_nums?,
            points: 0,
        })
    }
}

pub fn process(input: &str) -> color_eyre::Result<usize> {
    let mut cards: Vec<Card> = input.lines().flat_map(Card::from_str).collect();

    let sum = cards
        .iter_mut()
        .flat_map(|card| {
            for num in card.win_nums.clone() {
                if card.my_nums.contains(&num) {
                    card.add_point();
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
    fn test_process() -> color_eyre::Result<()> {
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
