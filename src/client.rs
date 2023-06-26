

use reqwest::{Client, Error, Response, RequestBuilder};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::types::*;

pub const BASE_URL: &str = "https://api.pluralkit.me/v2/";

pub struct PkClient {
    client: Client,
    pub token: String,
    user_agent: String,
}

impl PkClient {
    async fn get<T>(&self, endpoint: &str) -> Result<T, Error>
        where
            T: for<'a> Deserialize<'a>,
    {
        self.nonsense(self
            .client
            .get(BASE_URL.to_string() + endpoint)
        ).await
    }

    async fn patch<T>(&self, endpoint: &str, body: &T) -> Result<T, Error>
        where
            T: for<'a> Deserialize<'a>,
            T: Serialize,
    {
        self.nonsense(self
            .client
            .patch(BASE_URL.to_string() + endpoint)
            .json(body)
        ).await
    }

    async fn post<T>(&self, endpoint: &str, body: &T) -> Result<T, Error>
        where
            T: for<'a> Deserialize<'a>,
            T: Serialize,
    {
        self.nonsense(self
            .client
            .post(BASE_URL.to_string() + endpoint)
            .json(body)
        ).await
    }

    async fn nonsense<T>(&self, builder: RequestBuilder) -> Result<T, Error>
        where
            T: for<'a> Deserialize<'a>,
    {
        let req = builder
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await;

        if let Ok(res) = req {
            res.json::<T>().await
        } else {
            Err(req.unwrap_err())
        }
    }


