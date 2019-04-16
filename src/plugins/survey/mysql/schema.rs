table! {
    survey_fields (id) {
        id -> Bigint,
        form_id -> Bigint,
        key -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        required -> Bool,
        #[sql_name = "type"]
        type_ -> Text,
        position -> Smallint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    survey_forms (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        description -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        nbf -> Date,
        exp -> Date,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    survey_logs (id) {
        id -> Bigint,
        form_id -> Bigint,
        user_id -> Nullable<Bigint>,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    survey_responses (id) {
        id -> Bigint,
        form_id -> Bigint,
        email -> Varchar,
        username -> Varchar,
        ip -> Varchar,
        content -> Text,
        created_at -> Datetime,
    }
}

table! {
    survey_subscribers (id) {
        id -> Bigint,
        form_id -> Bigint,
        email -> Varchar,
        username -> Varchar,
        created_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    survey_fields,
    survey_forms,
    survey_logs,
    survey_responses,
    survey_subscribers,
);
