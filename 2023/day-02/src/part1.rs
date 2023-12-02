use color_eyre::Result;

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    bags: Vec<Bag>,
}

impl Game {
    fn new(id: u32) -> Self {
        Game {
            id,
            bags: Vec::new(),
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Bag { red, green, blue }
    }

    fn add_red(&mut self, num: u32) {
        self.red += num;
    }

    fn add_green(&mut self, num: u32) {
        self.green += num;
    }

    fn add_blue(&mut self, num: u32) {
        self.blue += num;
    }

    fn compare(&self, other: &Self) -> bool {
        self.red <= other.red && (self.green <= other.green) && self.blue <= other.blue
    }
}

fn parse_color(color: &str) -> Option<Color> {
    let color = color.to_lowercase();
    if color.contains("red") {
        Some(Color::Red)
    } else if color.contains("green") {
        Some(Color::Green)
    } else if color.contains("blue") {
        Some(Color::Blue)
    } else {
        None
    }
}

fn parse_line(line: &str) -> Result<Game> {
    let mut parts: Vec<_> = line.split(|c| c == ';' || c == ':').collect();
    let id: u32 = parts
        .remove(0)
        .split_whitespace()
        .last()
        .unwrap_or("ID not found")
        .parse()?;

    let mut game = Game::new(id);

    parts.iter().for_each(|part| {
        let colors: Vec<_> = part.split_whitespace().collect();
        let mut bag = Bag::default();
        for chunk in colors.chunks_exact(2).into_iter() {
            if let Some(parsed_color) = parse_color(chunk[1]) {
                if let Ok(amount) = chunk.get(0).unwrap_or(&"ID not found").parse::<u32>() {
                    match parsed_color {
                        Color::Red => bag.add_red(amount),
                        Color::Green => bag.add_green(amount),
                        Color::Blue => bag.add_blue(amount),
                    }
                }
            }
        }
        game.bags.push(bag);
    });

    Ok(game)
}

pub fn process(input: &str) -> Result<u32> {
    let max_bag = Bag::new(12, 13, 14);

    let games: Vec<Game> = input.lines().flat_map(|line| parse_line(line)).collect();

    let sum = games
        .iter()
        .filter(|game| game.bags.iter().all(|bag| bag.compare(&max_bag)))
        .map(|game| game.id)
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
