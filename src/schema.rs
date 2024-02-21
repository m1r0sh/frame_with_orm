// @generated automatically by Diesel CLI.

diesel::table! {
    rooms (id) {
        id -> Nullable<Integer>,
        name -> Text,
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(rooms -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    rooms,
    users,
);
