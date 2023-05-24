use std::error::Error;
use std::fs;

use clap::Parser;
use reqwest::Client;
use time::{OffsetDateTime, Time, UtcOffset};

use crate::api::client::PkClient;
use crate::api::types::System;
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

    let default_sys = client.get_system("txipz").await?;

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
        Some(Commands::Switch { command }) => {
            handle_switch(&mut client, &default_sys, command).await?;
        }
        _ => {
            println!("Command not implemented!");
        }
    }

    Ok(())
}


async fn handle_switch(client: &mut PkClient, sys: &System, command: SwitchCommands) -> Result<(), Box<dyn Error>> {
    match command {
        SwitchCommands::List { .. } => {
            let time = OffsetDateTime::now_utc();
            // todo: remove cursed replace_year duct tape
            let switches = client.get_system_switches(sys.id.as_str(), &time.replace_year(2020).unwrap(), &10).await?;
            println!("Switches: {:?}", switches);
        }
        SwitchCommands::New { members, time } => {
            //client.create_switch(sys.id.as_str(), &strify_vec(&members)).await?;
        }
        _ => {
            println!("Command not yet implemented!");
        }
    }
    Ok(())
}