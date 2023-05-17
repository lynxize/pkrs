mod types;
mod endpoints;

use std::error::Error;
use std::fs;

use clap::builder::Str;
use clap::Command;
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    Command::new("pkrs")
        .author("_Snowdrift <snowdriftdev@gmail.com>")
        .about("PluralKit Nonsense")




    Ok(())
}
