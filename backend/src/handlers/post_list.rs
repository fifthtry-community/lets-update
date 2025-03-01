#[ft_sdk::data]
fn post_list(
    mut me: backend::MaybeMe,
    app_url: ft_sdk::AppUrl,
) -> ft_sdk::data::Result {
    let posts: Vec<_> = backend::db::ListInput::default()
        .fetch(&mut me.conn, &me.ud, me.is_admin)?
        .into_iter()
        .map(|mut u| u.permalink = app_url.join(u.permalink).unwrap())
        .collect();
    ft_sdk::data::json(posts)
}
