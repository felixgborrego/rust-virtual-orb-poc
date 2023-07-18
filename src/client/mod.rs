use tokio::task;

pub mod check_signup;
pub mod telemetry;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn start_client(
    telemetry_url: String,
    telemetry_interval: u32,
    orb_server: String,
    orb_signup_interval: u32,
) {
    println!("Virtual Orb Test Client {}", VERSION);

    task::spawn(telemetry::start_telemetry(
        telemetry_url.clone(),
        telemetry_interval,
    ));

    check_signup::start_check_signup(&orb_server, orb_signup_interval)
        .await
        .expect("Unable to start client")
}
