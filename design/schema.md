# Schema

This app assumes every domain is a single user domain. This means for companies
and other orgs, the `cdp_feed` table should not be used. Orgs can create updates
so the `cdp_update` table should be used.

## cdp_feed

```sql
CREATE TABLE cdp_feed (
    id INTEGER PRIMARY KEY,
    update_id INTEGER NOT NULL, -- ID of the corresponding update
    update_source TEXT NOT NULL, -- Type of update source (e.g., Instagram, GitHub, Twitter, etc.)
    user_id INTEGER NOT NULL, -- Foreign key to the user who created the update
    content_type TEXT NOT NULL, -- Type of content (text, quote, link, review, testimonial, greeting, photo, video, workout, etc.)
    content TEXT NOT NULL, -- JSON field to store the content of the update
    links TEXT, -- JSON field to store links for media posts
    tags TEXT, -- JSON: Comma-separated list of tags
    my_tags TEXT, -- JSON: Comma-separated list of tags added by the user
    created_at INTEGER NOT NULL, -- Timestamp of post creation
    updated_at INTEGER NOT NULL, -- Timestamp of last update
    read INTEGER DEFAULT 0 NOT NULL, -- Read/unread status of the update
    archived INTEGER DEFAULT 0 NOT NULL, -- archived stuff dont show up in the feed
    FOREIGN KEY (user_id) REFERENCES fastn_user(id)
) STRICT;
```

## cdp_update

```sql
CREATE TABLE cdp_update (
    id INTEGER PRIMARY KEY,
    content_type TEXT NOT NULL, -- Type of content (text, quote, link, review, testimonial, greeting, photo, video, workout, etc.)
    content TEXT, -- JSON field to store the content of the update
    links TEXT, -- JSON field to store links for media posts
    tags TEXT, -- JSON: list of tags
    created_at INTEGER NOT NULL, -- Timestamp of post creation
    updated_at INTEGER NOT NULL, -- Timestamp of last update
    reply_to INTEGER NULL, -- ID of the post this post is replying to (foreign key to cdp_feed)
    user_id INTEGER NOT NULL, -- Foreign key to the user who created the update
    FOREIGN KEY (reply_to) REFERENCES cdp_feed(id),
    FOREIGN KEY (user_id) REFERENCES fastn_user(id)
);
```
