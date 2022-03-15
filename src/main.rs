use std::env;

use aws_sdk_secretsmanager::Client;
use eyre::{eyre, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);

    for (key, value) in env::vars() {
        if key.ends_with("_SECRET_ARN") {
            let secret_value = client.get_secret_value().secret_id(&value).send().await?;
            let secret_string = secret_value
                .secret_string()
                .ok_or_else(|| eyre!("could not read SecretString"))?;

            println!(
                "export {}='{}'",
                key.trim_end_matches("_SECRET_ARN"),
                secret_string,
            );
        }
    }

    Ok(())
}
