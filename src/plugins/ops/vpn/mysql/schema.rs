table! {
    vpn_logs (id) {
        id -> Bigint,
        user_id -> Bigint,
        #[sql_name = "type"]
        type_ -> Varchar,
        trusted_ip -> Varchar,
        trusted_port -> Integer,
        remote_ip -> Varchar,
        remote_port -> Integer,
        received -> Double,
        send -> Double,
        created_at -> Datetime,
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
