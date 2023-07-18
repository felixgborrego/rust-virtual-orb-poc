Virtual Orb CLI
======================================

This project shows a dummy implementation of the Virtual Orb CLI using Rust.
Warning: Nothing interesting to see here, just self-education code.

Components:

* A Virtual Orb client: Send telemetry data and simulate the signup by submitting images to the Virtual Orb server
* A Virtual Orb Server: A simple backend that processes the signup request 

[Video showing the client and server in action](https://www.loom.com/share/299e10c9180f4db780427d9652886059?sid=1f5de49d-3cb4-4e3e-8a0c-bd4729c818d5)

### Some considerations:
- The telemetry data is sent to a dummy endpoint: https://httpbin.org/post
- The telemetry protocol is a just dummy json, ideally, It should be using OpenTelemetry or any other telemetry framework.
- The signup processes is simulated by sending always the same image to the Virtual Orb server
- To be production ready, there are many missing things to consider, like:
  - The signup processes should be asynchronous, allowing for the request to queue to avoid overloading the server.
  - Encoding the image as a base64 string in a json is very inefficient, it should be sent using a binary probably to a dedicated storage system like s3.
  - Add Oauth support
  - Authorization
  - Handle network errors, retries, rate-liming, circuit breaking and backpressure, ...
  - Monitoring, alerting and scaling policies to guaranty a good SLO
  - The Client is only logging to stdout, but it should be sending notifications to a centralized logging system or Slack channel
  - Dockerize the application and set up the CI/CD pipeline
- It includes unit test for the core components and an end-to-end integration test `cargo test --test integration_test`
- It's using a very naive in-memory database, so it will be reset every time the server is restarted

## Build

```
cargo build
```

# Run it locally:

```
# Server
target/debug/rust-virtual-orb-poc server --orb-api-port 4242

# Client
target/debug/rust-virtual-orb-poc client                                \
            --telemetry-url https://httpbin.org/post --telemetry-interval 10 \
            --orb-server http://localhost:4242 --orb-signup-interval 2

```


### Usage:

* Client:
```
 ./rust-virtual-orb-poc client --help
Usage: rust-virtual-orb-poc client --telemetry-url <TELEMETRY_URL> --telemetry-interval <TELEMETRY_INTERVAL> --orb-server <ORB_SERVER> --orb-signup-interval <ORB_SIGNUP_INTERVAL>

Options:
      --telemetry-url <TELEMETRY_URL>              
      --telemetry-interval <TELEMETRY_INTERVAL>    Telemetry reporting interval in seconds
      --orb-server <ORB_SERVER>                    Orb lib api url, example http://localhost:4242
      --orb-signup-interval <ORB_SIGNUP_INTERVAL>  Period of the Orb signup test interval in seconds
      -h, --help                                   Print help
```

* Server:

```
./rust-virtual-orb-poc server --help
Usage: rust-virtual-orb-poc server --orb-api-port <ORB_API_PORT>

Options:
      --orb-api-port <ORB_API_PORT>  
      -h, --help                         Print help 
```

Once an image has been used in the signup process, the server will return 409 Conflict,
to facilitate testing you can reset the server with:
```
curl -X POST localhost:4242/api/reset
```
