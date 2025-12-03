use day_03::part2::process;
use miette::{Context, Result};

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{result}");
    Ok(())
}
