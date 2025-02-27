#[ft_sdk::form]
pub fn create(mut me: backend::MySelf, data: CreateUpdate) -> ft_sdk::form::Result {
    data.save(&mut me.conn)?;
    ft_sdk::form::redirect("/")
}

struct CreateUpdate {
    title: Option<String>,
    body: Option<String>,
}

impl CreateUpdate {
    fn save(&self, _conn: &ft_sdk::Connection) -> ft_sdk::Result<()> {
        todo!()
    }
}

impl ft_sdk::FromRequest for CreateUpdate {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let title: ft_sdk::Optional<"title"> = ft_sdk::FromRequest::from_request(req)?;
        let ft_sdk::Optional(body): ft_sdk::Optional<"body"> =
            ft_sdk::FromRequest::from_request(req)?;

        // either title or body is required
        if title.0.is_none() && body.is_none() {
            return Err(title.error("Either title or body is required").into());
        }

        Ok(CreateUpdate {
            title: title.0,
            body,
        })
    }
}
