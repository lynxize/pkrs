use std::error::Error;
use serde::Deserialize;
use crate::PkClient;

#[derive(Deserialize, Debug)]
pub struct System {
    id: String,
    name: Option<String>,
    description: Option<String>,
    tag: Option<String>,
    avatar_url: Option<String>,
    created: Option<String>,
    privacy: Option<SystemPrivacy>,
}

#[derive(Deserialize, Debug)]
pub struct SystemPrivacy {
    description_privacy: String,
    pronoun_privacy: String,
    member_list_privacy: String,
    group_list_privacy: String,
    front_privacy: String,
    front_history_privacy: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct MemberPrivacy {
    visibility: String,
    name_privacy: String,
    description_privacy: String,
    birthday_privacy: String,
    pronoun_privacy: String,
    avatar_privacy: String,
    metadata_privacy: String,
}

#[derive(Deserialize, Debug)]
pub struct ProxyTag {
    prefix: Option<String>,
    suffix: Option<String>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct GroupPrivacy {
    name_privacy: String,
    description_privacy: String,
    icon_privacy: String,
    list_privacy: String,
    metadata_privacy: String,
    visibility: String,
}

#[derive(Deserialize, Debug)]
pub struct Switch {
    id: String,
    time: String,
    members: Vec<SwitchMember>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct SystemGuildSettings {
    guild_id: Option<String>,
    proxying_enabled: bool,
    tag: Option<String>,
    tag_enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct AutoProxySettings {
    autoproxy_mode: AutoProxyMode,
    autoproxy_member: Option<String>,
    last_latch_timestamp: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum AutoProxyMode {
    Off,
    Front,
    Latch,
    Member,
}

#[derive(Deserialize, Debug)]
pub struct MemberGuildSettings {
    guild_id: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
}