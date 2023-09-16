// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Text,
        name -> Text,
        age -> Integer,
        favourite_colour -> Integer,
    }
}
