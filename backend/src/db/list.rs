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
    ) -> ft_sdk::Result<Paged<backend::Update>> {
        use backend::schema::cdp_update;
        use diesel::prelude::*;

        // if is_admin, find all posts
        // not logged in -> only public posts
        // logged in -> public posts and user's posts

        // for now assume we only have public posts
        let mut rows = if is_admin {
            cdp_update::table
                .select(backend::db::DbUpdate::as_select())
                .limit(i64::from(self.per_page + 1))
                .filter(cdp_update::id.lt(self.since))
                .order(cdp_update::id.desc())
                .load(conn)?
        } else {
            cdp_update::table
                .select(backend::db::DbUpdate::as_select())
                .filter(cdp_update::id.lt(self.since))
                .filter(cdp_update::is_public.eq(true))
                .limit(i64::from(self.per_page + 1))
                .order(cdp_update::id.desc())
                .load(conn)?
        };

        rows.reverse();

        let next = if rows.len() > self.per_page as usize {
            rows.pop().map(|u| u.id.to_string())
        } else {
            None
        };

        let mut updates = vec![];
        for row in rows {
            updates.push(row.into_update()?);
        }

        Ok(Paged {
            next,
            previous: None,
            items: updates,
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

impl<T: serde::Serialize> Paged<T> {
    pub fn fix_nav_links(&mut self, app_url: &ft_sdk::AppUrl) -> ft_sdk::Result<()> {
        if let Some(next) = &self.next {
            self.next = Some(format!("{}?since={next}", app_url.join("/")?));
        };

        if let Some(previous) = &self.previous {
            self.previous = Some(format!("{}?since={previous}", app_url.join("/")?));
        };

        Ok(())
    }
}
