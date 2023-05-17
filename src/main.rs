use clap::builder::Str;
use serde::Deserialize;
use std::fs;

const BASE_URL: &str = "https://api.pluralkit.me/v2/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("A little PluralKit Nonsense");
    let client = reqwest::Client::new();
    let token = fs::read_to_string("token").expect("No PK token found!");

    println!("Test Request");
    let res = client
        .get(BASE_URL.to_string() + "systems/txipz")
        .header("User-Agent", "Rust CLI Testing Nonsense")
        .header("Authorization", token)
        .send()
        .await?
        .json::<System>()
        .await?;

    println!("got {:#?}", res);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct System {
    id: String,
    name: Option<String>,
    description: Option<String>,
    tag: Option<String>,
    avatar_url: Option<String>,
    created: Option<String>,
    privacy: Option<SystemPrivacy>,
}

#[derive(Deserialize, Debug)]
struct SystemPrivacy {
    description_privacy: String,
    pronoun_privacy: String,
    member_list_privacy: String,
    group_list_privacy: String,
    front_privacy: String,
    front_history_privacy: String,
}

#[derive(Deserialize, Debug)]
struct Member {
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

#[derive(Deserialize, Debug)]
struct MemberPrivacy {
    visibility: String,
    name_privacy: String,
    description_privacy: String,
    birthday_privacy: String,
    pronoun_privacy: String,
    avatar_privacy: String,
    metadata_privacy: String,
}

#[derive(Deserialize, Debug)]
struct ProxyTag {
    prefix: Option<String>,
    suffix: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Group {
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

#[derive(Deserialize, Debug)]
struct GroupPrivacy {
    name_privacy: String,
    description_privacy: String,
    icon_privacy: String,
    list_privacy: String,
    metadata_privacy: String,
    visibility: String,
}

#[derive(Deserialize, Debug)]
struct Switch {
    id: String,
    time: String,
    members: Vec<SwitchMember>,
}

#[derive(Deserialize, Debug)]
enum SwitchMember {
    Full(Box<Member>),
    Id(String),
}

#[derive(Deserialize, Debug)]
struct Message {
    timestamp: String,
    original: String,
    sender: String,
    channel: String,
    guild: String,
    system: Option<System>,
    member: Option<Member>,
}

#[derive(Deserialize, Debug)]
struct SystemSettings {
    timezone: String,
    pings_enabled: bool,
    latch_timeout: Option<i32>,
    member_default_private: bool,
    group_default_private: bool,
    show_private_info: bool,
    member_limit: i32,
    group_limit: i32,
}

#[derive(Deserialize, Debug)]
struct SystemGuildSettings {
    guild_id: Option<String>,
    proxying_enabled: bool,
    tag: Option<String>,
    tag_enabled: bool,
}

#[derive(Deserialize, Debug)]
struct AutoProxySettings {
    autoproxy_mode: AutoProxyMode,
    autoproxy_member: Option<String>,
    last_latch_timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
enum AutoProxyMode {
    Off,
    Front,
    Latch,
    Member,
}

#[derive(Deserialize, Debug)]
struct MemberGuildSettings {
    guild_id: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
}
