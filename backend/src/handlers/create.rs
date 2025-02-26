#[ft_sdk::form]
pub fn create(_i: backend::MySelf) -> ft_sdk::form::Result {
    ft_sdk::form::reload()
}
