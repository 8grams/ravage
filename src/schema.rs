// @generated automatically by Diesel CLI.

diesel::table! {
    collection_headers (id) {
        id -> Integer,
        collection_id -> Integer,
        key -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    collections (id) {
        id -> Integer,
        name -> Text,
        host -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    load_tests (id) {
        id -> Integer,
        source_type -> Nullable<Text>,
        source_id -> Nullable<Integer>,
        name -> Text,
        log_path -> Nullable<Text>,
        report_path -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    request_headers (id) {
        id -> Integer,
        request_id -> Integer,
        key -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    requests (id) {
        id -> Integer,
        name -> Text,
        collection_id -> Integer,
        path -> Text,
        method -> Text,
        body_type -> Nullable<Text>,
        body_content -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(collection_headers -> collections (collection_id));
diesel::joinable!(request_headers -> requests (request_id));
diesel::joinable!(requests -> collections (collection_id));

diesel::allow_tables_to_appear_in_same_query!(
    collection_headers,
    collections,
    load_tests,
    request_headers,
    requests,
);
