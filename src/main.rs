use std::error::Error;

mod api;
mod command_handler;
mod commands;
mod types;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    command_handler::handle_commands().await
}
