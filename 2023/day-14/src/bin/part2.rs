use color_eyre::Result;
use day_14::part2::process;

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
