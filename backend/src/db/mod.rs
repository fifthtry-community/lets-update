mod create;
mod list;

pub use create::create;
pub use list::ListInput;

#[derive(diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = backend::schema::cdp_update)]
#[diesel(check_for_backend(ft_sdk::Sqlite))]
pub struct DbUpdate {
    pub id: i64,
    pub guid: String,
    pub content_type: String,
    pub content: String,
    pub links: String,
    pub tags: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[expect(unused)]
    pub reply_to: Option<i64>,
    #[expect(unused)]
    pub user_id: i64,
    pub is_public: bool,
}

impl DbUpdate {
    fn into_update(self, app_url: &ft_sdk::AppUrl) -> ft_sdk::Result<backend::Update> {
        match self.content_type.as_str() {
            backend::TEXT_POST => {
                #[derive(serde::Deserialize, Default)]
                #[serde(default)]
                struct TB {
                    title: Option<String>,
                    body: Option<String>,
                }

                let tb = serde_json::from_str::<TB>(&self.content)?;
                Ok(backend::Update {
                    permalink: app_url.join(backend::urls::post(self.guid))?,
                    title: tb.title,
                    body: tb.body,
                    links: serde_json::from_str(&self.links)?,
                    tags: serde_json::from_str(&self.tags)?,
                    is_public: self.is_public,
                    created_at: self.created_at.to_rfc3339(),
                    updated_at: self.updated_at.to_rfc3339(),

                    quote: None,
                    image: None,
                    video: None,

                    // TODO
                    likes: 0,
                    comments: 0,
                    // user_id, reply_to
                })
            }
            _ => unimplemented!(),
        }
    }
}
