use color_eyre::Result;

pub fn process(_input: &str) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "";
        assert_eq!(0, process(input)?);
        Ok(())
    }
}
