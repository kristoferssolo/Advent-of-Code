use color_eyre::Result;
use day_15::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
