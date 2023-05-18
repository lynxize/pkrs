use std::fs;

use reqwest::Client;

use crate::api::endpoints::*;
use crate::api::types::*;
use crate::command::commands::*;

pub(crate) async fn handle_commands() {
    let client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    let cli = Cli::parse();
}