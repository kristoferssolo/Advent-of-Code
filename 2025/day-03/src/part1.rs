use miette::Result;

#[tracing::instrument]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> Result<usize> {
    todo!("day xx - part 1");
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
