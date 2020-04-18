table! {
    ops_mail_aliases (id) {
        id -> Int8,
        domain_id -> Int8,
        source -> Varchar,
        destination -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    ops_mail_domains (id) {
        id -> Int8,
        name -> Varchar,
        locked_at -> Nullable<Timestamp>,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    ops_mail_users (id) {
        id -> Int8,
        domain_id -> Int8,
        email -> Varchar,
        password -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(ops_mail_aliases, ops_mail_domains, ops_mail_users,);
