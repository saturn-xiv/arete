table! {
    monitor_logs (id) {
        id -> Bigint,
        name -> Varchar,
        uid -> Varchar,
        code -> Varchar,
        value -> Text,
        created_at -> Datetime,
    }
}
