table! {
    survey_fields (id) {
        id -> Integer,
        form_id -> Integer,
        key -> Text,
        title -> Text,
        description -> Nullable<Text>,
        required -> Bool,
        #[sql_name = "type"]
        type_ -> Text,
        position -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_forms (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
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
        id -> Integer,
        form_id -> Integer,
        user_id -> Nullable<Integer>,
        ip -> Text,
        message -> Text,
        created_at -> Timestamp,
    }
}

table! {
    survey_responses (id) {
        id -> Integer,
        form_id -> Integer,
        email -> Text,
        username -> Text,
        ip -> Text,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    survey_subscribers (id) {
        id -> Integer,
        form_id -> Integer,
        email -> Text,
        username -> Text,
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
