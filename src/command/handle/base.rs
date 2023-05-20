use std::error::Error;
use std::fs;

use clap::Parser;
use reqwest::{Client};

use crate::api::endpoints::get_system;
use crate::api::types::PkClient;
use crate::command::def::base::*;
use crate::command::handle::member::handle_member;
use crate::command::handle::system::*;

pub(crate) async fn handle_commands() -> Result<(), Box<dyn Error>> {
    let mut client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    let cli = Cli::parse();

    let mut default_sys = get_system(&client, "txipz").await?;

    match cli.command {
        Some(Commands::Token { token }) => {
            client.token = token;
        }
        Some(Commands::System { command }) => {
            handle_system(&mut client, command, default_sys).await?;
        }
        Some(Commands::Member { command }) => {
            handle_member(&mut client, command).await?;
        }
        _ => {
            println!("Unknown command!");
        }
    }

    Ok(())
}
