// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Int4,
        descrip -> Varchar,
        isapproved -> Bool,
    }
}
