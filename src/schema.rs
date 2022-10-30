// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Int4,
        descrip -> Varchar,
        isapproved -> Bool,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    appointments,
    spatial_ref_sys,
);
