pub fn init_workspace() -> Result<(), anyhow::Error> {
    // stub
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(_workspace().is_ok());
    }
}
