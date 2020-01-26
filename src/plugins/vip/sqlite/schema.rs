table! {
    vip_members (id) {
        id -> Integer,
        nick_name -> Text,
        real_name -> Text,
        gender -> Text,
        birthday -> Date,
        contact -> Text,
        point -> BigInt,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
