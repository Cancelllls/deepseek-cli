pub struct Session {
    pub id: String,
}

impl Session {
    pub fn new() -> Self {
        Session {
            id: uuid::Uuid::new4().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert!(!session.id.is_empty());
    }
}
