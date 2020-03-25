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
pub mod models;
#[doc(inline)]
pub mod utils;

mod error;
mod gateway;
mod http;

pub use error::PandaError;
pub use http::HttpClient;

// Re-exports
pub use client::Session;
pub use models::gateway::events;

// Types
pub type HandlerResult = Result<(), Box<dyn std::error::Error>>;

/// Create a new panda Client with the default configs
pub async fn new(token: impl Into<String>) -> error::Result<client::Client> {
    client::Client::new(token).await
}
