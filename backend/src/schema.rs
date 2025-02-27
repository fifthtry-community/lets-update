diesel::table! {
    cdp_update (id) {
        id -> BigInt,
        guid -> Text,
        content_type -> Text,
        content -> Text,
        links -> Text,
        tags -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        reply_to -> Nullable<BigInt>,
        user_id -> BigInt,
        is_public -> Bool,
    }
}

diesel::table! {
    cdp_update_reaction (id) {
        id -> BigInt,
        update_id -> BigInt,
        user_id -> BigInt,
        reaction -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    cdp_feed (id) {
        id -> BigInt,
        user_id -> BigInt,
        update_guid -> Text,
        source_app -> Text,
        content_type -> Text,
        content -> Text,
        links -> Nullable<Text>,
        tags -> Nullable<Text>,
        reactions -> Nullable<Text>,
        my_reactions -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        read -> Bool,
        archived -> Bool,
    }
}

diesel::joinable!(cdp_update_reaction -> cdp_update (update_id));
