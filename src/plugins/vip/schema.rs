table! {
    vip_members (id) {
        id -> Int8,
        nick_name -> Varchar,
        real_name -> Varchar,
        gender -> Varchar,
        birthday -> Date,
        contact -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
