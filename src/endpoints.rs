use crate::*;
use crate::types::*;

pub async fn get_system(client: &PkClient, system_id: &str) -> Result<System, Box<dyn std::error::Error>> {
    let req = "systems/".to_string() + system_id;
    client.get(req.as_str()).await
}

pub async fn get_system_members(client: &PkClient, system_id: &str) -> Result<Vec<Member>, Box<dyn std::error::Error>> {
    let req = "systems/".to_string() + system_id + "/members";
    client.get(req.as_str()).await
}

pub async fn get_system_groups(client: &PkClient, system_id: &str) -> Result<Vec<Group>, Box<dyn std::error::Error>> {
    let req = "systems/".to_string() + system_id + "/groups";
    client.get(req.as_str()).await
}

pub async fn get_system_switches(client: &PkClient, system_id: &str, before: &str, limit: &i32) -> Result<Vec<Switch>, Box<dyn std::error::Error>> {
    let req = "systems/".to_string() + system_id + "/switches";
    // todo: handle before and limit
    client.get(req.as_str()).await
}

pub async fn get_member(client: &PkClient, member_id: &str) -> Result<Member, Box<dyn std::error::Error>> {
    let req = "members/".to_string() + member_id;
    client.get(req.as_str()).await
}

pub async fn get_member_groups(client: &PkClient, member_id: &str) -> Result<Vec<Group>, Box<dyn std::error::Error>> {
    let req = "members/".to_string() + member_id + "/groups";
    client.get(req.as_str()).await
}

pub async fn get_member_guild_settings(client: &PkClient, member_id: &str, guild_id: &str) -> Result<Vec<MemberGuildSettings>, Box<dyn std::error::Error>> {
    let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
    client.get(req.as_str()).await
}

pub async fn get_message(client: &PkClient, id: &str) -> Result<Message, Box<dyn std::error::Error>> {
    let req = "messages/".to_string() + id;
    client.get(req.as_str()).await
}

pub async fn get_group(client: &PkClient, group_id: &str) -> Result<Group, Box<dyn std::error::Error>> {
    let req = "groups/".to_string() + group_id;
    client.get(req.as_str()).await
}

pub async fn get_group_members(client: &PkClient, group_id: &str) -> Result<Vec<Member>, Box<dyn std::error::Error>> {
    let req = "groups/".to_string() + group_id + "/members";
    client.get(req.as_str()).await
}
