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
