table! {
    schema_migrations (id) {
        id -> Bigint,
        version -> Char,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Datetime>,
    }
}
