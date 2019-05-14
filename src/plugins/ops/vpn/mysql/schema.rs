table! {
    vpn_logs (id) {
        id -> Bigint,
        user_id -> Bigint,
        trusted_ip -> Varchar,
        trusted_port -> Integer,
        remote_ip -> Varchar,
        remote_port -> Integer,
        start_time -> Datetime,
        end_time -> Nullable<Datetime>,
        received -> Decimal,
        send -> Decimal,
    }
}

table! {
    vpn_users (id) {
        id -> Bigint,
        name -> Varchar,
        email -> Varchar,
        password -> Blob,
        online -> Bool,
        locked_at -> Nullable<Datetime>,
        startup -> Date,
        shutdown -> Date,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(vpn_logs, vpn_users,);
