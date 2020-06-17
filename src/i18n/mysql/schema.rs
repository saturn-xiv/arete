table! {
    locales (id) {
        id -> Bigint,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
