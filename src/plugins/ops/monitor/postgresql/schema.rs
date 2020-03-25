table! {
    monitor_logs (id) {
        id -> Int8,
        name -> Varchar,
        code -> Varchar,
        value -> Bytea,
        created_at -> Timestamp,
    }
}
