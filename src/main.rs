use crate::initializers::setup_tcp;
mod initializers;
#[tokio::main]async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Call the setup_and_listen function from the tcp_setup submodule
   setup_tcp::setup_and_listen().await?;
    Ok(())
}