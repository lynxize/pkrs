use clap::Subcommand;

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
    Create {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        avatar: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Delete {
        member_id: String,
        #[arg(short, long)]
        confirm: Option<bool>,
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
        name: String,
    },
    DisplayName {
        display_name: String,
    },
    Description {
        description: String,
    },
    Color {
        color: String, // todo: be better
    },
    Birthday {
        birthday: String, // todo: be better
    },
    Pronouns {
        pronouns: String,
    },
    Avatar {
        avatar: String, // todo: validate url
    },
    Banner {
        banner: String, // todo: validate url
    },
    Proxy {
        #[clap(subcommand)]
        command: MemberSetProxyCommands,
    },
    Privacy {
        #[clap(subcommand)]
        command: MemberSetPrivacyCommands,
    },
}

#[derive(Subcommand)]
pub enum MemberSetProxyCommands {
    Add { tag: String },
    Delete { tag: String },
    List,
}

#[derive(Subcommand)]
pub enum MemberSetPrivacyCommands {
    Visibility { is_public: bool },
    Name { is_public: bool },
    Description { is_public: bool },
    Avatar { is_public: bool },
    Pronouns { is_public: bool },
    Birthday { is_public: bool },
    Metadata { is_public: bool },
}

#[derive(Subcommand)]
pub enum MemberGroupCommands {
    List,
    Add { group_id: String },
    Remove { group_id: String },
}