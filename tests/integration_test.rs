use tokio::task;

use felix_garcia_virtual_orb::client::check_signup;
use felix_garcia_virtual_orb::server;

const TEST_PORT: u16 = 4242;

// Fully test the Orb signup protocol end to end
// Start a server in a separate thread
// Use the client to signup and check duplicated signup
#[tokio::test]
async fn test_orb_signup_protocol() {
    task::spawn(server::start_server(TEST_PORT));
    let url: String = format!("http://localhost:{}", TEST_PORT);
    let new_signup_result = check_signup::check_signup(&url).await;
    assert_eq!(new_signup_result, Ok("8cb94f86c85039ae".to_string()));

    let duplicated_signup_result = check_signup::check_signup(&url).await;
    assert_eq!(
        duplicated_signup_result,
        Err("❌ Signup request conflict - Image is already used".to_string())
    );
    reset(&url).await;

    let valid_again_result = check_signup::check_signup(&url).await;
    assert_eq!(valid_again_result, Ok("8cb94f86c85039ae".to_string()));
}

async fn reset(url: &str) {
    _ = reqwest::Client::new()
        .post(format!("{}/api/reset", url))
        .send()
        .await;
}
