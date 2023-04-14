// @generated automatically by Diesel CLI.

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
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    user_roles,
    users,
);
