# license-api-rs

License API connector written in Rust

## Installation

```bash
cargo add license-api
```

## Code example

```rust
use inquire::{Password, Text};
use license_api::auth::hwid::get_hwid;
use license_api::auth::models::LoginRequest;
use license_api::auth::traits::Authenticator;
use license_api::auth::authenticator::BasicAuthenticator;

#[tokio::main]
async fn main() {
    let authenticator = BasicAuthenticator::new("http://localhost:8080");

    let hwid = get_hwid().await;

    let username = Text::new("enter your username").prompt();

    let password = Password::new("enter your password")
        .without_confirmation()
        .prompt();

    let creds = LoginRequest {
        username: username.unwrap(),
        password: password.unwrap(),
    };

    match authenticator.login(&creds, &hwid).await {
        Ok(_) => {
            println!("successfully logged in!");
        },
        Err(_) => {
            eprintln!("failed to login");
        }
    };
}
```
