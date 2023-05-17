use std::fs;
use clap::builder::Str;
use serde::Deserialize;


const BASE_URL: &str = "https://api.pluralkit.me/v2/";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("A little PluralKit Nonsense");
    let client = reqwest::Client::new();
    let token = fs::read_to_string("token").expect("No PK token found!");

    println!("Test Request");
    let res = client.get(BASE_URL.to_string() + "systems/txipz")
        .header("User-Agent", "Rust CLI Testing Nonsense")
        .header("Authorization", token)
        .send()
        .await?
        .json::<System>()
        .await?;

    println!("got {:#?}", res);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct System {
    id: String,
    name: Option<String>,
    description: Option<String>,
    tag: Option<String>,
    avatar_url: Option<String>,
    created: Option<String>,
    privacy: Option<SystemPrivacy>
}

#[derive(Deserialize, Debug)]
struct SystemPrivacy {
    description_privacy: String,
    pronoun_privacy: String,
    member_list_privacy: String,
    group_list_privacy: String,
    front_privacy: String,
    front_history_privacy: String,
}