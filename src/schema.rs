// @generated automatically by Diesel CLI.

diesel::table! {
    collection_headers (id) {
        id -> Nullable<Integer>,
        collection_id -> Integer,
        key -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    collections (id) {
        id -> Nullable<Integer>,
        name -> Text,
        host -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    load_test (id) {
        id -> Nullable<Integer>,
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
        id -> Nullable<Integer>,
        request_id -> Nullable<Integer>,
        key -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    requests (id) {
        id -> Nullable<Integer>,
        name -> Text,
        collection_id -> Nullable<Integer>,
        path -> Text,
        method -> Text,
        body_type -> Text,
        body_content -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    collection_headers,
    collections,
    load_test,
    request_headers,
    requests,
);
