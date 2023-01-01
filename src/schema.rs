// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        sheet_id -> Integer,
        name -> Text,
        color -> Text,
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

diesel::table! {
    wins (id) {
        id -> Integer,
        winner_id -> Integer,
        loser_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    sheets,
    wins,
);
