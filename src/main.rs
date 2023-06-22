use std::error::Error;

use crate::command::base::handle_commands;

mod util;
mod command;
mod api;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    handle_commands().await
}
