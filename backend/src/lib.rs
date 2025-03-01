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
