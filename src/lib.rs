#![doc(
    html_logo_url = "https://i.postimg.cc/3rGyjPqQ/logo.png",
    html_favicon_url = "https://i.postimg.cc/3rGyjPqQ/logo.png"
)]
//! # Async Discord Library
//!
//! Panda it's a very simple and friendly discord api library
//!
//! # Features
//! - __Fast__
//! - __Simple__
//!

//#![deny(missing_docs)]
// Used by futures::select!
#![recursion_limit = "1024"]
// Modules
#[doc(inline)]
pub mod client;
#[doc(inline)]
pub mod error;
#[doc(inline)]
pub mod models;
#[doc(inline)]
pub mod utils;

mod gateway;
mod http;

// Re-exports
pub use models::gateway::events;

// Shortcut functions

/// Create a new panda Client with the default configs
pub async fn new(token: impl Into<String>) -> error::Result<client::Client> {
    client::Client::new(token).await
}
