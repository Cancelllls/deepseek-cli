pub fn verify_code: &str) -> bool {
    // stub: simple syntax check using syn
    syn::parse_file(code).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rust_code()        assert!(verify_code("fn main() {}"));
 }

    #[test]
    fn test_invalid_rust_code() {
        assert!(!verify_code("fn main("));
    }
}
