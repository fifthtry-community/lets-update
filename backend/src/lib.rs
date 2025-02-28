#![allow(clippy::derive_partial_eq_without_eq, clippy::get_first)]
#![deny(unused_crate_dependencies)]
#![warn(clippy::used_underscore_binding)]

extern crate self as backend;

mod db;
mod handlers;
mod me;
mod schema;
mod update;
mod urls;

pub use me::{MaybeMe, MySelf};
pub use update::{FeedItem, Link, Quote, Update, Video};

pub const TEXT_POST: &str = "text-post";

// fastn is buggy so we have to do this.
struct HTTPSScheme(pub ft_sdk::Scheme);

impl ft_sdk::FromRequest for HTTPSScheme {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let host = ft_sdk::Host::from_request(req)?;

        if host.without_port() == "127.0.0.1" {
            Ok(HTTPSScheme(ft_sdk::Scheme::Http))
        } else {
            Ok(HTTPSScheme(ft_sdk::Scheme::Https))
        }
    }
}

impl std::ops::Deref for HTTPSScheme {
    type Target = ft_sdk::Scheme;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