    async fn delete(&self, endpoint: &str) -> Result<Response, Error> {
        self
            .client
            .delete(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await
    }


    pub async fn get_system(&self, system_id: &str) -> Result<System, Error> {
        let req = "systems/".to_string() + system_id;
        self.get(req.as_str()).await
    }

    pub async fn update_system(&self, system: &System) -> Result<System, Error> {
        let req = "systems/".to_string() + &*system.id;
        self.patch(req.as_str(), system).await
    }

    pub async fn get_system_settings(
        &self,
        system_id: &str,
    ) -> Result<SystemSettings, Error> {
        let req = "systems/".to_string() + system_id + "/settings";
        self.get(req.as_str()).await
    }

    pub async fn update_system_settings(
        &self,
        system_id: &str,
        settings: &SystemSettings,
    ) -> Result<SystemSettings, Error> {
        let req = "systems/".to_string() + system_id + "/settings";
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_guild_settings(
        &self,
        system_id: &str,
        guild_id: &str,
    ) -> Result<SystemGuildSettings, Error> {
        let req = "systems/".to_string() + system_id + "/settings/guilds/" + guild_id;
        self.get(req.as_str()).await
    }

    pub async fn update_system_guild_settings(
        &self,
        system_id: &str,
        guild_id: &str,
        settings: &SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Error> {
        let req = "systems/".to_string() + system_id + "/settings/guilds/" + guild_id;
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_autoproxy_settings(
        &self,
        system_id: &str,
    ) -> Result<AutoProxySettings, Error> {
        todo!(); // need to handle query string
        let req = "systems/".to_string() + system_id + "/autoproxy";
        self.get(req.as_str()).await
    }

    pub async fn update_system_autoproxy_settings(
        &self,
        system_id: &str,
        settings: &AutoProxySettings,
    ) -> Result<AutoProxySettings, Error> {
        todo!(); // need to handle query string
        let req = "systems/".to_string() + system_id + "/autoproxy";
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_members(
        &self,
        system_id: &str,
    ) -> Result<Vec<Member>, Error> {
        let req = "systems/".to_string() + system_id + "/members";
        self.get(req.as_str()).await
    }

    pub async fn create_member(&self, member: &Member) -> Result<Member, Error> {
        self.post("members", member).await
    }

    pub async fn get_member(&self, member_id: &str) -> Result<Member, Error> {
        let req = "members/".to_string() + member_id;
        self.get(req.as_str()).await
    }

    pub async fn update_member(&self, member: &Member) -> Result<Member, Error> {
        let req = "members/".to_string() + &*member.id;
        self.patch(req.as_str(), member).await
    }

    pub async fn delete_member(&self, member_id: &str) -> Result<Response, Error> {
        let req = "members/".to_string() + member_id;
        self.delete(req.as_str()).await
    }

    pub async fn get_member_groups(
        &self,
        member_id: &str,
    ) -> Result<Vec<Group>, Error> {
        let req = "members/".to_string() + member_id + "/groups";
        self.get(req.as_str()).await
    }

    pub async fn add_member_groups(
        &self,
        member_id: &str,
        group_id: &Vec<String>,
    ) -> Result<(), Error> {
        let req = "members/".to_string() + member_id + "/groups/add";
        todo!()
    }

    pub async fn remove_member_groups(
        &self,
        member_id: &str,
        group: &Vec<String>,
    ) -> Result<(), Error> {
        let req = "members/".to_string() + member_id + "/groups/remove";
        todo!()
    }

    pub async fn overwrite_member_groups(
        &self,
        member_id: &str,
        group_ids: &Vec<String>,
    ) -> Result<(), Error> {
        let req = "members/".to_string() + member_id + "/groups/overwrite";
        todo!()
    }

    pub async fn get_member_guild_settings(
        &self,
        member_id: &str,
        guild_id: &str,
    ) -> Result<Vec<MemberGuildSettings>, Error> {
        let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
        self.get(req.as_str()).await
    }

    pub async fn update_member_guild_settings(
        &self,
        member_id: &str,
        guild_id: &str,
        settings: &MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Error> {
        let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_groups(
        &self,
        system_id: &str,
    ) -> Result<Vec<Group>, Error> {
        let req = "systems/".to_string() + system_id + "/groups";
        self.get(req.as_str()).await
    }

    pub async fn create_group(&self, group: &Group) -> Result<Group, Error> {
        self.post("groups", group).await
    }

    pub async fn get_group(&self, group_id: &str) -> Result<Group, Error> {
        let req = "groups/".to_string() + group_id;
        self.get(req.as_str()).await
    }

    pub async fn update_group(&self, group: &Group) -> Result<Group, Error> {
        let req = "groups/".to_string() + &*group.id;
        self.patch(req.as_str(), group).await
    }

    pub async fn delete_group(&self, group_id: &str) -> Result<Response, Error> {
        let req = "groups/".to_string() + group_id;
        self.delete(req.as_str()).await
    }

    pub async fn get_group_members(
        &self,
        group_id: &str,
    ) -> Result<Vec<Member>, Error> {
        let req = "groups/".to_string() + group_id + "/members";
        self.get(req.as_str()).await
    }

    pub async fn add_group_members(
        &self,
        group_id: &str,
        members: &Vec<String>,
    ) -> Result<(), Error> {
        let req = "groups/".to_string() + group_id + "/members/add";
        todo!()
    }

    pub async fn remove_group_members(
        &self,
        group_id: &str,
        members: &Vec<String>,
    ) -> Result<(), Error> {
        let req = "groups/".to_string() + group_id + "/members/remove";
        todo!()
    }

    pub async fn overwrite_group_members(
        &self,
        group_id: &str,
        member_ids: &Vec<String>,
    ) -> Result<(), Error> {
        let req = "groups/".to_string() + group_id + "/members/overwrite";
        todo!()
    }

    pub async fn get_system_switches(
        &self,
        system_id: &str,
        before: &OffsetDateTime,
        limit: &i32,
    ) -> Result<Vec<Switch>, Error> {
        let req = "systems/".to_string() + system_id + "/switches";
        // todo: handle before and limit
        self.get(req.as_str()).await
    }

    pub async fn get_system_fronters(
        &self,
        system_id: &str,
    ) -> Result<Switch, Error> {
        let req = "systems/".to_string() + system_id + "/fronters";
        self.get(req.as_str()).await
    }

    pub async fn create_switch(
        &self,
        system_id: &str,
        member_ids: &[&str],
        time: OffsetDateTime,
    ) -> Result<(), Error> {
        #[derive(Serialize, Deserialize, Debug)]
        struct JsonSwitch {
            #[serde(with = "time::serde::rfc3339")]
            timestamp: OffsetDateTime,
            members: Vec<String>,
        }

        let req = "systems/".to_string() + system_id + "/switches";
        self
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

    pub async fn get_message(&self, id: &str) -> Result<Message, Error> {
        let req = "messages/".to_string() + id;
        self.get(req.as_str()).await
    }
}