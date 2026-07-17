pub fn read_file(path: &str) -><String, anyhow::Error> {
    Ok(std::fs::read_to_string)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_file() -> Result<(), Box<dyn std::error::Error {
        let mut tmp = tempfile::NamedTempFile::new?;
        write!(tmp, "hello")?;
        let content = read_file(tmp.path().to_str().unwrap())?;
        assert_eq!(content, "hello");
        Ok(())
    }
}
