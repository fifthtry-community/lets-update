#[ft_sdk::form]
pub fn create_text_post(
    mut me: backend::MySelf,
    data: TextPost,
    app_url: ft_sdk::AppUrl,
    host: ft_sdk::Host,
    scheme: backend::HTTPSScheme,
) -> ft_sdk::form::Result {
    let guid = data.save(&mut me.conn)?;
    ft_sdk::form::redirect(app_url.join(&scheme, &host, &format!("/u/{guid}/"))?)
}

struct TextPost {
    title: Option<String>,
    body: Option<String>,
}

impl TextPost {
    fn save(self, conn: &mut ft_sdk::Connection) -> ft_sdk::Result<String> {
        backend::db::create(
            conn,
            backend::TEXT_POST,
            serde_json::json!({"title": self.title, "body": self.body}),
            vec![],
            vec![],
        )
    }
}

impl ft_sdk::FromRequest for TextPost {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let title: ft_sdk::Optional<"title"> = ft_sdk::FromRequest::from_request(req)?;
        let ft_sdk::Optional(body): ft_sdk::Optional<"body"> =
            ft_sdk::FromRequest::from_request(req)?;

        // either title or body is required
        if title.0.is_none() && body.is_none() {
            return Err(title.error("Either title or body is required").into());
        }

        Ok(TextPost {
            title: title.0,
            body,
        })
    }
}
