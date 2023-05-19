use clap::{Parser, Subcommand};
use clap::{Arg, ArgAction, Command};

use crate::types::AutoProxyMode;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
    },
}

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
        command: SystemSetGuildCommands,
    },
    Privacy {
        #[clap(subcommand)]
        command: SystemSetPrivacyCommands,
    },
}

#[derive(Subcommand)]
pub enum SystemSetGuildCommands {
    Proxy {
        should_proxy: bool
    },
    Tag {
        tag: String
    },
}

#[derive(Subcommand)]
pub enum SystemSetPrivacyCommands {
    Description {
        is_public: bool
    },
    Pronouns {
        is_public: bool
    },
    Members {
        is_public: bool
    },
    Groups {
        is_public: bool
    },
    Front {
        is_public: bool
    },
    FrontHistory {
        is_public: bool
    }
}

#[derive(Subcommand)]
pub enum MemberCommands {
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
    },
}

#[derive(Subcommand)]
pub enum MemberSetCommands {
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
    },
}

#[derive(Subcommand)]
pub enum MemberSetProxyCommands {
    Add {
        tag: String
    },
    Delete {
        tag: String
    },
    List,
}

#[derive(Subcommand)]
pub enum MemberSetPrivacyCommands {
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
    },
}

#[derive(Subcommand)]
pub enum MemberGroupCommands {
    List,
    Add {
        group_id: String
    },
    Remove {
        group_id: String
    },
}

#[derive(Subcommand)]
pub enum GroupCommands {
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
        member_id: String,
    },
    Remove {
        group_id: String,
        member_id: String,
    },
    Set {
        #[clap(subcommand)]
        command: GroupSetCommands,
    },
}

#[derive(Subcommand)]
pub enum GroupSetCommands {
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
    },
}

#[derive(Subcommand)]
pub enum GroupSetPrivacyCommands {
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
    },
}

#[derive(Subcommand)]
pub enum SwitchCommands {
    List {
        system_id: Option<String>,
    },
    New {
        members: Vec<String>
    },
}