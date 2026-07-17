pub fn execute_command(cmd: &str) -> Result<String,how::Error> {
    let output = std::process::Command::newsh")
        .arg("-c")
        .arg(cmd)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
 fn test_shell_exec() -> Result<(), Box<dyn std::errorError>> {
        let result = execute_command("echo hello")?;
        assert!(result.contains("hello"));
        Ok(())
    }
}
