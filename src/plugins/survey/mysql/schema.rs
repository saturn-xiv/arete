table! {
    survey_fields (id) {
        id -> Int8,
        form_id -> Int8,
        key -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        required -> Bool,
        #[sql_name = "type"]
        type_ -> Text,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_forms (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        description -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_logs (id) {
        id -> Int8,
        form_id -> Int8,
        user_id -> Nullable<Int8>,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    survey_responses (id) {
        id -> Int8,
        form_id -> Int8,
        email -> Varchar,
        username -> Varchar,
        ip -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    survey_subscribers (id) {
        id -> Int8,
        form_id -> Int8,
        email -> Varchar,
        username -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    survey_fields,
    survey_forms,
    survey_logs,
    survey_responses,
    survey_subscribers,
);
