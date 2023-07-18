extern crate core;

use clap::Parser;
use felix_garcia_virtual_orb::client;
use felix_garcia_virtual_orb::server;
use log::LevelFilter;
use simple_logger::SimpleLogger;

// Command line arguments test client
#[derive(Parser, Debug)]
#[clap(version)]
enum CliArgsCommand {
    // Start the Virtual Orb Client
    Client {
        // Url of the telemetry lib
        #[arg(long)]
        telemetry_url: String,

        /// Telemetry reporting interval in seconds
        #[arg(long)]
        telemetry_interval: u32,

        /// Orb lib api url, example http://localhost:4242
        #[arg(long)]
        orb_server: String,

        /// Period of the Orb signup test interval in seconds
        #[arg(long)]
        orb_signup_interval: u32,
    },

    // Start the Virtual Orb Server
    Server {
        // Listening port for the API
        #[arg(long)]
        orb_api_port: u16,
    },
}

#[tokio::main]
async fn main() {
    let args = CliArgsCommand::parse();

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("server", LevelFilter::Info)
        .with_module_level("client", LevelFilter::Info)
        .with_module_level("rocket", LevelFilter::Off)
        .init()
        .unwrap();

    match args {
        CliArgsCommand::Client {
            telemetry_url,
            telemetry_interval,
            orb_server,
            orb_signup_interval,
        } => {
            client::start_client(
                telemetry_url,
                telemetry_interval,
                orb_server,
                orb_signup_interval,
            )
            .await
        }
        CliArgsCommand::Server {
            orb_api_port: orb_server_port,
        } => server::start_server(orb_server_port).await,
    }
}
