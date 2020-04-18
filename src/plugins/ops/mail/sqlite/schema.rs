table! {
    ops_mail_aliases (id) {
        id -> Integer,
        domain_id -> Integer,
        source -> Text,
        destination -> Text,
        created_at -> Timestamp,
    }
}

table! {
    ops_mail_domains (id) {
        id -> Integer,
        name -> Text,
        locked_at -> Nullable<Timestamp>,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    ops_mail_users (id) {
        id -> Integer,
        domain_id -> Integer,
        email -> Text,
        password -> Text,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(ops_mail_aliases, ops_mail_domains, ops_mail_users,);
