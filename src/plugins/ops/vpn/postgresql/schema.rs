table! {
    vpn_logs (id) {
        id -> Int8,
        user_id -> Int8,
        trusted_ip -> Varchar,
        trusted_port -> Int4,
        remote_ip -> Varchar,
        remote_port -> Int4,
        received -> Nullable<Int8>,
        send -> Nullable<Int8>,
        opened_at -> Timestamp,
        closed_at -> Nullable<Timestamp>,
    }
}

table! {
    vpn_users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Bytea,
        online -> Bool,
        fixed_ip -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        startup -> Date,
        shutdown -> Date,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(vpn_logs, vpn_users,);
