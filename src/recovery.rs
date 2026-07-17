pub fn recover_session(id: &str) -> Option<String> {
    if id.is_empty() {
        None
    } else {
        Some(format!("Recovered session {}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_valid_id() {
        assert_eq!(
            recover_session("123").unwrap(),
            "Recovered session 123"
        );
    }

    #[test]
    test_recover_empty_id() {
        assert!(recover_session("").is_none());
    }
}
