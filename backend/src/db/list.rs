#[derive(Debug, serde::Deserialize)]
pub struct ListInput {
    #[expect(unused)]
    tags: Vec<String>,
    #[expect(unused)]
    since: Option<String>,
    #[expect(unused)]
    per_page: Option<u8>,
}

impl ListInput {
    pub fn fetch(
        self,
        conn: &mut ft_sdk::Connection,
        _user: &Option<ft_sdk::UserData>,
        _is_admin: bool,
    ) -> ft_sdk::Result<Vec<backend::Update>> {
        use backend::schema::cdp_update;
        use diesel::prelude::*;

        // if is_admin, find all posts
        // not logged in -> only public posts
        // logged in -> public posts and user's posts

        // for now assume we only have public posts
        let rows = cdp_update::table
            .select(backend::db::DbUpdate::as_select())
            .filter(cdp_update::is_public.eq(true))
            .load(conn)?;

        let mut result = vec![];
        for row in rows {
            result.push(row.into_update()?);
        }

        Ok(result)
    }
}
