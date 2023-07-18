use std::sync::Mutex;

pub struct InmemoryDB {
    registered_signup_hash_token: Mutex<Vec<String>>,
}

impl InmemoryDB {
    pub fn new() -> Self {
        InmemoryDB {
            registered_signup_hash_token: Mutex::new(vec![]),
        }
    }

    // Use the mutex lock to provide uniqueness guaranties over the image hash token
    pub fn check_existence_and_store(&self, image_hash_token: &str) -> bool {
        let mut tokens = self.registered_signup_hash_token.lock().unwrap();
        let new_hash = image_hash_token.to_string();
        if tokens.contains(&new_hash) {
            false
        } else {
            tokens.push(new_hash);
            true
        }
    }

    pub fn clear(&self) {
        let mut tokens = self.registered_signup_hash_token.lock().unwrap();
        tokens.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_existence_and_store() {
        let state = InmemoryDB::new();

        let result = state.check_existence_and_store("test");
        assert!(result, "It must be true if the image hash token is new");

        let result = state.check_existence_and_store("test");
        assert!(
            !result,
            "It mast be false if the image hash token already exist"
        );

        state.clear();
        let result = state.check_existence_and_store("test");
        assert!(result, "It's tru again after a reset");
    }
}
