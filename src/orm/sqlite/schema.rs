table! {
    schema_migrations (id) {
        id -> Int8,
        version -> Varchar,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Timestamp>,
    }
}
