use crate::initializers::setup_tcp;
mod initializers;
mod security;
#[tokio::main]async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
   setup_tcp::setup_and_listen().await?;
    Ok(())
    
}