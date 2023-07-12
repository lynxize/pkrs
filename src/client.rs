use reqwest::{Client, Error, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::model::*;

const BASE_URL: &str = "https://api.pluralkit.me/v2/";

pub struct PkClient {
    pub client: Client,
    pub token: String,
    pub user_agent: String,
}

impl Default for PkClient {
    fn default() -> Self {
        PkClient {
            client: Client::new(),
            token: "".to_string(),
            user_agent: "pk + rust project".to_string(),
        }
    }
}

impl PkClient {
    // all
    async fn get<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.get_json(self.client.get(BASE_URL.to_string() + endpoint))
            .await
    }

    // of this
    async fn get_query<T>(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.get_json(
            self.client
                .get(BASE_URL.to_string() + endpoint)
                .query(query),
        )
        .await
    }

    // duplication
    async fn patch<T>(&self, endpoint: &str, body: &T) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a>,
        T: Serialize,
    {
        self.get_json(
            self.client
                .patch(BASE_URL.to_string() + endpoint)
                .json(body),
        )
        .await
    }

    // feels
    async fn patch_query<T>(
        &self,
        endpoint: &str,
        body: &T,
        query: &[(&str, &str)],
    ) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a>,
        T: Serialize,
    {
        self.get_json(
            self.client
                .patch(BASE_URL.to_string() + endpoint)
                .query(query)
                .json(body),
        )
        .await
    }

    // extremely
    async fn post<T>(&self, endpoint: &str, body: &T) -> Result<T, Error>
        where
            T: for<'a> Deserialize<'a>,
            T: Serialize,
    {
        self.get_json(self.client.post(BASE_URL.to_string() + endpoint).json(body))
            .await
    }

    // unclean
    async fn post_only<T>(&self, endpoint: &str, body: &T) -> Result<Response, Error>
    where
        T: for<'a> Deserialize<'a>,
        T: Serialize,
    {
        self.req(self.client.post(BASE_URL.to_string() + endpoint).json(body))
            .await
    }

    async fn get_json<T>(&self, builder: RequestBuilder) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a>,
    {
        let r = self.req(builder).await?;
        r.json::<T>().await
    }

    async fn req(&self, builder: RequestBuilder) -> Result<Response, Error> {
        builder
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await
    }

    async fn delete(&self, endpoint: &str) -> Result<Response, Error> {
        self.client
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

    pub async fn get_system_settings(&self, system_id: &str) -> Result<SystemSettings, Error> {
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
        guild_id: &str,
    ) -> Result<AutoProxySettings, Error> {
        let req = "systems/".to_string() + system_id + "/autoproxy";
        self.get_query(req.as_str(), &[("guild_id", guild_id)])
            .await
    }

    pub async fn update_system_autoproxy_settings(
        &self,
        system_id: &str,
        guild_id: &str,
        settings: &AutoProxySettings,
    ) -> Result<AutoProxySettings, Error> {
        let req = "systems/".to_string() + system_id + "/autoproxy";
        self.patch_query(req.as_str(), settings, &[("guild_id", guild_id)])
            .await
    }

    pub async fn get_system_members(&self, system_id: &str) -> Result<Vec<Member>, Error> {
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

    pub async fn get_member_groups(&self, member_id: &str) -> Result<Vec<Group>, Error> {
        let req = "members/".to_string() + member_id + "/groups";
        self.get(req.as_str()).await
    }

    async fn member_groups(
        &self,
        action: &str,
        member_id: &str,
        group_ids: &[&str],
    ) -> Result<(), Error> {
        let r = self
            .req(
                self.client
                    .post(BASE_URL.to_string() + "members/" + member_id + "/groups/" + action)
                    .json(group_ids),
            )
            .await?;
        if r.status() == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            // does this even work?
            Err(r.error_for_status().unwrap_err())
        }
    }

    pub async fn add_member_groups(
        &self,
        member_id: &str,
        group_ids: &[&str],
    ) -> Result<(), Error> {
        self.member_groups("add", member_id, group_ids).await
    }

    pub async fn remove_member_groups(
        &self,
        member_id: &str,
        group_ids: &[&str],
    ) -> Result<(), Error> {
        self.member_groups("remove", member_id, group_ids).await
    }

    pub async fn overwrite_member_groups(
        &self,
        member_id: &str,
        group_ids: &[&str],
    ) -> Result<(), Error> {
        self.member_groups("overwrite", member_id, group_ids).await
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

    pub async fn get_system_groups(&self, system_id: &str) -> Result<Vec<Group>, Error> {
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

    pub async fn get_group_members(&self, group_id: &str) -> Result<Vec<Member>, Error> {
        let req = "groups/".to_string() + group_id + "/members";
        self.get(req.as_str()).await
    }

    async fn group_members(
        &self,
        action: &str,
        group_id: &str,
        member_ids: &[&str],
    ) -> Result<(), Error> {
        let r = self
            .req(
                self.client
                    .post(BASE_URL.to_string() + "groups/" + group_id + "/members/" + action)
                    .json(member_ids),
            )
            .await?;
        if r.status() == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            // does this even work?
            Err(r.error_for_status().unwrap_err())
        }
    }

    pub async fn add_group_members(
        &self,
        group_id: &str,
        member_ids: &[&str],
    ) -> Result<(), Error> {
        self.group_members("add", group_id, member_ids).await
    }

    pub async fn remove_group_members(
        &self,
        group_id: &str,
        member_ids: &[&str],
    ) -> Result<(), Error> {
        self.group_members("remove", group_id, member_ids).await
    }

    pub async fn overwrite_group_members(
        &self,
        group_id: &str,
        member_ids: &[&str],
    ) -> Result<(), Error> {
        self.group_members("overwrite", group_id, member_ids).await
    }

    pub async fn get_system_switches(
        &self,
        system_id: &str,
        before: &OffsetDateTime,
        limit: &i32,
    ) -> Result<Vec<Switch>, Error> {
        let req = "systems/".to_string() + system_id + "/switches";
        self.get_query(
            req.as_str(),
            &[
                ("before", before.format(&Rfc3339).unwrap().as_str()),
                ("limit", limit.to_string().as_str()),
            ],
        )
        .await
    }

    pub async fn get_system_fronters(&self, system_id: &str) -> Result<Switch, Error> {
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
        self.post_only::<JsonSwitch>(
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
