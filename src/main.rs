use std::error::Error;
use std::fs;


use clap::{Parser, Subcommand};
use clap::{Arg, ArgAction, Command};
use reqwest::Client;

use crate::endpoints::*;
use crate::types::*;

mod types;
mod endpoints;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = PkClient {
        client: Client::new(),
        token: fs::read_to_string("token").expect("No PK token found!"),
        user_agent: "Testing Rust CLI nonsense".to_string(),
    };

    let cli = Cli::parse();

    Ok(())
}



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Token {
        token: String,
    },
    System {
        #[clap(subcommand)]
        command: SystemCommands,
    },
    Member {
       #[clap(subcommand)]
        command: MemberCommands,
    },
    Group {
        #[clap(subcommand)]
        command: GroupCommands,
    },
    Switch {
        #[clap(subcommand)]
        command: SwitchCommands,
    }
}

#[derive(Subcommand)]
enum SystemCommands {
    Get {
        system_id: String,
    },
    Set {
        #[clap(subcommand)]
        command: SystemSetCommands,
    }
}

#[derive(Subcommand)]
enum SystemSetCommands {
    Timezone {
        timezone: String // todo, be better
    },
    Pings {
        pings: bool,
    },
    AutoProxy {
       mode: AutoProxyMode
    },
    Guilds {
        guild_id: String,
        #[clap(subcommand)]
        command: SystemSetGuildCommands
    }
}

#[derive(Subcommand)]
enum SystemSetGuildCommands {
    Proxy {
        should_proxy: bool
    },
    Tag {
        tag: String
    }
}

#[derive(Subcommand)]
enum MemberCommands {
    Get {
        member_id: String,
    },
    List {
        system_id: Option<String>,
    },
    Set {
        member_id: String,
        #[clap(subcommand)]
        command: MemberSetCommands,
    },
    Create,
    Delete {
        member_id: String,
    },
    Group {
        member_id: String,
        #[clap(subcommand)]
        command: MemberGroupCommands,
    }
}

#[derive(Subcommand)]
enum MemberSetCommands {
    Name {
        name: String
    },
    DisplayName {
        display_name: String
    },
    Description {
        description: String
    },
    Color {
        color: String // todo: be better
    },
    Birthday {
        birthday: String // todo: be better
    },
    Pronouns {
        pronouns: String
    },
    Avatar {
        avatar: String // todo: validate url
    },
    Banner {
        banner: String // todo: validate url
    },
    Proxy {
        #[clap(subcommand)]
        command: MemberSetProxyCommands
    },
    Privacy {
        #[clap(subcommand)]
        command: MemberSetPrivacyCommands
    }
}

#[derive(Subcommand)]
enum MemberSetProxyCommands {
    Add {
        tag: String
    },
    Delete {
        tag: String
    },
    List
}


#[derive(Subcommand)]
enum MemberSetPrivacyCommands {
    Visibility {
        is_public: bool
    },
    Name {
        is_public: bool
    },
    Description {
        is_public: bool
    },
    Avatar {
        is_public: bool
    },
    Pronouns {
        is_public: bool
    },
    Birthday {
        is_public: bool
    },
    Metadata {
        is_public: bool
    }
}

#[derive(Subcommand)]
enum MemberGroupCommands {
    List,
    Add {
        group_id: String
    },
    Remove {
        group_id: String
    }
}

#[derive(Subcommand)]
enum GroupCommands {
    Get {
        group_id: String,
    },
    List {
        system_id: Option<String>,
    },
    Create {
        name: String,
    },
    Add {
        group_id: String,
        member_id: String
    },
    Remove {
        group_id: String,
        member_id: String
    },
    Set {
        #[clap(subcommand)]
        command: GroupSetCommands,
    }
}

#[derive(Subcommand)]
enum GroupSetCommands {
    Name {
        name: String
    },
    DisplayName {
        display_name: String
    },
    Description {
        description: String
    },
    Color {
        color: String // todo: be better
    },
    Icon {
        icon: String // todo: validate url
    },
    Banner {
        banner: String // todo: validate url
    },
    Privacy {
        #[clap(subcommand)]
        command: GroupSetPrivacyCommands
    }
}

#[derive(Subcommand)]
enum GroupSetPrivacyCommands {
    Visibility {
        is_public: bool
    },
    Name {
        is_public: bool
    },
    Description {
        is_public: bool
    },
    Icon {
        is_public: bool
    },
    List {
        is_public: bool
    },
    Metadata {
        is_public: bool
    }
}

#[derive(Subcommand)]
enum SwitchCommands {
    List {
        system_id: Option<String>,
    },
    New {
        members: Vec<String>
    }
}