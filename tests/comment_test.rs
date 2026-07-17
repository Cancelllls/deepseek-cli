#[test]
fn test_comment_present_after_shebang() {
    let content = include_str!("../src/main.rs");
    let lines: Vec<&str = content.lines().collect();
    
    // Shebang should be first
    assert!(lines[0].starts_with("#!/"), "Shebang missing");
    
    // Comment should be second line exactly
    assert_eq!(lines[1], "//Seek CLI v0.1.0", "Version comment missing or malformed");
    
    // Ensure no duplicate comment appears later
    let comment_count = lines.iter().filter(|l| l.contains("DeepSeek CLI v0.1.0")).count();
    assert_eq!(_count, 1, "Comment must appear exactly once");
}
