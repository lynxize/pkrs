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
    pub async fn get_system(&self, system_id: &PkId) -> Result<System, Error> {
        self.get(format!("systems/{}", system_id)).await
    }

    pub async fn update_system(&self, system: &System) -> Result<System, Error> {
        self.patch_get_json(format!("systems/{}", system.id), system)
            .await
    }

    pub async fn get_system_settings(&self, system_id: &PkId) -> Result<SystemSettings, Error> {
        self.get(format!("systems/{}/settings", system_id)).await
    }

    pub async fn update_system_settings(
        &self,
        system_id: &PkId,
        settings: &SystemSettings,
    ) -> Result<SystemSettings, Error> {
        self.patch_get_json(format!("systems/{}/settings", system_id), settings)
            .await
    }

    pub async fn get_system_guild_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
    ) -> Result<SystemGuildSettings, Error> {
        let req = format!("systems/{}/settings/guilds/{}", system_id, guild_id);
        self.get(req).await
    }

    pub async fn update_system_guild_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
        settings: &SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Error> {
        let req = format!("systems/{}/settings/guilds/{}", system_id, guild_id);
        self.patch_get_json(req, settings).await
    }

    pub async fn get_system_autoproxy_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
    ) -> Result<AutoProxySettings, Error> {
        let req = format!("systems/{}/autoproxy", system_id);
        self.get_query_get_json(req, &[("guild_id", guild_id)])
            .await
    }

    pub async fn update_system_autoproxy_settings(
        &self,
        system_id: &PkId,
        guild_id: &str,
        settings: &AutoProxySettings,
    ) -> Result<AutoProxySettings, Error> {
        let req = format!("systems/{}/autoproxy", system_id);
        self.patch_query_get_json(req, settings, &[("guild_id", guild_id)])
            .await
    }

    pub async fn get_system_members(&self, system_id: &PkId) -> Result<Vec<Member>, Error> {
        self.get(format!("systems/{}/members", system_id)).await
    }

    pub async fn create_member(&self, member: &Member) -> Result<Member, Error> {
        self.post_get_json("members".to_string(), member).await
    }

    pub async fn get_member(&self, member_id: &PkId) -> Result<Member, Error> {
        self.get(format!("members/{}", member_id)).await
    }

    pub async fn update_member(&self, member: &Member) -> Result<Member, Error> {
        self.patch_get_json(format!("members/{}", member.id), member)
            .await
    }

    pub async fn delete_member(&self, member_id: &PkId) -> Result<Response, Error> {
        self.delete(format!("members/{}", member_id)).await
    }

    pub async fn get_member_groups(&self, member_id: &PkId) -> Result<Vec<Group>, Error> {
        self.get(format!("members/{}/groups", member_id)).await
    }

    async fn member_groups(
        &self,
        action: &str,
        member_id: &PkId,
        group_ids: &[&PkId],
    ) -> Result<(), Error> {
        let r = self
            .request(
                self.client
                    .post(format!(
                        "{}members/{}/groups/{}",
                        BASE_URL, member_id, action
                    ))
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
        self.get(format!("members/{}/guilds/{}", member_id, guild_id))
            .await
    }

    pub async fn update_member_guild_settings(
        &self,
        member_id: &PkId,
        guild_id: &str,
        settings: &MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Error> {
        self.patch_get_json(
            format!("members/{}/guilds/{}", member_id, guild_id),
            settings,
        )
        .await
    }

    pub async fn get_system_groups(&self, system_id: &PkId) -> Result<Vec<Group>, Error> {
        self.get(format!("systems/{}/groups", system_id)).await
    }

    pub async fn create_group(&self, group: &Group) -> Result<Group, Error> {
        self.post_get_json("groups".to_string(), group).await
    }

    pub async fn get_group(&self, group_id: &PkId) -> Result<Group, Error> {
        self.get(format!("groups/{}", group_id)).await
    }

    pub async fn update_group(&self, group: &Group) -> Result<Group, Error> {
        self.patch_get_json(format!("groups/{}", group.id), group)
            .await
    }

    pub async fn delete_group(&self, group_id: &PkId) -> Result<Response, Error> {
        self.delete(format!("groups/{}", group_id)).await
    }

    pub async fn get_group_members(&self, group_id: &PkId) -> Result<Vec<Member>, Error> {
        self.get(format!("groups/{}/members", group_id)).await
    }

    async fn group_members(
        &self,
        action: &str,
        group_id: &PkId,
        member_ids: &[&PkId],
    ) -> Result<(), Error> {
        let r = self
            .request(
                self.client
                    .post(format!(
                        "{}groups/{}/members/{}",
                        BASE_URL.to_string(),
                        group_id,
                        action
                    ))
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
        self.get_query_get_json(
            format!("systems/{}/switches", system_id),
            &[
                ("before", before.format(&Rfc3339).unwrap().as_str()),
                ("limit", limit.to_string().as_str()),
            ],
        )
        .await
    }

    pub async fn get_system_fronters(&self, system_id: &PkId) -> Result<Switch, Error> {
        self.get(format!("systems/{}/fronters", system_id)).await
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
        self.post(
            format!("systems/{}/switches", system_id),
            &SwitchCreate {
                timestamp: time,
                members: member_ids,
            },
        )
        .await
    }

    pub async fn get_switch(
        &self,
        system_id: &PkId,
        switch_id: &Uuid,
    ) -> Result<Vec<Switch>, Error> {
        self.get(format!("systems/{}/switches/{}", system_id, switch_id))
            .await
    }

    pub async fn update_switch(
        &self,
        system_id: &PkId,
        switch_id: &Uuid,
        time: OffsetDateTime,
    ) -> Result<Switch, Error> {
        let req = format!("systems/{}/switches/{}", system_id, switch_id);
        #[derive(Serialize, Deserialize, Debug)]
        struct SwitchTimeUpdate {
            #[serde(with = "time::serde::rfc3339")]
            timestamp: OffsetDateTime,
        }
        self.get_response_json(
            self.client
                .patch(BASE_URL.to_string() + req.as_str())
                .json(&SwitchTimeUpdate { timestamp: time }),
        )
        .await
    }

    pub async fn update_switch_members(
        &self,
        system_id: &PkId,
        switch_id: &Uuid,
        members: &[&PkId],
    ) -> Result<Switch, Error> {
        let req = format!("systems/{}/switches/{}/members", system_id, switch_id);
        self.get_response_json(
            self.client
                .patch(BASE_URL.to_string() + req.as_str())
                .json(members),
        )
        .await
    }

    pub async fn delete_switch(
        &self,
        system_id: &PkId,
        switch_id: &Uuid,
    ) -> Result<Response, Error> {
        self.delete(format!("systems/{}/switches/{}", system_id, switch_id))
            .await
    }

    pub async fn get_message(&self, id: &str) -> Result<Message, Error> {
        self.get(format!("messages/{}", id)).await
    }

    // all
    async fn get<T: for<'a> Deserialize<'a>>(&self, endpoint: String) -> Result<T, Error> {
        self.get_response_json(self.client.get(BASE_URL.to_string() + &*endpoint))
            .await
    }

    // of this
    async fn get_query_get_json<T: for<'a> Deserialize<'a>>(
        &self,
        endpoint: String,
        query: &[(&str, &str)],
    ) -> Result<T, Error> {
        self.get_response_json(
            self.client
                .get(BASE_URL.to_string() + &*endpoint)
                .query(query),
        )
        .await
    }

    // duplication
    async fn patch_get_json<T: for<'a> Deserialize<'a>>(
        &self,
        endpoint: String,
        body: &T,
    ) -> Result<T, Error>
    where
        T: Serialize,
    {
        self.get_response_json(
            self.client
                .patch(BASE_URL.to_string() + &*endpoint)
                .json(body),
        )
        .await
    }

    // feels
    async fn patch_query_get_json<T: for<'a> Deserialize<'a>>(
        &self,
        endpoint: String,
        body: &T,
        query: &[(&str, &str)],
    ) -> Result<T, Error>
    where
        T: Serialize,
    {
        self.get_response_json(
            self.client
                .patch(BASE_URL.to_string() + &*endpoint)
                .query(query)
                .json(body),
        )
        .await
    }

    // extremely
    async fn post_get_json<T: for<'a> Deserialize<'a>>(
        &self,
        endpoint: String,
        body: &T,
    ) -> Result<T, Error>
    where
        T: Serialize,
    {
        self.get_response_json(
            self.client
                .post(BASE_URL.to_string() + &*endpoint)
                .json(body),
        )
        .await
    }

    // unclean
    async fn post<T: Serialize>(&self, endpoint: String, body: &T) -> Result<Response, Error> {
        self.request(
            self.client
                .post(BASE_URL.to_string() + &*endpoint)
                .json(body),
        )
        .await
    }

    async fn get_response_json<T: for<'a> Deserialize<'a>>(
        &self,
        builder: RequestBuilder,
    ) -> Result<T, Error> {
        self.request(builder).await?.json::<T>().await
    }

    async fn delete(&self, endpoint: String) -> Result<Response, Error> {
        self.request(self.client.delete(BASE_URL.to_string() + &*endpoint))
            .await
    }

    async fn request(&self, builder: RequestBuilder) -> Result<Response, Error> {
        builder
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await
    }
}
