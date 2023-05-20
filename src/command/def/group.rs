use clap::Subcommand;

#[derive(Subcommand)]
pub enum GroupCommands {
    Get {
        group_id: String,
    },
    List {
        system_id: Option<String>,
    },
    Create {
        #[arg(short, long)]
        name: Option<String>,
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
    Icon {
        icon: String, // todo: validate url
    },
    Banner {
        banner: String, // todo: validate url
    },
    Privacy {
        #[clap(subcommand)]
        command: GroupSetPrivacyCommands,
    },
}

#[derive(Subcommand)]
pub enum GroupSetPrivacyCommands {
    Visibility { is_public: bool },
    Name { is_public: bool },
    Description { is_public: bool },
    Icon { is_public: bool },
    List { is_public: bool },
    Metadata { is_public: bool },
}
