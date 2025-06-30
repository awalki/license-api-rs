# license-api-rs

License API connector written in Rust

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
    let api = LicenseAPI::new("http://localhost:8080");

    let hwid = get_hwid().await;

    let username = Text::new("Enter your username:")
        .prompt()
        .expect("Failed to read username");
    let password = Password::new("Enter your password:")
        .without_confirmation()
        .prompt()
        .expect("Failed to read password");

    let creds = LoginRequest {
        username,
        password,
        hwid,
    };

    match api.login(&creds).await {
        Ok(true) => println!("✔ Successfully logged in and HWID linked!"),
        Ok(false) => eprintln!("⚠ Login succeeded but HWID linking returned failure status."),
        Err(err) => eprintln!("❌ Login failed: {}", err),
    }

    Ok(())
}
```
