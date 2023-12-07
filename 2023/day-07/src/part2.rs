use std::{cmp::Ordering, collections::HashMap, convert::TryInto, str::FromStr};

use crate::error::{CombinationError, HandParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(ch: char) -> Result<Card, HandParseError> {
        match ch {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::_9),
            '8' => Ok(Card::_8),
            '7' => Ok(Card::_7),
            '6' => Ok(Card::_6),
            '5' => Ok(Card::_5),
            '4' => Ok(Card::_4),
            '3' => Ok(Card::_3),
            '2' => Ok(Card::_2),
            _ => Err(HandParseError::InvalidCard(ch)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Combination {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn parse_bid(s: &str) -> Result<usize, HandParseError> {
        s.parse().map_err(|e| HandParseError::InvalidBid(e))
    }

    fn combination(&self) -> Result<Combination, CombinationError> {
        let mut card_count = HashMap::new();
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| b.cmp(a));

        for &card in cards.iter() {
            *card_count.entry(card).or_insert(0) += 1;
            if card == Card::J {
                for (_, value) in card_count.iter_mut() {
                    *value += 1;
                }
            }
        }

        if let Some((_, _)) = card_count.iter().find(|(_, &count)| count == 5) {
            Ok(Combination::FiveOfAKind)
        } else if let Some((_, _)) = card_count.iter().find(|(_, &count)| count == 4) {
            Ok(Combination::FourOfAKind)
        } else if let Some((&card, _)) = card_count.iter().find(|(_, &count)| count == 3) {
            if card_count.contains_key(&card) && card_count.get(&card) == Some(&2) {
                Ok(Combination::FullHouse)
            } else {
                Ok(Combination::ThreeOfAKind)
            }
        } else {
            let pairs: Vec<Card> = card_count
                .iter()
                .filter(|&(_, &count)| count == 2)
                .map(|(&card, _)| card)
                .collect();
            match pairs.len() {
                2 => Ok(Combination::TwoPair),
                1 => Ok(Combination::OnePair),
                _ => Ok(Combination::HighCard),
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.combination(), other.combination()) {
            (Ok(self_comb), Ok(other_comb)) => match self_comb.cmp(&other_comb) {
                Ordering::Equal => {
                    for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                        match self_card.cmp(other_card) {
                            Ordering::Equal => continue,
                            other => return other,
                        }
                    }
                    Ordering::Equal
                }
                other => other,
            },
            (Err(_), _) => Ordering::Less,
            (_, Err(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let hand: Vec<&str> = s.split_whitespace().collect();

        let cards: Result<Vec<Card>, HandParseError> = hand
            .first()
            .ok_or_else(|| HandParseError::InvalidCardCount)?
            .chars()
            .map(Card::from_char)
            .collect();

        let bid = Self::parse_bid(hand.last().ok_or(HandParseError::InvalidCardCount)?)?;

        Ok(Self {
            cards: cards?
                .try_into()
                .map_err(|_| HandParseError::InvalidCardCount)?,
            bid,
        })
    }
}

pub fn process(input: &str) -> color_eyre::Result<usize> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| Vec::new());

    hands.sort();

    hands.iter().for_each(|hand| {
        println!(
            "{:?} \t| {:?}\t| {}",
            hand.cards,
            hand.combination().unwrap(),
            hand.bid
        );
    });

    let total = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> color_eyre::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, process(input)?);
        Ok(())
    }
}
