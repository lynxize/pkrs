use std::error::Error;

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::api::types::*;

pub const BASE_URL: &str = "https://api.pluralkit.me/v2/";

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


    pub async fn get_system(&self, system_id: &str) -> Result<System, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id;
        self.get(req.as_str()).await
    }

    pub async fn update_system(&self, system: &System) -> Result<System, Box<dyn Error>> {
        let req = "systems/".to_string() + &*system.id;
        self.patch(req.as_str(), system).await
    }

    pub async fn get_system_settings(
        &self,
        system_id: &str,
    ) -> Result<SystemSettings, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/settings";
        self.get(req.as_str()).await
    }

    pub async fn update_system_settings(
        &self,
        system_id: &str,
        settings: &SystemSettings,
    ) -> Result<SystemSettings, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/settings";
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_guild_settings(
        &self,
        system_id: &str,
        guild_id: &str,
    ) -> Result<SystemGuildSettings, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/settings/guilds/" + guild_id;
        self.get(req.as_str()).await
    }

    pub async fn update_system_guild_settings(
        &self,
        system_id: &str,
        guild_id: &str,
        settings: &SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/settings/guilds/" + guild_id;
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_autoproxy_settings(
        &self,
        system_id: &str,
    ) -> Result<AutoProxySettings, Box<dyn Error>> {
        todo!(); // need to handle query string
        let req = "systems/".to_string() + system_id + "/autoproxy";
        self.get(req.as_str()).await
    }

    pub async fn update_system_autoproxy_settings(
        &self,
        system_id: &str,
        settings: &AutoProxySettings,
    ) -> Result<AutoProxySettings, Box<dyn Error>> {
        todo!(); // need to handle query string
        let req = "systems/".to_string() + system_id + "/autoproxy";
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_members(
        &self,
        system_id: &str,
    ) -> Result<Vec<Member>, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/members";
        self.get(req.as_str()).await
    }

    pub async fn create_member(&self, member: &Member) -> Result<Member, Box<dyn Error>> {
        self.post("members", member).await
    }

    pub async fn get_member(&self, member_id: &str) -> Result<Member, Box<dyn Error>> {
        let req = "members/".to_string() + member_id;
        self.get(req.as_str()).await
    }

    pub async fn update_member(&self, member: &Member) -> Result<Member, Box<dyn Error>> {
        let req = "members/".to_string() + &*member.id;
        self.patch(req.as_str(), member).await
    }

    pub async fn delete_member(&self, member_id: &str) -> Result<Response, Box<dyn Error>> {
        let req = "members/".to_string() + member_id;
        self.delete(req.as_str()).await
    }

    pub async fn get_member_groups(
        &self,
        member_id: &str,
    ) -> Result<Vec<Group>, Box<dyn Error>> {
        let req = "members/".to_string() + member_id + "/groups";
        self.get(req.as_str()).await
    }

    pub async fn add_member_groups(
        &self,
        member_id: &str,
        group_id: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let req = "members/".to_string() + member_id + "/groups/add";
        todo!()
    }

    pub async fn remove_member_groups(
        &self,
        member_id: &str,
        group: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let req = "members/".to_string() + member_id + "/groups/remove";
        todo!()
    }

    pub async fn overwrite_member_groups(
        &self,
        member_id: &str,
        group_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let req = "members/".to_string() + member_id + "/groups/overwrite";
        todo!()
    }

    pub async fn get_member_guild_settings(
        &self,
        member_id: &str,
        guild_id: &str,
    ) -> Result<Vec<MemberGuildSettings>, Box<dyn Error>> {
        let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
        self.get(req.as_str()).await
    }

    pub async fn update_member_guild_settings(
        &self,
        member_id: &str,
        guild_id: &str,
        settings: &MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Box<dyn Error>> {
        let req = "members/".to_string() + member_id + "/guilds/" + guild_id;
        self.patch(req.as_str(), settings).await
    }

    pub async fn get_system_groups(
        &self,
        system_id: &str,
    ) -> Result<Vec<Group>, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/groups";
        self.get(req.as_str()).await
    }

    pub async fn create_group(&self, group: &Group) -> Result<Group, Box<dyn Error>> {
        self.post("groups", group).await
    }

    pub async fn get_group(&self, group_id: &str) -> Result<Group, Box<dyn Error>> {
        let req = "groups/".to_string() + group_id;
        self.get(req.as_str()).await
    }

    pub async fn update_group(&self, group: &Group) -> Result<Group, Box<dyn Error>> {
        let req = "groups/".to_string() + &*group.id;
        self.patch(req.as_str(), group).await
    }

    pub async fn delete_group(&self, group_id: &str) -> Result<Response, Box<dyn Error>> {
        let req = "groups/".to_string() + group_id;
        self.delete(req.as_str()).await
    }

    pub async fn get_group_members(
        &self,
        group_id: &str,
    ) -> Result<Vec<Member>, Box<dyn Error>> {
        let req = "groups/".to_string() + group_id + "/members";
        self.get(req.as_str()).await
    }

    pub async fn add_group_members(
        &self,
        group_id: &str,
        members: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let req = "groups/".to_string() + group_id + "/members/add";
        todo!()
    }

    pub async fn remove_group_members(
        &self,
        group_id: &str,
        members: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let req = "groups/".to_string() + group_id + "/members/remove";
        todo!()
    }

    pub async fn overwrite_group_members(
        &self,
        group_id: &str,
        member_ids: &Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        let req = "groups/".to_string() + group_id + "/members/overwrite";
        todo!()
    }

    pub async fn get_system_switches(
        &self,
        system_id: &str,
        before: &str,
        limit: &i32,
    ) -> Result<Vec<Switch>, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/switches";
        // todo: handle before and limit
        self.get(req.as_str()).await
    }

    pub async fn get_system_fronters(
        &self,
        system_id: &str,
    ) -> Result<Switch, Box<dyn Error>> {
        let req = "systems/".to_string() + system_id + "/fronters";
        self.get(req.as_str()).await
    }

    pub async fn create_switch(
        &self,
        system_id: &str,
        member_ids: &Vec<&str>,
        time: String,
    ) -> Result<(), Box<dyn Error>> {
        #[derive(Serialize, Deserialize, Debug)]
        struct JsonSwitch {
            timestamp: String,
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

    pub async fn get_message(&self, id: &str) -> Result<Message, Box<dyn Error>> {
        let req = "messages/".to_string() + id;
        self.get(req.as_str()).await
    }
}