use clap::builder::Str;
use std::fs;
use reqwest::Client;
use serde::Deserialize;

mod types;
mod endpoints;

use crate::types::*;
use crate::endpoints::*;


const BASE_URL: &str = "https://api.pluralkit.me/v2/";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("A little PluralKit Nonsense");
    let client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    println!("Test Request");
    let sys = get_system(&client, "txipz").await?;
    let members = sys.get_members(&client).await?;

    println!("got {:#?}", members);

    Ok(())
}

pub struct PkClient {
    client: Client,
    token: String,
    user_agent: String
}

impl PkClient {
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Box<dyn std::error::Error>>
        where T: for<'a> Deserialize<'a>
    {
        let res = self.client
            .get(BASE_URL.to_string() + endpoint)
            .header("User-Agent", &self.user_agent)
            .header("Authorization", &self.token)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(res)
    }
}