pub fn create(
    conn: &mut ft_sdk::Connection,
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
            // created_at -> BigInt,
            // updated_at -> BigInt,
            // reply_to -> Nullable<BigInt>,
            // user_id -> BigInt,
            // is_public -> Bool,
            // cdp_update::title.eq(self.title),
            // cdp_update::body.eq(self.body),
            // cdp_update::link.eq(self.link),
            // cdp_update.quote.eq(self.quote),
            // cdp_update.image.eq(self.image),
            // cdp_update.video.eq(self.video),
            // cdp_update.tags.eq(&self.tags),
            // cdp_update.likes.eq(self.likes),
            // cdp_update.comments.eq(self.comments),
            // cdp_update.permalink.eq(self.permalink),
            // cdp_update.created_on.eq(self.created_on),
            // cdp_update.updated_on.eq(self.updated_on),
        ))
        .execute(conn)?;

    Ok(guid)
}
