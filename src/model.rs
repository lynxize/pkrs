use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_either::StringOrStruct;
use time::OffsetDateTime;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkId(pub String);

impl Display for PkId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct System {
    pub id: PkId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub avatar_url: Option<Url>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<OffsetDateTime>,
    pub privacy: Option<SystemPrivacy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemPrivacy {
    pub description_privacy: Privacy,
    pub pronoun_privacy: Privacy,
    pub member_list_privacy: Privacy,
    pub group_list_privacy: Privacy,
    pub front_privacy: Privacy,
    pub front_history_privacy: Privacy,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    pub id: PkId,
    pub uuid: Uuid,
    pub name: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub birthday: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<Url>,
    pub webhook_avatar_url: Option<Url>,
    pub banner: Option<Url>,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created: Option<OffsetDateTime>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub autoproxy_enabled: Option<bool>,
    pub message_count: Option<i32>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_message_timestamp: Option<OffsetDateTime>,
    pub privacy: Option<MemberPrivacy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberPrivacy {
    pub visibility: Privacy,
    pub name_privacy: Privacy,
    pub description_privacy: Privacy,
    pub birthday_privacy: Privacy,
    pub pronoun_privacy: Privacy,
    pub avatar_privacy: Privacy,
    pub metadata_privacy: Privacy,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    pub id: PkId,
    pub uuid: Uuid,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<Url>,
    pub banner: Option<Url>,
    pub color: Option<String>,
    pub privacy: Option<GroupPrivacy>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupPrivacy {
    pub name_privacy: Privacy,
    pub description_privacy: Privacy,
    pub icon_privacy: Privacy,
    pub list_privacy: Privacy,
    pub metadata_privacy: Privacy,
    pub visibility: Privacy,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Switch {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub members: Vec<StringOrStruct<Member>>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub id: String,
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
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_latch_timestamp: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AutoProxyMode {
    Off,
    Front,
    Latch,
    Member,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Privacy {
    Public,
    Private,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberGuildSettings {
    pub guild_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<Url>,
}
