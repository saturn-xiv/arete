table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Binary,
        salt -> Nullable<Binary>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
