// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "division"))]
    pub struct Division;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "league"))]
    pub struct League;
}

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Text,
        team -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::League;
    use super::sql_types::Division;

    teams (id) {
        id -> Int4,
        name -> Text,
        city -> Text,
        league -> League,
        division -> Division,
    }
}

diesel::joinable!(players -> teams (team));

diesel::allow_tables_to_appear_in_same_query!(
    players,
    teams,
);
