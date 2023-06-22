use std::error::Error;

use reqwest::StatusCode;

use crate::api::client::PkClient;
use crate::api::types::*;
use crate::command::def::*;
use crate::util::get_input;

pub async fn handle_member(
    client: &mut PkClient,
    command: MemberCommands,
) -> Result<(), Box<dyn Error>> {
    match command {
        MemberCommands::Get { member_id } => {
            let member = client.get_member(member_id.as_str()).await?;
            println!("Member: {:?}", member);
        }
        MemberCommands::Create {
            name,
            avatar,
            description,
        } => {
            // todo: cleanup
            let n = name.unwrap_or_else(|| get_input("Name: "));
            let m = client.create_member(
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
                }
            )
                .await?;
            println!("Created Member: {}", &m.name)
        }
        MemberCommands::Delete { member_id, confirm } => {
            let c = confirm.unwrap_or_else(|| get_input("Confirm (y/n): ").contains("y"));
            if c {
                let res = client.delete_member(member_id.as_str()).await?;

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

    Ok(())
}