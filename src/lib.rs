//! A very simple wrapper around the [PluralKit](https://pluralkit.me/) api using [reqwest](https://docs.rs/reqwest/latest/reqwest/) and [serde](https://crates.io/crates/serde).
//!
//! It closely follows the structure of the API itself, and as such the official [API documentation](https://pluralkit.me/api/) is likely the best resource for now.
//!
//! # Examples
//! Creating a `PkClient`
//! ```
//! use pkrs::client::PkClient;
//!
//! let client = PkClient {
//!     token: my-token,
//!     user_agent: "my pk+rust project".to_string(),
//!     client: Default::default()
//! };
//! ```

pub mod client;
pub mod model;
