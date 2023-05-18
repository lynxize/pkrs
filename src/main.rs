use std::error::Error;

mod api;
mod command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    command::handler::handle_commands().await;
    Ok(())
}
