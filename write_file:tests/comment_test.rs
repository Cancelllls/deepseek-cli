#[test]
fn test_commentresent_after_shebang() {
    let = include_str!("src/main.rs");
    let lines: Vec<&str> = content.lines().collect();
    
    // Shebang should be first
    assert!(lines[0].starts_with("#!/"), "Shebang missing");
    
    // Comment should be second line exactly
    assert_eq!(lines[1], "// DeepSeek CLI v.1.0", "Version comment missing or malformed");
    
    // Ensure no duplicate comment appears later
    let comment_count = lines.iter().filter(|l| l.contains("DeepSeek CLI v.1.0")).count();
    assert_eq!(_count, 1, "Comment must appear exactly once");
}
