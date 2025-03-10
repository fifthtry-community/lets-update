#[ft_sdk::form]
pub fn create_text_post(
    mut me: backend::MySelf,
    ft_sdk::Json(mut data): ft_sdk::Json<TextPost>,
    app_url: ft_sdk::AppUrl,
) -> ft_sdk::form::Result {
    data.check()?;

    if !me.ud.verified_email {
        return Err(ft_sdk::single_error("title", "Email must be verified to post.").into());
    }
    data.save(&mut me.conn, me.ud.id)?;
    ft_sdk::form::redirect(app_url.join("/")?)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TextPost {
    title: Option<String>,
    body: Option<String>,
    is_public: bool,
}

impl TextPost {
    fn check(&mut self) -> ft_sdk::Result<()> {
        if self.body.as_ref().map(|v| v.trim().is_empty()).unwrap_or(true) {
            self.body = None;
        }

        if self.title.as_ref().map(|v| v.trim().is_empty()).unwrap_or(true) {
            self.title = None;
        }

        if self.title.is_none() && self.body.is_none() {
            return Err(ft_sdk::single_error("title", "Either title or body must be provided").into());
        }

        Ok(())
    }

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
