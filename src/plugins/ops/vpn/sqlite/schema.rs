table! {
    vpn_logs (id) {
        id -> Integer,
        user_id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        trusted_ip -> Text,
        trusted_port -> Integer,
        remote_ip -> Text,
        remote_port -> Integer,
        received -> Double,
        send -> Double,
        created_at -> Timestamp,
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
