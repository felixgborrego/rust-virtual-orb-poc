use std::error::Error;
use std::time::Duration;

use base64;
use base64::{engine, Engine};
use log::{error, info};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

// Image used for the check file
static IMAGE_DATA_PNG: &[u8] = include_bytes!("../../resources/check_file_very_human.png");

type ErrorMessage = String;
type SignupHashToken = String;

// Json request to send to the Orb server to signup
#[derive(Serialize, Deserialize, Debug)]
struct SignupRequest {
    id: String,
    image_base64: String,
}

impl SignupRequest {
    fn new(image_bytes: &[u8]) -> Self {
        SignupRequest {
            id: Ulid::new().to_string(), // generate a unique id per signup
            image_base64: engine::general_purpose::STANDARD.encode(image_bytes),
        }
    }
}

fn capture_image() -> Vec<u8> {
    // mock the capture image function fetching an static image of a real human eye
    IMAGE_DATA_PNG.to_vec()
}

// Start the signup check loop
pub async fn start_check_signup(
    orb_server: &str,
    orb_signup_interval: u32,
) -> Result<(), Box<dyn Error>> {
    match orb_signup_interval {
        0 => {
            info!("👁 Signup check disabled");
            Ok(())
        }
        _ => {
            info!("👁 Signup check started...");
            loop {
                {
                    let check_result = check_signup(orb_server).await;
                    match check_result {
                        Ok(signup_hash_token) => {
                            info!(
                                "✅ Signup request successful! Unique Image Hash Token {}",
                                signup_hash_token
                            )
                        }
                        Err(error_msg) => error!("error: {}", error_msg),
                    }
                    tokio::time::sleep(Duration::from_secs(orb_signup_interval.into())).await;
                }
            }
        }
    }
}

// Call the signup endpoint and and returns the assigned image hash token
// or an error message if unable to signup
pub async fn check_signup(orb_server: &str) -> Result<SignupHashToken, ErrorMessage> {
    let image_bytes = capture_image();
    let request = SignupRequest::new(image_bytes.as_slice());

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/signup", orb_server))
        .json(&request)
        .send()
        .await
        .map_err(|err| {
            // Handle connection or request errors
            format!("❌️ Signup unable to connect {}", err)
        })?;

    if res.status().is_success() {
        Ok(res.text().await.map_err(|_| "Unable to process response")?)
    } else if res.status() == StatusCode::CONFLICT {
        Err("❌ Signup request conflict - Image is already used".to_string())
    } else {
        Err("❌️ Signup request failed".to_string())
    }
}
