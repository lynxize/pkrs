use std::error::Error;
use std::fs;

use clap::builder::Str;
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.pluralkit.me/v2/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("A little PluralKit Nonsense");
    let client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    println!("Test Request");
    let sys = get_system(&client, "txipz").await?;
    let members = get_system_members(&client, &sys.id).await?;

    println!("got {:#?}", members);

    Ok(())
}

pub struct PkClient {
    client: Client,
    token: String,
    user_agent: String,
}

impl PkClient {
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a>,
    {
        let res = self.client
            .get(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }

    pub async fn patch<T>(&self, endpoint: &str, body: &T) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a>,
            T: Serialize
    {
        let res = self.client
            .patch(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }

    pub async fn post<T>(&self, endpoint: &str, body: &T) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a>,
            T: Serialize
    {
        let res = self.client
            .post(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }

    pub async fn delete(&self, endpoint: &str) -> Result<Response, Box<dyn Error>>
    {
        let res = self.client
            .delete(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await?;

        Ok(res)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct System {
    id: String,
    name: Option<String>,
    description: Option<String>,
    tag: Option<String>,
    avatar_url: Option<String>,
    created: Option<String>,
    privacy: Option<SystemPrivacy>,
}

impl System {
    pub async fn get_members(
        &self,
        client: &PkClient,
    ) -> Result<Vec<Member>, Box<dyn Error>> {
        get_system_members(client, self.id.as_str()).await
    }

    pub async fn get_groups(
        &self,
        client: &PkClient,
    ) -> Result<Vec<Group>, Box<dyn Error>> {
        get_system_groups(client, self.id.as_str()).await
    }

    pub async fn get_settings(
        &self,
        client: &PkClient,
    ) -> Result<SystemSettings, Box<dyn Error>> {
        get_system_settings(client, self.id.as_str()).await
    }

    pub async fn get_guild_settings(
        &self,
        client: &PkClient,
        guild_id: &str,
    ) -> Result<SystemGuildSettings, Box<dyn Error>> {
        get_system_guild_settings(client, self.id.as_str(), guild_id).await
    }

    pub async fn get_autoproxy_settings(
        &self,
        client: &PkClient,
    ) -> Result<AutoProxySettings, Box<dyn Error>> {
        get_system_autoproxy_settings(client, self.id.as_str()).await
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemPrivacy {
    description_privacy: String,
    pronoun_privacy: String,
    member_list_privacy: String,
    group_list_privacy: String,
    front_privacy: String,
    front_history_privacy: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    id: String,
    uuid: String,
    name: String,
    display_name: Option<String>,
    color: Option<String>,
    birthday: Option<String>,
    pronouns: Option<String>,
    avatar_url: Option<String>,
    webhook_avatar_url: Option<String>,
    banner: Option<String>,
    description: Option<String>,
    created: Option<String>,
    proxy_tags: Vec<ProxyTag>,
    keep_proxy: bool,
    autoproxy_enabled: Option<bool>,
    message_count: Option<i32>,
    last_message_timestamp: Option<String>,
    privacy: Option<MemberPrivacy>,
}

impl Member {
    pub async fn get_guild_settings(
        &self,
        client: &PkClient,
        guild_id: &str,
    ) -> Result<Vec<MemberGuildSettings>, Box<dyn Error>> {
        get_member_guild_settings(client, self.id.as_str(), guild_id).await
    }

    pub async fn get_groups(&self, client: &PkClient) -> Result<Vec<Group>, Box<dyn Error>> {
        get_member_groups(client, self.id.as_str()).await
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberPrivacy {
    visibility: String,
    name_privacy: String,
    description_privacy: String,
    birthday_privacy: String,
    pronoun_privacy: String,
    avatar_privacy: String,
    metadata_privacy: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProxyTag {
    prefix: Option<String>,
    suffix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    id: String,
    uuid: String,
    name: String,
    display_name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    banner: Option<String>,
    color: Option<String>,
    privacy: Option<GroupPrivacy>,
}

impl Group {
    pub async fn get_members(
        &self,
        client: &PkClient,
    ) -> Result<Vec<Member>, Box<dyn Error>> {
        get_group_members(client, self.id.as_str()).await
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupPrivacy {
    name_privacy: String,
    description_privacy: String,
    icon_privacy: String,
    list_privacy: String,
    metadata_privacy: String,
    visibility: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Switch {
    id: String,
    time: String,
    members: Vec<SwitchMember>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SwitchMember {
    Full(Box<Member>),
    Id(String),
}

#[derive(Deserialize, Debug)]
pub struct Message {
    timestamp: String,
    original: String,
    sender: String,
    channel: String,
    guild: String,
    system: Option<System>,
    member: Option<Member>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemSettings {
    timezone: String,
    pings_enabled: bool,
    latch_timeout: Option<i32>,
    member_default_private: bool,
    group_default_private: bool,
    show_private_info: bool,
    member_limit: i32,
    group_limit: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemGuildSettings {
    guild_id: Option<String>,
    proxying_enabled: bool,
    tag: Option<String>,
    tag_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AutoProxySettings {
    autoproxy_mode: AutoProxyMode,
    autoproxy_member: Option<String>,
    last_latch_timestamp: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum AutoProxyMode {
    Off,
    Front,
    Latch,
    Member,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberGuildSettings {
    guild_id: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
}

pub async fn get_system(
    client: &PkClient,
    system_id: &str,
) -> Result<System, Box<dyn Error>> {
    let req = "systems/".to_string() + system_id;
    client.get(req.as_str()).await
}

pub async fn update_system(
    client: &PkClient,
    system: &System,
) -> Result<System, Box<dyn Error>> {
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
    // todo: handle query params
    let req = "systems/".to_string() + system_id + "/autoproxy";
    client.get(req.as_str()).await
}

pub async fn update_system_autoproxy_settings(
    client: &PkClient,
    system_id: &str,
    settings: &AutoProxySettings,
) -> Result<AutoProxySettings, Box<dyn Error>> {
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

pub async fn create_member(
    client: &PkClient,
    member: &Member,
) -> Result<Member, Box<dyn Error>> {
    client.post("members", member).await
}

pub async fn get_member(
    client: &PkClient,
    member_id: &str,
) -> Result<Member, Box<dyn Error>> {
    let req = "members/".to_string() + member_id;
    client.get(req.as_str()).await
}

pub async fn update_member(
    client: &PkClient,
    member: &Member,
) -> Result<Member, Box<dyn Error>> {
    let req = "members/".to_string() + &*member.id;
    client.patch(req.as_str(), member).await
}

pub async fn delete_member(
    client: &PkClient,
    member_id: &str,
) -> Result<Response, Box<dyn Error>> {
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
    group_id: &str,
) -> Result<(), Box<dyn Error>> {
    let req = "members/".to_string() + member_id + "/groups/add";
    todo!()
}

pub async fn remove_member_groups(
    client: &PkClient,
    member_id: &str,
    group_id: &str,
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

pub async fn create_group(
    client: &PkClient,
    group: &Group,
) -> Result<Group, Box<dyn Error>> {
    client.post("groups", group).await
}

pub async fn get_group(
    client: &PkClient,
    group_id: &str,
) -> Result<Group, Box<dyn Error>> {
    let req = "groups/".to_string() + group_id;
    client.get(req.as_str()).await
}

pub async fn update_group(
    client: &PkClient,
    group: &Group,
) -> Result<Group, Box<dyn Error>> {
    let req = "groups/".to_string() + &*group.id;
    client.patch(req.as_str(), group).await
}

pub async fn delete_group(
    client: &PkClient,
    group_id: &str,
) -> Result<Response, Box<dyn Error>> {
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
    member_id: &str,
) -> Result<(), Box<dyn Error>> {
    let req = "groups/".to_string() + group_id + "/members/add";
    todo!()
}

pub async fn remove_group_members(
    client: &PkClient,
    group_id: &str,
    member_id: &str,
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
    time: String
) {
    #[derive(Serialize, Deserialize, Debug)]
    struct JsonSwitch {
        timestamp: String,
        members: Vec<String>
    }

    let req = "systems/".to_string() + system_id + "/switches";
    client.post::<JsonSwitch>(req.as_str(), &JsonSwitch {
        timestamp: time,
        members: member_ids.iter().map(|&s|s.into()).collect()
    });
}

pub async fn get_message(
    client: &PkClient,
    id: &str,
) -> Result<Message, Box<dyn Error>> {
    let req = "messages/".to_string() + id;
    client.get(req.as_str()).await
}