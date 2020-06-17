table! {
    monitor_logs (id) {
        id -> Int8,
        name -> Varchar,
        uid -> Varchar,
        code -> Varchar,
        value -> Text,
        created_at -> Timestamp,
    }
}
