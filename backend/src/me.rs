pub struct MaybeMe {
    pub now: chrono::DateTime<chrono::Utc>,
    pub ud: Option<ft_sdk::UserData>,
    pub conn: ft_sdk::Connection,
    pub is_admin: bool,
}


impl ft_sdk::FromRequest for MaybeMe {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let cookie: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }> =
            ft_sdk::FromRequest::from_request(req)?;
        let mut conn = ft_sdk::FromRequest::from_request(req)?;
        Ok(MaybeMe {
            now: ft_sdk::FromRequest::from_request(req)?,
            ud: ft_sdk::auth::ud(cookie, &mut conn)?,
            conn,
            is_admin: false,
        })
    }
}

pub struct MySelf {
    pub now: chrono::DateTime<chrono::Utc>,
    pub ud: ft_sdk::UserData,
    pub conn: ft_sdk::Connection,
}

impl ft_sdk::FromRequest for MySelf {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let maybe_me = MaybeMe::from_request(req)?;
        let ud = match maybe_me.ud {
            Some(v) => v,
            None => return Err(ft_sdk::unauthorised!("Not logged in").into()),
        };

        Ok(MySelf {
            now: maybe_me.now,
            ud,
            conn: maybe_me.conn,
        })
    }
}
