table! {
    locales (id) {
        id -> Int8,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
