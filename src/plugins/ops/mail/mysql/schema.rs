table! {
    ops_mail_aliases (id) {
        id -> Bigint,
        domain_id -> Bigint,
        email -> Varchar,
        source -> Varchar,
        destination -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    ops_mail_domains (id) {
        id -> Bigint,
        name -> Varchar,
        locked_at -> Nullable<Datetime>,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    ops_mail_users (id) {
        id -> Bigint,
        domain_id -> Bigint,
        email -> Varchar,
        password -> Varchar,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(ops_mail_aliases, ops_mail_domains, ops_mail_users,);
