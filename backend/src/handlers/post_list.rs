#[ft_sdk::data]
fn post_list(
    mut me: backend::MaybeMe,
    app_url: ft_sdk::AppUrl,
    ft_sdk::Default(tags): ft_sdk::Default<"tags", Vec<String>>,
    ft_sdk::Default(since): ft_sdk::Default<"since", Option<String>>,
    per_page: ft_sdk::Default<"per-page", Option<u8>>,
) -> ft_sdk::data::Result {
    ft_sdk::println!("since: {since:?}, app_url: {app_url:?}, tags: {tags:?}");
    let per_page = per_page
        .check(|p| p < &Some(100), "Can not return more than 100 rows")?
        .get()
        .unwrap_or(10);
    let mut posts = backend::db::ListInput {
        tags,
        since: since
            .map(|s| serde_json::from_str(s.as_str()).unwrap_or(i64::MAX))
            .unwrap_or(i64::MAX),
        per_page,
    }
        .fetch(&mut me.conn, &me.ud, me.is_admin)?;
    posts.fix_nav_links(&app_url)?;
    posts
        .items
        .iter_mut()
        .for_each(|u| u.permalink = app_url.join(&u.permalink).unwrap());
    ft_sdk::data::json(posts)
}
