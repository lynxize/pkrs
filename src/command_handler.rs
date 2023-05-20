use clap::Parser;
use std::error::Error;
use std::fs;

use reqwest::{Client, StatusCode};

use crate::api::*;
use crate::commands::*;
use crate::types::*;
use crate::util::get_input;

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
        Some(Commands::System { command }) => match command {
            SystemCommands::Get { system_id } => {
                let system = get_system(
                    &client,
                    system_id.unwrap_or(String::from(default_sys.id)).as_str(),
                )
                .await?;
                println!("System: {:?}", system);
            }

            SystemCommands::Set { command } => {
                handle_system_set(&mut client, command, &mut default_sys).await?;
            }
        },
        Some(Commands::Member { command }) => {
            match command {
                MemberCommands::Get { member_id } => {
                    let member = get_member(&client, member_id.as_str()).await?;
                    println!("Member: {:?}", member);
                }
                MemberCommands::Create {
                    name,
                    avatar,
                    description,
                } => {
                    // todo: cleanup
                    let n = name.unwrap_or_else(|| get_input("Name: "));
                    let m = create_member(
                        &client,
                        &Member {
                            id: "".to_string(),
                            uuid: "".to_string(),
                            name: n,
                            display_name: None,
                            color: None,
                            birthday: None,
                            pronouns: None,
                            avatar_url: avatar,
                            webhook_avatar_url: None,
                            banner: None,
                            description,
                            created: None,
                            proxy_tags: vec![],
                            keep_proxy: false,
                            autoproxy_enabled: None,
                            message_count: None,
                            last_message_timestamp: None,
                            privacy: None,
                        },
                    )
                    .await?;
                    println!("Created Member: {}", &m.name)
                },
                MemberCommands::Delete { member_id, confirm } => {
                    let c = confirm.unwrap_or_else(|| get_input("Confirm (y/n): ").contains("y"));
                    if c {
                        let res = delete_member(&client, member_id.as_str()).await?;

                        if res.status() == StatusCode::NO_CONTENT {
                            println!("Deleted member {}", member_id);
                        } else {
                            println!("Failed to delete member {}", member_id);
                        }
                    } else {
                        println!("Aborted deletion!");
                    }
                }
                _ => {
                    println!("Command not yet implemented!");
                }
            }
        },
        _ => {
            println!("Unknown command!");
        }
    }

    Ok(())
}

async fn handle_system_set(
    client: &mut PkClient,
    command: SystemSetCommands,
    mut sys: &mut System,
) -> Result<(), Box<dyn Error>> {
    match command {
        SystemSetCommands::Name { name } => {
            sys.name = Some(name.clone());
            sys.update(client).await?;
            println!("Set name to {}", &name);
        }
        SystemSetCommands::Description { description } => {
            sys.description = Some(description.clone());
            sys.update(client).await?;
            println!("Set description to {}", &description);
        }
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
