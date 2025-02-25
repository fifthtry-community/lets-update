// visit /backend/hello-world/ to see the result
#[ft_sdk::processor]
pub fn hello_world(
) -> ft_sdk::processor::Result {
    ft_sdk::processor::json(serde_json::json!({"hello": "there"}))
}
