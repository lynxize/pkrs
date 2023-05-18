use std::error::Error;

use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};

use crate::api::types::*;

pub const BASE_URL: &str = "https://api.pluralkit.me/v2/";

pub async fn get_system(client: &PkClient, system_id: &str) -> Result<System, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id;
    client.get(req.as_str()).await
}

pub async fn update_system(client: &PkClient, system: &System) -> Result<System, Box<dyn Error>> {
    let req = "systems/".to_string() + &*system.id;
    client.patch(req.as_str(), system).await
}

pub async fn get_system_settings(
    client: &PkClient,
    system_id: &str,
) -> Result<SystemSettings, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/settings";
    client.get(req.as_str()).await
}

pub async fn update_system_settings(
    client: &PkClient,
    system_id: &str,
    settings: &SystemSettings,
) -> Result<SystemSettings, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/settings";
    client.patch(req.as_str(), settings).await
}

pub async fn get_system_guild_settings(
    client: &PkClient,
    system_id: &str,
    guild_id: &str,
) -> Result<SystemGuildSettings, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/settings/guilds/" + guild_id;
    client.get(req.as_str()).await
}

pub async fn update_system_guild_settings(
    client: &PkClient,
    system_id: &str,
    guild_id: &str,
    settings: &SystemGuildSettings,
) -> Result<SystemGuildSettings, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/settings/guilds/" + guild_id;
    client.patch(req.as_str(), settings).await
}

pub async fn get_system_autoproxy_settings(
    client: &PkClient,
    system_id: &str,
) -> Result<AutoProxySettings, Box<dyn Error>> {
    todo!(); // need to handle query string
    let req = "systems/".to_string() + system_id + "/autoproxy";
    client.get(req.as_str()).await
}

pub async fn update_system_autoproxy_settings(
    client: &PkClient,
    system_id: &str,
    settings: &AutoProxySettings,
) -> Result<AutoProxySettings, Box<dyn Error>> {
    todo!(); // need to handle query string
    let req = "systems/".to_string() + system_id + "/autoproxy";
    client.patch(req.as_str(), settings).await
}

pub async fn get_system_members(
    client: &PkClient,
    system_id: &str,
) -> Result<Vec<Member>, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/members";
    client.get(req.as_str()).await
}

pub async fn create_member(client: &PkClient, member: &Member) -> Result<Member, Box<dyn Error>> {
    client.post("members", member).await
}

pub async fn get_member(client: &PkClient, member_id: &str) -> Result<Member, Box<dyn Error>> {
    let req = "members/".to_string() + member_id;
    client.get(req.as_str()).await
}

pub async fn update_member(client: &PkClient, member: &Member) -> Result<Member, Box<dyn Error>> {
    let req = "members/".to_string() + &*member.id;
    client.patch(req.as_str(), member).await
}

pub async fn delete_member(client: &PkClient, member_id: &str) -> Result<Response, Box<dyn Error>> {
    let req = "members/".to_string() + member_id;
    client.delete(req.as_str()).await
}

pub async fn get_member_groups(
    client: &PkClient,
    member_id: &str,
) -> Result<Vec<Group>, Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/groups";
    client.get(req.as_str()).await
}

pub async fn add_member_groups(
    client: &PkClient,
    member_id: &str,
    group_id: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/groups/add";
    todo!()
}

pub async fn remove_member_groups(
    client: &PkClient,
    member_id: &str,
    group: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/groups/remove";
    todo!()
}

pub async fn overwrite_member_groups(
    client: &PkClient,
    member_id: &str,
    group_ids: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/groups/overwrite";
    todo!()
}

pub async fn get_member_guild_settings(
    client: &PkClient,
    member_id: &str,
    guild_id: &str,
) -> Result<Vec<MemberGuildSettings>, Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
    client.get(req.as_str()).await
}

pub async fn update_member_guild_settings(
    client: &PkClient,
    member_id: &str,
    guild_id: &str,
    settings: &MemberGuildSettings,
) -> Result<MemberGuildSettings, Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
    client.patch(req.as_str(), settings).await
}

pub async fn get_system_groups(
    client: &PkClient,
    system_id: &str,
) -> Result<Vec<Group>, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/groups";
    client.get(req.as_str()).await
}

pub async fn create_group(client: &PkClient, group: &Group) -> Result<Group, Box<dyn Error>> {
    client.post("groups", group).await
}

pub async fn get_group(client: &PkClient, group_id: &str) -> Result<Group, Box<dyn Error>> {
    let req = "groups/".to_string() + group_id;
    client.get(req.as_str()).await
}

pub async fn update_group(client: &PkClient, group: &Group) -> Result<Group, Box<dyn Error>> {
    let req = "groups/".to_string() + &*group.id;
    client.patch(req.as_str(), group).await
}

pub async fn delete_group(client: &PkClient, group_id: &str) -> Result<Response, Box<dyn Error>> {
    let req = "groups/".to_string() + group_id;
    client.delete(req.as_str()).await
}

pub async fn get_group_members(
    client: &PkClient,
    group_id: &str,
) -> Result<Vec<Member>, Box<dyn Error>> {
    let req = "groups/".to_string() + group_id + "/members";
    client.get(req.as_str()).await
}

pub async fn add_group_members(
    client: &PkClient,
    group_id: &str,
    members: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let req = "groups/".to_string() + group_id + "/members/add";
    todo!()
}

pub async fn remove_group_members(
    client: &PkClient,
    group_id: &str,
    members: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let req = "groups/".to_string() + group_id + "/members/remove";
    todo!()
}

pub async fn overwrite_group_members(
    client: &PkClient,
    group_id: &str,
    member_ids: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let req = "groups/".to_string() + group_id + "/members/overwrite";
    todo!()
}

pub async fn get_system_switches(
    client: &PkClient,
    system_id: &str,
    before: &str,
    limit: &i32,
) -> Result<Vec<Switch>, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/switches";
    // todo: handle before and limit
    client.get(req.as_str()).await
}

pub async fn get_system_fronters(
    client: &PkClient,
    system_id: &str,
) -> Result<Switch, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id + "/fronters";
    client.get(req.as_str()).await
}

pub async fn create_switch(
    client: &PkClient,
    system_id: &str,
    member_ids: &Vec<&str>,
    time: String,
) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize, Deserialize, Debug)]
    struct JsonSwitch {
        timestamp: String,
        members: Vec<String>,
    }

    let req = "systems/".to_string() + system_id + "/switches";
    client
        .post::<JsonSwitch>(
            req.as_str(),
            &JsonSwitch {
                timestamp: time,
                members: member_ids.iter().map(|&s| s.into()).collect(),
            },
        )
        .await?;

    Ok(())
}

pub async fn get_message(client: &PkClient, id: &str) -> Result<Message, Box<dyn Error>> {
    let req = "messages/".to_string() + id;
    client.get(req.as_str()).await
}
