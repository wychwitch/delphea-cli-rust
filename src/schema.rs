// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        sheet_id -> Integer,
        name -> Text,
        color -> Text,
        won_against -> Binary,
        note -> Text,
        favorited -> Bool,
    }
}

diesel::table! {
    sheets (id) {
        id -> Integer,
        name -> Text,
        color -> Text,
        note -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    sheets,
);
