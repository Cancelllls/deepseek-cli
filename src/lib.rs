pub mod workspace;
pub mod tools;
pub mod verifier;
pub mod display;
pub mod session;
pub mod router;
pub mod recovery;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
