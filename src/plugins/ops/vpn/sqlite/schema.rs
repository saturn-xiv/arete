table! {
    vpn_logs (id) {
        id -> Integer,
        user_id -> Integer,
        trusted_ip -> Text,
        trusted_port -> Integer,
        remote_ip -> Text,
        remote_port -> Integer,
        received -> Nullable<BigInt>,
        send -> Nullable<BigInt>,
        opened_at -> Timestamp,
        closed_at -> Nullable<Timestamp>,
    }
}

table! {
    vpn_users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Binary,
        online -> Bool,
        fixed_ip -> Nullable<Text>,
        locked_at -> Nullable<Timestamp>,
        startup -> Date,
        shutdown -> Date,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(vpn_logs, vpn_users,);
