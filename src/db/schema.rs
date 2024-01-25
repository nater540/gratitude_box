// @generated automatically by Diesel CLI.

diesel::table! {
    teams (id) {
        id -> Uuid,
        #[max_length = 20]
        slack_id -> Varchar,
        api_key -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 20]
        slack_id -> Varchar,
        #[max_length = 60]
        slack_team_id -> Varchar,
        updated_at -> Timestamp,
        points -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
