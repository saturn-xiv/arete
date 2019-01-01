table! {
    locales (id) {
        id -> Int8,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
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
