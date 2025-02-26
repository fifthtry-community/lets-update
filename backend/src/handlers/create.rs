#[ft_sdk::form]
pub fn create(
    _i: backend::MySelf,
    title: ft_sdk::Optional<"title">,
    ft_sdk::Optional(body): ft_sdk::Optional<"body">,
) -> ft_sdk::form::Result {
    // either title or body is required
    if title.0.is_none() && body.is_none() {
        return Err(title.error("Either title or body is required").into());
    }

    ft_sdk::form::reload()
}
