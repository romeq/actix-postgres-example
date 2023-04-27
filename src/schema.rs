// @generated automatically by Diesel CLI.

diesel::table! {
    files (file_id) {
        file_id -> Uuid,
        owner_id -> Uuid,
        original_filename -> Varchar,
        was_encrypted -> Bool,
        uploaded_at -> Timestamp,
        encryption_iv -> Nullable<Varchar>,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    user_roles (user_roles_id) {
        user_roles_id -> Uuid,
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(files -> users (owner_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    roles,
    user_roles,
    users,
);
