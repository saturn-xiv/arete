table! {
    locales (id) {
        id -> Integer,
        lang -> Text,
        code -> Text,
        message -> Text,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
