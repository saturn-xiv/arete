table! {
    monitor_logs (id) {
        id -> Bigint,
        name -> Varchar,
        code -> Varchar,
        value -> Blob,
        created_at -> Datetime,
    }
}
