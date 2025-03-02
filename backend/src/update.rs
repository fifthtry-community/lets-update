#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Update {
    // every update has a unique url
    pub permalink: String,
    // some updates will have title, to be shown more prominently
    pub title: Option<String>,
    // this is the body of the update, can include markdown syntax
    pub body: Option<String>,
    // if you are linking to something
    pub links: Vec<Link>,
    // in case of image the title/sub-title/body provides enough information for alt text
    pub quote: Option<Quote>,
    pub image: Option<Link>,
    pub video: Option<Video>,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub likes: i64,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Link {
    pub url: String,
    pub title: String,
    pub sub_title: Option<String>,
    pub thumbnail: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Video {
    pub link: Link,
    pub youtube_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FeedItem {
    pub update: Update,
    pub contact: lets_network::Contact,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Quote {
    pub text: String,
    pub by: Option<String>,
    pub link: Option<String>,
    pub image: Option<String>,
}
