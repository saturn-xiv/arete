table! {
    vpn_logs (id) {
        id -> Int8,
        user_id -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        trusted_ip -> Varchar,
        trusted_port -> Int4,
        remote_ip -> Varchar,
        remote_port -> Int4,
        received -> Float8,
        send -> Float8,
        created_at -> Timestamp,
    }
}

table! {
    vpn_users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Bytea,
        online -> Bool,
        locked_at -> Nullable<Timestamp>,
        startup -> Date,
        shutdown -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(vpn_logs, vpn_users,);
