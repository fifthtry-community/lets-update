#[derive(Debug, Default, serde::Deserialize)]
pub struct ListInput {
    #[expect(unused)]
    pub tags: Vec<String>,
    pub since: i64,
    pub per_page: u8,
}

impl ListInput {
    pub fn fetch(
        self,
        conn: &mut ft_sdk::Connection,
        _user: &Option<ft_sdk::UserData>,
        is_admin: bool,
        app_url: &ft_sdk::AppUrl,
    ) -> ft_sdk::Result<Paged<backend::Update>> {
        use backend::schema::cdp_update;
        use diesel::prelude::*;

        // if is_admin, find all posts
        // not logged in -> only public posts
        // logged in -> public posts and user's posts

        // for now assume we only have public posts
        let (mut rows, mut previous_rows) = if is_admin {
            let rows = cdp_update::table
                .select(backend::db::DbUpdate::as_select())
                .limit(i64::from(self.per_page + 1))
                .filter(cdp_update::id.le(self.since))
                .order(cdp_update::id.desc())
                .load(conn)?;

            let previous_rows = cdp_update::table
                .select(cdp_update::id)
                .filter(cdp_update::id.ge(self.since))
                .order(cdp_update::id.asc())
                .limit(i64::from(self.per_page + 2))
                .load::<i64>(conn)?;

            (rows, previous_rows)
        } else {
            let rows = cdp_update::table
                .select(backend::db::DbUpdate::as_select())
                .filter(cdp_update::id.le(self.since))
                .filter(cdp_update::is_public.eq(true))
                .limit(i64::from(self.per_page + 1))
                .order(cdp_update::id.desc())
                .load(conn)?;

            let previous_rows = cdp_update::table
                .select(cdp_update::id)
                .filter(cdp_update::id.ge(self.since))
                .filter(cdp_update::is_public.eq(true))
                .order(cdp_update::id.asc())
                .limit(i64::from(self.per_page + 2))
                .load::<i64>(conn)?;

            (rows, previous_rows)
        };

        let next = if rows.len() > self.per_page as usize {
            rows.pop()
                .map(|u| format!("{}?since={id}", app_url.join("/").unwrap(), id = u.id))
        } else {
            None
        };

        let previous = if previous_rows.is_empty() {
            None
        } else if previous_rows.len() <= (self.per_page + 1) as usize {
            Some(app_url.join("/")?)
        } else {
            previous_rows.pop();
            
            previous_rows
                .pop()
                .map(|id| format!("{}?since={id}", app_url.join("/").unwrap()))
        };

        let mut items = vec![];
        for row in rows {
            items.push(row.into_update(app_url)?);
        }

        Ok(Paged {
            next,
            previous,
            items,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Paged<ITEM: serde::Serialize> {
    // this is what you should use as since for the next page
    pub next: Option<String>,
    // this is what you should use as since for the previous page
    pub previous: Option<String>,
    pub items: Vec<ITEM>,
}
