table! {
    monitor_logs (id) {
        id -> Bigint,
        name -> Varchar,
        code -> Varchar,
        value -> Text,
        created_at -> Datetime,
    }
}
