use color_eyre::Result;
use {{crate_name}}::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
