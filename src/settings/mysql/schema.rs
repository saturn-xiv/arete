table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        value -> Binary,
        salt -> Nullable<Binary>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
