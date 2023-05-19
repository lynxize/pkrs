use std::error::Error;
use std::fs;
use clap::Parser;

use reqwest::Client;

use crate::api::*;
use crate::types::*;
use crate::commands::*;

pub(crate) async fn handle_commands() -> Result<(), Box<dyn Error>>{
    let mut client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("../token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    let cli = Cli::parse();

    let mut default_sys = get_system(&client, "txipz").await?;

    match cli.command {
        Some(Commands::Token { token }) => {
           client.token = token;
        },
        Some(Commands::System { command }) => {
            match command {
                SystemCommands::Get{ system_id } => {
                    let system = get_system(&client, system_id.unwrap_or(String::from(default_sys.id)).as_str()).await?;
                    println!("System: {:?}", system);
                },

                SystemCommands::Set { command } => {
                    handle_system_set(&mut client, command, &mut default_sys).await?;
                },
            }
        },
        Some(Commands::Member { command }) => {
            match command {
                MemberCommands::Get { member_id } => {
                    let member = get_member(&client, member_id.as_str()).await?;
                    println!("Member: {:?}", member);
                },
                _ => {
                    println!("Command not yet implemented!");
                }
            }
        }
        _ => {
            println!("Unknown command!");
        }
    }

    Ok(())
}

async fn handle_system_set(client: &mut PkClient, command: SystemSetCommands, mut sys: &mut System) -> Result<(), Box<dyn Error>> {
    match command {
        SystemSetCommands::Name { name } => {
            sys.name = Some(name.clone());
            sys.update(client).await?;
            println!("Set name to {}", &name);
        },
        SystemSetCommands::Description { description } => {
            sys.description = Some(description.clone());
            sys.update(client).await?;
            println!("Set description to {}", &description);
        },
        SystemSetCommands::Tag { tag } => {
            sys.tag = Some(tag.clone());
            sys.update(client).await?;
            println!("Set tag to {}", &tag);
        }
        _ => {
            println!("Command not yet implemented!");
        }
    }
    Ok(())
}