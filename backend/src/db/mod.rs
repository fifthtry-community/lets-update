mod create;
mod list;

pub use create::create;
pub use list::ListInput;

#[derive(diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = backend::schema::cdp_update)]
pub struct DbUpdate {
    #[expect(unused)]
    pub guid: String,
    #[expect(unused)]
    pub content_type: String,
    #[expect(unused)]
    pub content: String,
    #[expect(unused)]
    pub links: String,
    #[expect(unused)]
    pub tags: String,
    #[expect(unused)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[expect(unused)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[expect(unused)]
    pub reply_to: Option<i64>,
    #[expect(unused)]
    pub user_id: i64,
    #[expect(unused)]
    pub is_public: bool,
}
