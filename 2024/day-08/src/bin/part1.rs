use day_08::part1::process;
use miette::{Context, Result};

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{result}");
    Ok(())
}
