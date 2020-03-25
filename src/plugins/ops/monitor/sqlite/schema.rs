table! {
    monitor_logs (id) {
        id -> Integer,
        name -> Text,
        code -> Text,
        value -> Binary,
        created_at -> Timestamp,
    }
}
