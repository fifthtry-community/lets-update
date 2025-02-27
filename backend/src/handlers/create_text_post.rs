#[ft_sdk::form]
pub fn create_text_post(
    mut me: backend::MySelf,
    ft_sdk::Json(data): ft_sdk::Json<TextPost>,
    app_url: ft_sdk::AppUrl,
    host: ft_sdk::Host,
    scheme: backend::HTTPSScheme,
) -> ft_sdk::form::Result {
    let guid = data.save(&mut me.conn, me.ud.id)?;
    ft_sdk::form::redirect(app_url.join(&scheme, &host, &format!("/u/{guid}/"))?)
}

#[derive(Debug, serde::Deserialize)]
struct TextPost {
    title: Option<String>,
    body: Option<String>,
    is_public: bool,
}

impl TextPost {
    fn save(self, conn: &mut ft_sdk::Connection, user_id: i64) -> ft_sdk::Result<String> {
        backend::db::create(
            conn,
            user_id,
            backend::TEXT_POST,
            serde_json::json!({"title": self.title, "body": self.body}),
            vec![],
            vec![],
            self.is_public,
        )
    }
}
