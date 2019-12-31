table! {
    forum_posts (id) {
        id -> Bigint,
        user_id -> Bigint,
        topic_id -> Bigint,
        post_id -> Nullable<Bigint>,
        body -> Text,
        media_type -> Varchar,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    forum_topics (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(forum_posts, forum_topics,);
