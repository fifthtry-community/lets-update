#[ft_sdk::data]
fn post_list(
    _me: backend::MaybeMe,
    ft_sdk::Json(_input): ft_sdk::Json<Input>,
) -> ft_sdk::data::Result {
    todo!()
}

#[derive(Debug, serde::Deserialize)]
struct Input {
    #[expect(unused)]
    tags: Vec<String>,
    #[expect(unused)]
    since: Option<String>,
    #[expect(unused)]
    per_page: Option<u8>,
}
