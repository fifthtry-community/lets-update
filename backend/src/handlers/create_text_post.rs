#[ft_sdk::form]
pub fn create_text_post(
    mut me: backend::MaybeMe,
    ft_sdk::Json(data): ft_sdk::Json<TextPost>,
    // app_url: ft_sdk::AppUrl,
) -> ft_sdk::form::Result {
    if data.title.is_none() && data.body.is_none() {
        return Err(ft_sdk::single_error("title", "Either title or body must be provided").into());
    }
    // todo: we are hardcoding user_id to 1 since auth is not working on my machine
    data.save(&mut me.conn, me.ud.map(|v| v.id).unwrap_or(1))?;
    // ft_sdk::form::redirect(app_url.join(backend::urls::post(guid))?)
    ft_sdk::form::reload()
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
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
