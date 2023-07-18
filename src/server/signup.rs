use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{Error, ErrorKind};

use base64::{engine, Engine};
use log::{info, warn};

use crate::server::in_memory_db::InmemoryDB;
use crate::server::SignupRequest;

pub fn process_signup(state: &InmemoryDB, request: &SignupRequest) -> Result<String, io::Error> {
    info!("📩 New Signup request Id: {}", request.id);
    let decode_img = engine::general_purpose::STANDARD
        .decode(&request.image_base64)
        .map_err(|err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to decode Base64 image: {}", err),
            )
        })?;

    let hash = orb_image_hash(decode_img.as_slice());
    if state.check_existence_and_store(&hash) {
        Ok(hash)
    } else {
        warn!("Orb Image already exist");
        Err(Error::new(
            ErrorKind::AlreadyExists,
            "Orb image already exist".to_string(),
        ))
    }
}

fn orb_image_hash(image: &[u8]) -> String {
    // Very naive Orb hash
    let mut hasher = DefaultHasher::new();
    image.hash(&mut hasher);
    let hash_value = hasher.finish();
    format!("{:x}", hash_value)
}

// Module to test the signup process
#[cfg(test)]
mod tests {
    use super::*;

    static IMAGE_DATA_PNG: &[u8] = include_bytes!("../../resources/check_file_very_human.png");

    #[test]
    fn test_orb_image_hash() {
        let hash1 = orb_image_hash(&[1, 2, 3, 4, 5]);
        let hash2 = orb_image_hash(&[5, 5, 5, 5, 5]);
        let hash3 = orb_image_hash(IMAGE_DATA_PNG);

        assert_eq!(hash1, "371dd8e3f2423f4a", "Expected Orb Image hash token");
        assert_eq!(hash2, "c3094d9734dac10b", "Expected Orb Image hash token");
        assert_eq!(hash3, "8cb94f86c85039ae", "Expected Orb Image hash token");
    }

    #[test]
    fn test_valid_signup_process() {
        let state = InmemoryDB::new();
        let image = vec![1, 2, 3, 4, 5];
        let signup_request = SignupRequest {
            id: "test_id".to_string(),
            image_base64: engine::general_purpose::STANDARD.encode(&image),
        };
        let result = process_signup(&state, &signup_request);
        assert!(result.is_ok());
        let result = process_signup(&state, &signup_request);
        assert!(
            result.is_err(),
            "Expected error, the image hash already exist"
        );
    }
}
