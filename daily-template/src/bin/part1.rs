use color_eyre::Result;
use {{crate_name}}::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
