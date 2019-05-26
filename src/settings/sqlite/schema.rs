table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Binary,
        salt -> Nullable<Binary>,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
