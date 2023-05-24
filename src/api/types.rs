use std::error::Error;

use clap::ValueEnum;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct System {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(with="crate::util::timeser")]
    pub created: Option<OffsetDateTime>,
    pub privacy: Option<SystemPrivacy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemPrivacy {
    pub description_privacy: String,
    pub pronoun_privacy: String,
    pub member_list_privacy: String,
    pub group_list_privacy: String,
    pub front_privacy: String,
    pub front_history_privacy: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub birthday: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub webhook_avatar_url: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    #[serde(with="crate::util::timeser")]
    pub created: Option<OffsetDateTime>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub autoproxy_enabled: Option<bool>,
    pub message_count: Option<i32>,
    #[serde(with="crate::util::timeser")]
    pub last_message_timestamp: Option<OffsetDateTime>,
    pub privacy: Option<MemberPrivacy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberPrivacy {
    pub visibility: String,
    pub name_privacy: String,
    pub description_privacy: String,
    pub birthday_privacy: String,
    pub pronoun_privacy: String,
    pub avatar_privacy: String,
    pub metadata_privacy: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub privacy: Option<GroupPrivacy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupPrivacy {
    pub name_privacy: String,
    pub description_privacy: String,
    pub icon_privacy: String,
    pub list_privacy: String,
    pub metadata_privacy: String,
    pub visibility: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Switch {
    pub id: String,
    #[serde(with="time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub members: Vec<String>,
}

/* // todo: handle (and verify) the fact that it can sometimes send member objects
#[derive(Deserialize, Serialize, Debug)]
pub enum SwitchMember {
    Full(Box<Member>),
    Id(String),
}
 */

#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(with="time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub original: String,
    pub sender: String,
    pub channel: String,
    pub guild: String,
    pub system: Option<System>,
    pub member: Option<Member>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemSettings {
    pub timezone: String,
    pub pings_enabled: bool,
    pub latch_timeout: Option<i32>,
    pub member_default_private: bool,
    pub group_default_private: bool,
    pub show_private_info: bool,
    pub member_limit: i32,
    pub group_limit: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemGuildSettings {
    pub guild_id: Option<String>,
    pub proxying_enabled: bool,
    pub tag: Option<String>,
    pub tag_enabled: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AutoProxySettings {
    pub autoproxy_mode: AutoProxyMode,
    pub autoproxy_member: Option<String>,
    #[serde(with="crate::util::timeser")]
    pub last_latch_timestamp: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone, ValueEnum)]
pub enum AutoProxyMode {
    Off,
    Front,
    Latch,
    Member,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberGuildSettings {
    pub guild_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}
