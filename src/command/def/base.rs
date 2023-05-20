use clap::{Arg, ArgAction, Command};
use clap::{Parser, Subcommand};


use crate::command::def::group::*;
use crate::command::def::member::*;
use crate::command::def::system::*;

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
pub enum SwitchCommands {
    List {
        system_id: Option<String>,
    },
    New {
        members: Vec<String>,
        time: Option<String>, // todo: validate date
    },
}
