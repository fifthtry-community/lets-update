#[ft_sdk::data]
fn post_list(
    mut me: backend::MaybeMe,
    mut app_url: ft_sdk::AppUrl,
    ft_sdk::Query(tags): ft_sdk::Query<"tags", Option<String>>,
    ft_sdk::Query(since): ft_sdk::Query<"since", Option<String>>,
    per_page: ft_sdk::Default<"per-page", Option<u8>>,
) -> ft_sdk::data::Result {
    if app_url.url.as_ref().map(AsRef::as_ref) == Some("/backend/") {
        app_url.url = Some("/updates/".to_string());
    };

    ft_sdk::println!("since: {since:?}, app_url: {app_url:?}, tags: {tags:?}");
    let per_page = per_page
        .check(|p| p < &Some(100), "Can not return more than 100 rows")?
        .get()
        .unwrap_or(10);
    let tags = tags
        .map(|t| t.split(',').map(|s| s.to_string()).collect())
        .unwrap_or_default();
    ft_sdk::data::json(
        backend::db::ListInput {
            tags,
            since: since
                .map(|s| serde_json::from_str(s.as_str()).unwrap_or(i64::MAX))
                .unwrap_or(i64::MAX),
            per_page,
        }
        .fetch(&mut me.conn, &me.ud, me.is_admin, &app_url)?,
    )
}
