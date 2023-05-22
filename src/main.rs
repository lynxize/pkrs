use std::error::Error;

use crate::command::handle::base::handle_commands;

mod api {
    pub mod client;
    pub mod types;
}

mod command {
    pub mod def {
        pub mod base;
        pub mod group;
        pub mod member;
        pub mod system;
    }

    pub mod handle {
        pub mod base;
        pub mod group;
        pub mod member;
        pub mod system;
    }
}

mod util;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    handle_commands().await
}
