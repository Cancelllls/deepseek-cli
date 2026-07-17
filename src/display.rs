use colored::*;

pub fn print_info(msg: &str) {
    println!("{}", msg.blue());
}

pub fn print_error(msg: &str) {
    eprintln!("{}", msg.red());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_info() {
        // just ensure no panic
        print_info("test");
    }

    #[test]
    fn test_print_error() {
        print_error("test");
    }
}
