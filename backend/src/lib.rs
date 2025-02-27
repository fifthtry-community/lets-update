#![allow(clippy::derive_partial_eq_without_eq, clippy::get_first)]
#![deny(unused_crate_dependencies)]
#![warn(clippy::used_underscore_binding)]

extern crate self as backend;

mod handlers;
mod schema;

pub struct MySelf {
    pub now: chrono::DateTime<chrono::Utc>,
    pub ud: ft_sdk::UserData,
    pub conn: ft_sdk::Connection,
}

impl ft_sdk::FromRequest for MySelf {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let cookie: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }> =
            ft_sdk::FromRequest::from_request(req)?;
        let mut conn = ft_sdk::FromRequest::from_request(req)?;
        let ud = match ft_sdk::auth::ud(cookie, &mut conn)? {
            Some(v) => v,
            None => return Err(ft_sdk::unauthorised!("Not logged in").into()),
        };
        Ok(MySelf {
            now: ft_sdk::FromRequest::from_request(req)?,
            ud,
            conn,
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Update {
    // every update has a unique url
    pub permalink: String,
    // some updates will have title, to be shown more prominently
    pub title: Option<String>,
    // this is the body of the update, can include markdown syntax
    pub body: Option<String>,
    // if you are linking to something
    pub link: Option<Link>,
    // in case of image the title/sub-title/body provides enough information for alt text
    pub quote: Option<Quote>,
    pub image: Option<Link>,
    pub video: Option<Video>,
    pub tags: Vec<String>,
    pub likes: i64,
    pub comments: i64,
    pub created_on: String,
    pub updated_on: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub url: String,
    pub title: String,
    pub sub_title: Option<String>,
    pub thumbnail: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Video {
    pub link: Link,
    pub youtube_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FeedItem {
    pub update: Update,
    pub contact: lets_network::Contact,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Quote {
    pub text: String,
    pub by: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
}
