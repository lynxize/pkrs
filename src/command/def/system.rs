use clap::Subcommand;

use crate::api::types::AutoProxyMode;

#[derive(Subcommand)]
pub enum SystemCommands {
    Get {
        system_id: Option<String>,
    },
    Set {
        #[clap(subcommand)]
        command: SystemSetCommands,
    },
}

#[derive(Subcommand)]
pub enum SystemSetCommands {
    Name {
        name: String,
    },
    Description {
        description: String,
    },
    Tag {
        tag: String,
    },
    Avatar {
        avatar: String, // todo: validate url
    },
    Timezone {
        timezone: String, // todo, be better
    },
    Pings {
        pings: bool,
    },
    AutoProxy {
        mode: AutoProxyMode,
    },
    Guilds {
        guild_id: String,
        #[clap(subcommand)]
        command: SystemSetGuildCommands,
    },
    Privacy {
        #[clap(subcommand)]
        command: SystemSetPrivacyCommands,
    },
}

#[derive(Subcommand)]
pub enum SystemSetGuildCommands {
    Proxy { should_proxy: bool },
    Tag { tag: String },
}

#[derive(Subcommand)]
pub enum SystemSetPrivacyCommands {
    Description { is_public: bool },
    Pronouns { is_public: bool },
    Members { is_public: bool },
    Groups { is_public: bool },
    Front { is_public: bool },
    FrontHistory { is_public: bool },
}