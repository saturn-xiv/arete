table! {
    attachments (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        size -> BigInt,
        mime_type -> Text,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    cards (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        media_type -> Text,
        action -> Text,
        href -> Text,
        logo -> Text,
        loc -> Text,
        lang -> Text,
        position -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        name -> Text,
        icon -> Text,
        color -> Text,
        position -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    category_resources (id) {
        id -> Integer,
        category_id -> Integer,
        resource_type -> Text,
        resource_id -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    friend_links (id) {
        id -> Integer,
        title -> Text,
        home -> Text,
        logo -> Text,
        position -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> Integer,
        ip -> Text,
        body -> Text,
        media_type -> Text,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Integer,
        href -> Text,
        label -> Text,
        loc -> Text,
        lang -> Text,
        x -> SmallInt,
        y -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Integer,
        user_id -> Integer,
        ip -> Text,
        message -> Text,
        created_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Integer,
        user_id -> Integer,
        url -> Text,
        body -> Text,
        media_type -> Text,
        level -> Text,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    policies (id) {
        id -> Integer,
        user_id -> Integer,
        role -> Text,
        resource -> Nullable<Text>,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tag_resources (id) {
        id -> Integer,
        tag_id -> Integer,
        resource_type -> Text,
        resource_id -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        icon -> Text,
        color -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        real_name -> Text,
        nick_name -> Text,
        email -> Text,
        password -> Nullable<Binary>,
        uid -> Text,
        provider_type -> Text,
        provider_id -> Text,
        logo -> Text,
        sign_in_count -> BigInt,
        current_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Text>,
        last_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_ip -> Nullable<Text>,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Integer,
        point -> BigInt,
        resource_type -> Text,
        resource_id -> Integer,
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
