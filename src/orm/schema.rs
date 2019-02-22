table! {
    schema_migrations (id) {
        id -> Int8,
        version -> Bpchar,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Timestamp>,
    }
}
