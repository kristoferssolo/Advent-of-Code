use miette::Result;

#[tracing::instrument]
pub fn process(input: &str) -> Result<usize> {
    todo!("day xx - part 2");
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "";
        todo!("haven't built test yet");
        let result = 0;
        assert_eq!(process(input)?, result);
        Ok(())
    }
}
