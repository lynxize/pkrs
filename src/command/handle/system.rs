use std::error::Error;

use crate::api::client::PkClient;
use crate::api::types::System;
use crate::command::def::system::*;

pub async fn handle_system(
    client: &mut PkClient,
    command: SystemCommands,
    default_sys: System,
) -> Result<(), Box<dyn Error>> {
    match command {
        SystemCommands::Get { system_id } => {
            let system = client.get_system(
                system_id.unwrap_or(default_sys.id).as_str(),
            )
                .await?;
            println!("System: {:?}", system);
        }

        SystemCommands::Set { command } => {
            handle_system_set(client, command, default_sys).await?;
        }
    }
    Ok(())
}

async fn handle_system_set(
    client: &mut PkClient,
    command: SystemSetCommands,
    mut sys: System,
) -> Result<(), Box<dyn Error>> {
    match command {
        SystemSetCommands::Name { name } => {
            sys.name = Some(name.clone());
            client.update_system(&sys).await?;
            if sys.name == Some(name.clone()) {
                println!("Set name to {}", &name);
            } else {
                println!("Failed to set name");
            }
        }
        SystemSetCommands::Tag { tag } => {
            sys.tag = Some(tag.clone());
            client.update_system(&sys).await?;
            if sys.tag == Some(tag.clone()) {
                println!("Set tag to {}", &tag);
            } else {
                println!("Failed to set tag");
            }
        }
        SystemSetCommands::Description { description } => {
            sys.description = Some(description.clone());
            client.update_system(&sys).await?;
            if sys.description == Some(description.clone()) {
                println!("Set description to {}", &description);
            } else {
                println!("Failed to set description");
            }
        }
        _ => {
            println!("Command not yet implemented!");
        }
    }
    Ok(())
}
