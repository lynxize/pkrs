use std::error::Error;

mod api;
mod types;
mod command_handler;
mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    command_handler::handle_commands().await
}
