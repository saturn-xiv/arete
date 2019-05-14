table! {
    vpn_logs (id) {
        id -> Integer,
        user_id -> BigInt,
        trusted_ip -> Text,
        trusted_port -> Integer,
        remote_ip -> Text,
        remote_port -> Integer,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        received -> Double,
        send -> Double,
    }
}

table! {
    vpn_users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Binary,
        online -> Bool,
        locked_at -> Nullable<Timestamp>,
        startup -> Date,
        shutdown -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(vpn_logs, vpn_users,);
