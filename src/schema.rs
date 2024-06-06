// @generated automatically by Diesel CLI.

diesel::table! {
    barrel (id) {
        id -> Int4,
        position_id -> Int4,
        driver_id -> Nullable<Int4>,
        going_to_position_id -> Nullable<Int4>,
        contains -> Varchar,
    }
}

diesel::table! {
    driver (id) {
        id -> Int4,
        first_name -> Varchar,
        second_name -> Varchar,
    }
}

diesel::table! {
    position (id) {
        id -> Int4,
        name -> Varchar,
        type_ -> Varchar,
    }
}

diesel::joinable!(barrel -> driver (driver_id));

diesel::allow_tables_to_appear_in_same_query!(
    barrel,
    driver,
    position,
);
