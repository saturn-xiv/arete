table! {
    vpn_logs (id) {
        id -> Bigint,
        user_id -> Bigint,
        trusted_ip -> Varchar,
        trusted_port -> Integer,
        remote_ip -> Varchar,
        remote_port -> Integer,
        received -> Nullable<Bigint>,
        send -> Nullable<Bigint>,
        opened_at -> Datetime,
        closed_at -> Nullable<Datetime>,
    }
}

table! {
    vpn_users (id) {
        id -> Bigint,
        name -> Varchar,
        email -> Varchar,
        password -> Blob,
        online -> Bool,
        fixed_ip -> Nullable<Varchar>,
        locked_at -> Nullable<Datetime>,
        startup -> Date,
        shutdown -> Date,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    vpn_logs,
    vpn_users,
);
