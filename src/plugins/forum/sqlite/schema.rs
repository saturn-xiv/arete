table! {
    forum_posts (id) {
        id -> Integer,
        user_id -> Integer,
        topic_id -> Integer,
        post_id -> Nullable<Integer>,
        body -> Text,
        media_type -> Text,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Text,
        media_type -> Text,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    forum_posts,
    forum_topics,
);
