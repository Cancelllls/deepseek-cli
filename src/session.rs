pub struct Session {
    pub id: String,
}

impl Session {
    fn new() -> Self {
        Session {
            id: uuid::Uuid::new_v4().to_string(),
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
