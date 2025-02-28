#[derive(Debug, serde::Deserialize)]
pub struct ListInput {
    #[expect(unused)]
    tags: Vec<String>,
    #[expect(unused)]
    since: Option<String>,
    #[expect(unused)]
    per_page: Option<u8>,
}

impl ListInput {
    pub fn fetch(
        self,
        _conn: &mut ft_sdk::Connection,
        _user: &Option<ft_sdk::UserData>,
    ) -> ft_sdk::Result<Vec<backend::Update>> {
        todo!()
    }
}
