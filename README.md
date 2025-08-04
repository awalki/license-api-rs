# license-api-rs

License API client library made with Rust

## Installation

```bash
cargo add license-api
```

## Code example

```rust
use inquire::{Password, Text};
use license_api::auth::LoginRequest;
use license_api::hwid::get_hwid;
use license_api::auth::LicenseAPI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = LicenseAPI::new("http://localhost:8080");

    let hwid = get_hwid(true, "anything");
    let key = String::from("your-license-key");

    let creds = LoginRequest {
        key,
        hwid,
    };

    if let Ok(true) = api.login(&creds).await {
        println!("✔ Successfully logged in and HWID linked!");
    } else {
        println!("❌ Failed to login");
    }

    Ok(())
}
```
