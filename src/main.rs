use color_eyre::Result;
use purchase_api::server::create_server;

#[tokio::main]
pub async fn main() -> Result<()> {
    color_eyre::install()?;

    create_server().await
}