table! {
    schema_migrations (id) {
        id -> Integer,
        version -> Text,
        name -> Text,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Timestamp>,
    }
}
