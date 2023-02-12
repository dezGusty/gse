// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Nullable<Integer>,
        name -> Text,
        display_name -> Nullable<Text>,
        rating -> Float,
    }
}
