#[ft_sdk::data]
fn post_list(
    mut me: backend::MaybeMe,
    app_url: ft_sdk::AppUrl,
    ft_sdk::Default(tags): ft_sdk::Default<"tags", Vec<String>>,
    ft_sdk::Default(since): ft_sdk::Default<"since", Option<String>>,
    ft_sdk::Default(per_page): ft_sdk::Default<"per-page", Option<u8>>,
) -> ft_sdk::data::Result {
    let per_page = per_page.unwrap_or(10);
    if per_page > 100 {
        return Err(ft_sdk::single_error("per-page", "Can not return more than 100").into());
    }

    let mut posts = backend::db::ListInput {
        tags,
        since,
        per_page,
    }
        .fetch(&mut me.conn, &me.ud, me.is_admin)?;
    posts.iter_mut().for_each(|u| u.permalink = app_url.join(&u.permalink).unwrap());
    ft_sdk::data::json(posts)
}
