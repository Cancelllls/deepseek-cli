pub fn route_command(input: &str) -> &'static str {
 if input.starts_with("file") {
        "file_tool"
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[]
    fn test_route_file() {
        assert_eq!(route_command("file read test.txt"),file_tool");
    }

    #[test]
    fn test_unknown_route() {
        assert_eq!(route_command("unknown"), "unknown");
    }
}
