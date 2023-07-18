use std::time::Duration;

use log::{error, info};
use rand::Rng;
use serde::{Deserialize, Serialize};

// Telemetry info to send to the server
#[derive(Serialize, Deserialize, Debug)]
struct TelemetryStatus {
    cpu_usage: f32,
    memory_usage: f32,
    disk_space: f32,
    timestamp: u32,
}

impl TelemetryStatus {
    fn new() -> Self {
        TelemetryStatus {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_space: 0.0,
            timestamp: 1,
        }
    }

    // Update the telemetry info (random values)
    fn refresh(&mut self) {
        let mut rng = rand::thread_rng();
        self.cpu_usage = rng.gen_range(10..80) as f32 / 100.0;
        self.memory_usage = rng.gen_range(10..80) as f32 / 100.0;
        self.disk_space = rng.gen_range(10..80) as f32 / 100.0;
        self.timestamp = 1;
    }
}

// refresh the telemetry status
pub async fn start_telemetry(telemetry_url: String, telemetry_interval: u32) {
    match telemetry_interval {
        0 => {
            info!("📡 Telemetry disabled");
        }
        _ => {
            info!("📡 Telemetry started...");
            let mut telemetry = TelemetryStatus::new();

            loop {
                telemetry.refresh();
                send_telemetry(&telemetry, &telemetry_url).await;

                tokio::time::sleep(Duration::from_secs(telemetry_interval as u64)).await;
            }
        }
    }
}

async fn send_telemetry(telemetry: &TelemetryStatus, telemetry_url: &str) {
    let client = reqwest::Client::new();
    let res = client.post(telemetry_url).json(&telemetry).send().await;

    match res {
        Ok(_) => {
            info!("📡 Telemetry sent");
        }
        Err(e) => {
            error!("📡 Unable to send telemetry error: {}", e);
        }
    }
}
