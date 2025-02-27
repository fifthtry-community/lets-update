pub fn create(
    conn: &mut ft_sdk::Connection,
    user_id: i64,
    content_type: &str,
    content: serde_json::Value,
    links: Vec<backend::Link>,
    tags: Vec<String>,
) -> ft_sdk::Result<String> {
    use backend::schema::cdp_update;
    use diesel::prelude::*;

    let guid = ft_sdk::uuid_without_dashes();

    diesel::insert_into(cdp_update::table)
        .values((
            cdp_update::guid.eq(&guid),
            cdp_update::content_type.eq(content_type),
            cdp_update::content.eq(&serde_json::to_string(&content)?),
            cdp_update::links.eq(&serde_json::to_string(&links)?),
            cdp_update::tags.eq(&serde_json::to_string(&tags)?),
            cdp_update::created_at.eq(ft_sdk::env::now()),
            cdp_update::updated_at.eq(ft_sdk::env::now()),
            cdp_update::reply_to.eq(None::<i64>),
            cdp_update::user_id.eq(user_id),
            cdp_update::is_public.eq(true),
        ))
        .execute(conn)?;

    Ok(guid)
}
