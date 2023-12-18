// @generated automatically by Diesel CLI.

diesel::table! {
    bookmarks (id) {
        id -> Integer,
        url -> Text,
        description -> Text,
    }
}

diesel::table! {
    bookmarks_tags (id) {
        id -> Integer,
        bookmark_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        tag -> Text,
    }
}

diesel::joinable!(bookmarks_tags -> bookmarks (bookmark_id));
diesel::joinable!(bookmarks_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookmarks,
    bookmarks_tags,
    tags,
);
