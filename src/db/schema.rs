// @generated automatically by Diesel CLI.

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
