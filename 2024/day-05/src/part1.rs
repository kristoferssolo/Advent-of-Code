use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
enum OrderingError {
    #[error("No separator '{0}' was found")]
    NoSeparator(char),
    #[error("Too many values found ({0}) expected 2")]
    TooManyValues(usize),
    #[error("Not enough values found ({0}) expected 2")]
    NotEnoughtValues(usize),
    #[error("Failed to parse number")]
    ParseError,
}

#[derive(Debug)]
struct Ordering(usize, usize);

impl FromStr for Ordering {
    type Err = OrderingError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let splitter = '|';
        if !s.contains(splitter) {
            return Err(OrderingError::NoSeparator(splitter));
        }

        let pages = s.split(splitter).collect::<Vec<_>>();

        if pages.len() > 2 {
            return Err(OrderingError::TooManyValues(pages.len()));
        }
        if pages.len() < 2 {
            return Err(OrderingError::NotEnoughtValues(pages.len()));
        }

        let x = pages[0]
            .trim()
            .parse::<usize>()
            .map_err(|_| OrderingError::ParseError)?;
        let y = pages[1]
            .trim()
            .parse::<usize>()
            .map_err(|_| OrderingError::ParseError)?;

        Ok(Ordering(x, y))
    }
}

impl Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Rules(Vec<Ordering>);

impl Rules {
    fn check(&self, pages: &Pages) -> Option<Page> {
        let len = pages.0.len();
        if len < 2 {
            return None;
        }

        pages
            .0
            .windows(2)
            .all(|window| self.is_pair(&window[0], &window[1]))
            .then(|| pages.0[len / 2])
    }

    fn is_pair(&self, a: &Page, b: &Page) -> bool {
        self.0.iter().any(|Ordering(x, y)| *x == a.0 && *y == b.0)
    }
}

impl FromStr for Rules {
    type Err = OrderingError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(Ordering::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Rules(rules))
    }
}

#[derive(Debug, Error, Diagnostic)]
enum PageError {
    #[error("Failed to parse number")]
    ParseError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Page(usize);

impl FromStr for Page {
    type Err = PageError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let page = s.parse().map_err(|_| PageError::ParseError);
        Ok(Page(page?))
    }
}

#[derive(Debug)]
struct Pages(Vec<Page>);

impl FromStr for Pages {
    type Err = PageError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(Page::from_str)
            .collect::<Result<Vec<_>, _>>();
        Ok(Pages(pages?))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    let (rules, pages) = parse(input)?;
    let sum = pages
        .iter()
        .filter_map(|page| rules.check(page))
        .map(|page| page.0)
        .sum();
    Ok(sum)
}

fn parse(input: &str) -> Result<(Rules, Vec<Pages>)> {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let rules = Rules::from_str(sections.first().unwrap())?;
    let pages = get_pages(sections.last().unwrap())?;

    Ok((rules, pages))
}

fn get_pages(section: &str) -> Result<Vec<Pages>> {
    Ok(section
        .lines()
        .map(Pages::from_str)
        .collect::<Result<Vec<_>, _>>()?)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_RULES: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";

    #[rstest]
    #[case("75,47,61,53,29", Some(Page(61)))]
    #[case("97,61,53,29,13", Some(Page(53)))]
    #[case("75,29,13", Some(Page(29)))]
    #[case("75,97,47,61,53", None)]
    #[case("61,13,29", None)]
    #[case("97,13,75,29,47", None)]
    fn test_pages(#[case] input: &str, #[case] expected: Option<Page>) -> Result<()> {
        let rules = Rules::from_str(TEST_RULES)?;
        let pages = Pages::from_str(input)?;

        assert_eq!(rules.check(&pages), expected);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = 143;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
