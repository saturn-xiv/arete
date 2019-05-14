table! {
    vpn_logs (id) {
        id -> Int8,
        user_id -> Int8,
        trusted_ip -> Varchar,
        trusted_port -> Int4,
        remote_ip -> Varchar,
        remote_port -> Int4,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        received -> Numeric,
        send -> Numeric,
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
