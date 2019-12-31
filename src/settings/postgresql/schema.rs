table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
