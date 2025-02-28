#[ft_sdk::data]
fn post_list(
    mut me: backend::MaybeMe,
    ft_sdk::Json(input): ft_sdk::Json<backend::db::ListInput>,
) -> ft_sdk::data::Result {
    ft_sdk::data::json(input.fetch(&mut me.conn, &me.ud)?)
}
