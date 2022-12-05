// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        owner_id -> Integer,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        project_id -> Integer,
        assignee_id -> Nullable<Integer>,
        done -> Bool,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
        is_admin -> Bool,
    }
}

diesel::joinable!(projects -> users (owner_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks -> users (assignee_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    users,
);
