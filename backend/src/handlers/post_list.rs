#[ft_sdk::data]
fn post_list(
    mut me: backend::MaybeMe,
    app_url: ft_sdk::AppUrl,
) -> ft_sdk::data::Result {
    let mut posts = backend::db::ListInput::default()
        .fetch(&mut me.conn, &me.ud, me.is_admin)?;
    posts.iter_mut().for_each(|u| u.permalink = app_url.join(&u.permalink).unwrap());
    ft_sdk::data::json(posts)
}
