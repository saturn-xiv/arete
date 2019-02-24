table! {
    attachments (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        size -> Int8,
        mime_type -> Varchar,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    cards (id) {
        id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int8,
        parent_id -> Nullable<Int8>,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    category_resources (id) {
        id -> Int8,
        category_id -> Int8,
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    friend_links (id) {
        id -> Int8,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> Int8,
        ip -> Inet,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Int8,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Int2,
        y -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        ip -> Inet,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Int8,
        user_id -> Int4,
        url -> Varchar,
        body -> Text,
        media_type -> Varchar,
        level -> Varchar,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    policies (id) {
        id -> Int8,
        user_id -> Int8,
        role -> Varchar,
        resource -> Nullable<Varchar>,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tag_resources (id) {
        id -> Int8,
        tag_id -> Int8,
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int8,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        real_name -> Varchar,
        nick_name -> Varchar,
        email -> Varchar,
        password -> Nullable<Bytea>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Int8,
        current_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Inet>,
        last_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_ip -> Nullable<Inet>,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Int8,
        point -> Int8,
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    attachments,
    cards,
    categories,
    category_resources,
    friend_links,
    leave_words,
    links,
    logs,
    notifications,
    policies,
    tag_resources,
    tags,
    users,
    votes,
);