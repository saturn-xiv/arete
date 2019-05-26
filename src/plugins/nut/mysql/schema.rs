table! {
    attachments (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        size -> Bigint,
        mime_type -> Varchar,
        url -> Varchar,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    cards (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Smallint,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    categories (id) {
        id -> Bigint,
        parent_id -> Nullable<Bigint>,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        position -> Smallint,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    category_resources (id) {
        id -> Bigint,
        category_id -> Bigint,
        resource_type -> Varchar,
        resource_id -> Bigint,
        created_at -> Datetime,
    }
}

table! {
    friend_links (id) {
        id -> Bigint,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        position -> Smallint,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    leave_words (id) {
        id -> Bigint,
        ip -> Varchar,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    links (id) {
        id -> Bigint,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Smallint,
        y -> Smallint,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    logs (id) {
        id -> Bigint,
        user_id -> Bigint,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    notifications (id) {
        id -> Bigint,
        user_id -> Integer,
        url -> Varchar,
        body -> Text,
        media_type -> Varchar,
        level -> Varchar,
        read -> Bool,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    policies (id) {
        id -> Bigint,
        user_id -> Bigint,
        role -> Varchar,
        resource -> Nullable<Varchar>,
        nbf -> Date,
        exp -> Date,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    tag_resources (id) {
        id -> Bigint,
        tag_id -> Bigint,
        resource_type -> Varchar,
        resource_id -> Bigint,
        created_at -> Datetime,
    }
}

table! {
    tags (id) {
        id -> Bigint,
        name -> Varchar,
        icon -> Varchar,
        color -> Varchar,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Bigint,
        real_name -> Varchar,
        nick_name -> Varchar,
        email -> Varchar,
        password -> Nullable<Blob>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Bigint,
        current_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Datetime>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Datetime>,
        locked_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    votes (id) {
        id -> Bigint,
        point -> Bigint,
        resource_type -> Varchar,
        resource_id -> Bigint,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
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
