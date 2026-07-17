pub mod file;
pub mod shell;

pub fn run_tool(name: &str) -> String {
    format!("Running tool: {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_tool() {
        assert_eq!(run_tool("hello"), "Running tool: hello");
    }
}
