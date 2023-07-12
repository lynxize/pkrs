use reqwest::{Client, Error, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

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
    async fn get<T: for<'a> Deserialize<'a>>(&self, endpoint: &str) -> Result<T, Error> {
        self.get_json(self.client.get(BASE_URL.to_string() + endpoint))
            .await
    }

    // of this
    async fn get_query<T: for<'a> Deserialize<'a>>(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<T, Error> {
        self.get_json(
            self.client
                .get(BASE_URL.to_string() + endpoint)
                .query(query),
        )
        .await
    }

    // duplication
    async fn patch<T: for<'a> Deserialize<'a>>(&self, endpoint: &str, body: &T) -> Result<T, Error>
    where
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
    async fn patch_query<T: for<'a> Deserialize<'a>>(
        &self,
        endpoint: &str,
        body: &T,
        query: &[(&str, &str)],
    ) -> Result<T, Error>
    where
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
    async fn post<T: for<'a> Deserialize<'a>>(&self, endpoint: &str, body: &T) -> Result<T, Error>
        where
            T: Serialize,
    {
        self.get_json(self.client.post(BASE_URL.to_string() + endpoint)
            .json(body))
            .await
    }

    // unclean
    async fn post_only<T: Serialize>(&self, endpoint: &str, body: &T) -> Result<Response, Error>  {
        self.req(self.client.post(BASE_URL.to_string() + endpoint)
            .json(body))
            .await
    }

    async fn get_json<T: for<'a> Deserialize<'a>>(&self, builder: RequestBuilder) -> Result<T, Error> {
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

    pub async fn get_system(&self, system_id: &PkId) -> Result<System, Error> {
        let req = format!("systems/{}", system_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn update_system(&self, system: &System) -> Result<System, Error> {
        let req = format!("systems/{}", system.id.to_string());
        self.patch(req.as_str(), system).await
    }

    pub async fn get_system_settings(&self, system_id: &PkId) -> Result<SystemSettings, Error> {
        let req = format!("systems/{}/settings", system_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn update_system_settings(
        &self,
        system_id: &PkId,
        settings: &SystemSettings,
    ) -> Result<SystemSettings, Error> {
        let req = format!("systems/{}/settings", system_id.to_string());
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_guild_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
    ) -> Result<SystemGuildSettings, Error> {
        let req = format!("systems/{}/settings/guilds/{}", system_id.to_string(), guild_id);
        self.get(req.as_str()).await
    }

    pub async fn update_system_guild_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
        settings: &SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Error> {
        let req = format!("systems/{}/settings/guilds/{}", system_id.to_string(), guild_id);
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_autoproxy_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
    ) -> Result<AutoProxySettings, Error> {
        let req = format!("systems/{}/autoproxy", system_id.to_string());
        self.get_query(req.as_str(), &[("guild_id", guild_id)])
            .await
    }

    pub async fn update_system_autoproxy_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
        settings: &AutoProxySettings,
    ) -> Result<AutoProxySettings, Error> {
        let req = format!("systems/{}/autoproxy", system_id.to_string());
        self.patch_query(req.as_str(), settings, &[("guild_id", guild_id)])
            .await
    }

    pub async fn get_system_members(&self, system_id: &PkId) -> Result<Vec<Member>, Error> {
        let req = format!("systems/{}/members", system_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn create_member(&self, member: &Member) -> Result<Member, Error> {
        self.post("members", member).await
    }

    pub async fn get_member(&self, member_id: &PkId) -> Result<Member, Error> {
        let req = format!("members/{}", member_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn update_member(&self, member: &Member) -> Result<Member, Error> {
        let req = format!("members/{}", member.id.to_string());
        self.patch(req.as_str(), member).await
    }

    pub async fn delete_member(&self, member_id: &PkId) -> Result<Response, Error> {
        let req = format!("members/{}", member_id.to_string());
        self.delete(req.as_str()).await
    }

    pub async fn get_member_groups(&self, member_id: &PkId) -> Result<Vec<Group>, Error> {
        let req = format!("members/{}/groups", member_id.to_string());
        self.get(req.as_str()).await
    }

    async fn member_groups(
        &self,
        action: &str,
        member_id: &PkId,
        group_ids: &[&PkId],
    ) -> Result<(), Error> {
        let r = self
            .req(
                self.client
                    .post(format!("{}members/{}/groups/{}", BASE_URL, member_id.to_string(), action))
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
        member_id: &PkId,
        group_ids: &[&PkId],
    ) -> Result<(), Error> {
        self.member_groups("add", member_id, group_ids).await
    }

    pub async fn remove_member_groups(
        &self,
        member_id: &PkId,
        group_ids: &[&PkId],
    ) -> Result<(), Error> {
        self.member_groups("remove", member_id, group_ids).await
    }

    pub async fn overwrite_member_groups(
        &self,
        member_id: &PkId,
        group_ids: &[&PkId],
    ) -> Result<(), Error> {
        self.member_groups("overwrite", member_id, group_ids).await
    }

    pub async fn get_member_guild_settings(
        &self,
        member_id: &PkId,
        guild_id: &str,
    ) -> Result<Vec<MemberGuildSettings>, Error> {
        let req = format!("members/{}/guilds/{}", member_id.to_string(), guild_id);
        self.get(req.as_str()).await
    }

    pub async fn update_member_guild_settings(
        &self,
        member_id: &PkId,
        guild_id: &str,
        settings: &MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Error> {
        let req = format!("members/{}/guilds/{}", member_id.to_string(), guild_id);
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_groups(&self, system_id: &PkId) -> Result<Vec<Group>, Error> {
        let req = format!("systems/{}/groups", system_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn create_group(&self, group: &Group) -> Result<Group, Error> {
        self.post("groups", group).await
    }

    pub async fn get_group(&self, group_id: &PkId) -> Result<Group, Error> {
        let req = format!("groups/{}", group_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn update_group(&self, group: &Group) -> Result<Group, Error> {
        let req = format!("groups/{}", group.id.to_string());
        self.patch(req.as_str(), group).await
    }

    pub async fn delete_group(&self, group_id: &PkId) -> Result<Response, Error> {
        let req = format!("groups/{}", group_id.to_string());
        self.delete(req.as_str()).await
    }

    pub async fn get_group_members(&self, group_id: &PkId) -> Result<Vec<Member>, Error> {
        let req = format!("groups/{}/members", group_id.to_string());
        self.get(req.as_str()).await
    }

    async fn group_members(
        &self,
        action: &str,
        group_id: &PkId,
        member_ids: &[&PkId],
    ) -> Result<(), Error> {
        let r = self
            .req(
                self.client
                    .post(format!("{}groups/{}/members/{}", BASE_URL.to_string(), group_id.to_string(), action))
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
        group_id: &PkId,
        member_ids: &[&PkId],
    ) -> Result<(), Error> {
        self.group_members("add", group_id, member_ids).await
    }

    pub async fn remove_group_members(
        &self,
        group_id: &PkId,
        member_ids: &[&PkId],
    ) -> Result<(), Error> {
        self.group_members("remove", group_id, member_ids).await
    }

    pub async fn overwrite_group_members(
        &self,
        group_id: &PkId,
        member_ids: &[&PkId],
    ) -> Result<(), Error> {
        self.group_members("overwrite", group_id, member_ids).await
    }

    pub async fn get_system_switches(
        &self,
        system_id: &PkId,
        before: &OffsetDateTime,
        limit: &i32,
    ) -> Result<Vec<Switch>, Error> {
        self.get_query(
            format!("systems/{}/switches", system_id.to_string()).as_str(),
            &[
                ("before", before.format(&Rfc3339).unwrap().as_str()),
                ("limit", limit.to_string().as_str()),
            ],
        )
        .await
    }

    pub async fn get_system_fronters(&self, system_id: &PkId) -> Result<Switch, Error> {
        let req = format!("systems/{}/fronters", system_id.to_string());
        self.get(req.as_str()).await
    }

    pub async fn create_switch(
        &self,
        system_id: &PkId,
        member_ids: Vec<PkId>,
        time: Option<OffsetDateTime>,
    ) -> Result<Response, Error> {
        #[derive(Serialize, Deserialize, Debug)]
        struct SwitchCreate {
            #[serde(with = "time::serde::rfc3339::option")]
            timestamp: Option<OffsetDateTime>,
            members: Vec<PkId>,
        }
        self.post_only(
            format!("systems/{}/switches", system_id.to_string()).as_str(),
            &SwitchCreate {
                timestamp: time,
                members: member_ids,
            },
        )
        .await
    }

    pub async fn get_switch(&self, system_id: &PkId, switch_id: &Uuid) -> Result<Vec<Switch>, Error>{
        let req = format!("systems/{}/switches/{}", system_id.to_string(), switch_id);
        self.get(req.as_str()).await
    }

    pub async fn update_switch(&self, system_id: &PkId, switch_id: &Uuid, time: OffsetDateTime) -> Result<Switch, Error>{
        let req = format!("systems/{}/switches/{}", system_id.to_string(), switch_id);
        #[derive(Serialize, Deserialize, Debug)]
        struct SwitchTimeUpdate {
            #[serde(with = "time::serde::rfc3339")]
            timestamp: OffsetDateTime
        }
        self.get_json(
            self.client
                .patch(BASE_URL.to_string() + req.as_str())
                .json(&SwitchTimeUpdate{timestamp: time}),
        ).await
    }

    pub async fn update_switch_members(&self, system_id: &PkId, switch_id: &Uuid, members: &[&PkId]) -> Result<Switch, Error> {
        let req = format!("systems/{}/switches/{}/members", system_id.to_string(), switch_id);
        self.get_json(
            self.client
                .patch(BASE_URL.to_string() + req.as_str())
                .json(members),
        ).await
    }

    pub async fn delete_switch(&self, system_id: &PkId, switch_id: &Uuid) -> Result<Response, Error> {
        let req = format!("systems/{}/switches/{}", system_id.to_string(), switch_id);
        self.delete(req.as_str()).await
    }

    pub async fn get_message(&self, id: &str) -> Result<Message, Error> {
        let req = format!("messages/{}", id);
        self.get(req.as_str()).await
    }
}
