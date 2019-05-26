table! {
    settings (id) {
        id -> Bigint,
        key -> Varchar,
        value -> Blob,
        salt -> Nullable<Blob>,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
