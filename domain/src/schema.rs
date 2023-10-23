// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (id) {
        id -> Int4,
        password -> Text,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        title -> Text,
    }
}

diesel::table! {
    role_permissions (id) {
        id -> Int4,
        role_id -> Nullable<Int4>,
        permission_id -> Nullable<Int4>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        title -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        lastname -> Text,
        email -> Text,
        created_at -> Timestamp,
        role_id -> Int4,
        credential_id -> Int4,
    }
}

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(users -> credentials (credential_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    permissions,
    role_permissions,
    roles,
    users,
);
