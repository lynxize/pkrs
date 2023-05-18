use std::error::Error;

use clap::ValueEnum;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::api::endpoints::*;

pub struct PkClient {
    pub(crate) client: Client,
    pub(crate) token: String,
    pub(crate) user_agent: String,
}

impl PkClient {
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a>,
    {
        let res = self
            .client
            .get(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<T>()
            .await?;
        //println!("{:#?}", res);
        Ok(res)
    }

    pub async fn patch<T>(&self, endpoint: &str, body: &T) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a>,
            T: Serialize,
    {
        let res = self
            .client
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
            T: Serialize,
    {
        let res = self
            .client
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

    pub async fn delete(&self, endpoint: &str) -> Result<Response, Box<dyn Error>> {
        let res = self
            .client
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
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub avatar_url: Option<String>,
    pub created: Option<String>,
    pub privacy: Option<SystemPrivacy>,
}

impl System {
    pub async fn update(&self, client: &PkClient) -> Result<System, Box<dyn Error>> {
        update_system(client, &self).await
    }

    pub async fn get_settings(&self, client: &PkClient) -> Result<SystemSettings, Box<dyn Error>> {
        get_system_settings(client, self.id.as_str()).await
    }

    pub async fn update_settings(
        &self,
        client: &PkClient,
        settings: &SystemSettings,
    ) -> Result<SystemSettings, Box<dyn Error>> {
        update_system_settings(client, self.id.as_str(), settings).await
    }

    pub async fn get_guild_settings(
        &self,
        client: &PkClient,
        guild_id: &str,
    ) -> Result<SystemGuildSettings, Box<dyn Error>> {
        get_system_guild_settings(client, self.id.as_str(), guild_id).await
    }

    pub async fn update_guild_settings(
        &self,
        client: &PkClient,
        guild_id: &str,
        settings: &SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Box<dyn Error>> {
        update_system_guild_settings(client, self.id.as_str(), guild_id, settings).await
    }

    pub async fn get_autoproxy_settings(
        &self,
        client: &PkClient,
    ) -> Result<AutoProxySettings, Box<dyn Error>> {
        get_system_autoproxy_settings(client, self.id.as_str()).await
    }

    pub async fn update_autoproxy_settings(
        &self,
        client: &PkClient,
        settings: &AutoProxySettings,
    ) -> Result<AutoProxySettings, Box<dyn Error>> {
        update_system_autoproxy_settings(client, self.id.as_str(), settings).await
    }

    pub async fn get_members(&self, client: &PkClient) -> Result<Vec<Member>, Box<dyn Error>> {
        get_system_members(client, self.id.as_str()).await
    }

    pub async fn get_groups(&self, client: &PkClient) -> Result<Vec<Group>, Box<dyn Error>> {
        get_system_groups(client, self.id.as_str()).await
    }

    pub async fn get_switches(
        &self,
        client: &PkClient,
        before: &str,
        limit: &i32,
    ) -> Result<Vec<Switch>, Box<dyn Error>> {
        get_system_switches(client, self.id.as_str(), before, limit).await
    }

    pub async fn get_fronters(&self, client: &PkClient) -> Result<Switch, Box<dyn Error>> {
        get_system_fronters(client, self.id.as_str()).await
    }

    pub async fn create_switch(
        &self,
        client: &PkClient,
        member_ids: &Vec<&str>,
        time: String,
    ) -> Result<(), Box<dyn Error>> {
        create_switch(client, self.id.as_str(), member_ids, time).await
    }
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
    pub created: Option<String>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub autoproxy_enabled: Option<bool>,
    pub message_count: Option<i32>,
    pub last_message_timestamp: Option<String>,
    pub privacy: Option<MemberPrivacy>,
}

impl Member {
    pub async fn create(&self, client: &PkClient) -> Result<Member, Box<dyn Error>> {
        create_member(client, &self).await
    }

    pub async fn update(&self, client: &PkClient) -> Result<Member, Box<dyn Error>> {
        update_member(client, &self).await
    }

    pub async fn delete(&self, client: &PkClient) -> Result<Response, Box<dyn Error>> {
        delete_member(client, self.id.as_str()).await
    }

    pub async fn get_groups(&self, client: &PkClient) -> Result<Vec<Group>, Box<dyn Error>> {
        get_member_groups(client, &self.id.as_str()).await
    }

    pub async fn add_groups(
        &self,
        client: &PkClient,
        group_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        add_member_groups(client, self.id.as_str(), group_ids).await
    }

    pub async fn remove_groups(
        &self,
        client: &PkClient,
        group_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        remove_member_groups(client, self.id.as_str(), group_ids).await
    }

    pub async fn overwrite_groups(
        &self,
        client: &PkClient,
        group_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        overwrite_member_groups(client, self.id.as_str(), group_ids).await
    }

    pub async fn get_guild_settings(
        &self,
        client: &PkClient,
        guild_id: &str,
    ) -> Result<Vec<MemberGuildSettings>, Box<dyn Error>> {
        get_member_guild_settings(client, self.id.as_str(), guild_id).await
    }

    pub async fn update_guild_settings(
        &self,
        client: &PkClient,
        guild_id: &str,
        settings: &MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Box<dyn Error>> {
        update_member_guild_settings(client, self.id.as_str(), guild_id, settings).await
    }
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

impl Group {
    pub async fn create(&self, client: &PkClient) -> Result<Group, Box<dyn Error>> {
        create_group(client, &self).await
    }

    pub async fn update(&self, client: &PkClient) -> Result<Group, Box<dyn Error>> {
        update_group(client, &self).await
    }

    pub async fn delete(&self, client: &PkClient) -> Result<Response, Box<dyn Error>> {
        delete_group(client, self.id.as_str()).await
    }

    pub async fn get_members(&self, client: &PkClient) -> Result<Vec<Member>, Box<dyn Error>> {
        get_group_members(client, self.id.as_str()).await
    }

    pub async fn add_members(
        &self,
        client: &PkClient,
        member_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        add_group_members(client, self.id.as_str(), member_ids).await
    }

    pub async fn remove_members(
        &self,
        client: &PkClient,
        member_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        remove_group_members(client, self.id.as_str(), member_ids).await
    }

    pub async fn overwrite_members(
        &self,
        client: &PkClient,
        member_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        overwrite_group_members(client, self.id.as_str(), member_ids).await
    }
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
    pub time: String,
    pub members: Vec<SwitchMember>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SwitchMember {
    Full(Box<Member>),
    Id(String),
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub timestamp: String,
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
    pub last_latch_timestamp: Option<String>,
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