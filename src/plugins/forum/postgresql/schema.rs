table! {
    forum_posts (id) {
        id -> Int8,
        user_id -> Int8,
        topic_id -> Int8,
        post_id -> Nullable<Int8>,
        body -> Text,
        media_type -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(forum_posts, forum_topics,);
