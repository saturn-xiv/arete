table! {
    vip_members (id) {
        id -> Bigint,
        nick_name -> Varchar,
        real_name -> Varchar,
        gender -> Varchar,
        birthday -> Date,
        contact -> Text,
        point -> Bigint,
        version -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
